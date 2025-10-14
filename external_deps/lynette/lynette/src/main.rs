use clap::{ArgGroup, Args, Parser as ClapParser, Subcommand};
use lynette::{
    code::{fdetect_nl, fget_target, get_calls_at, get_func_at},
    deghost::remove_ghost_from_file,
    func::{fextract_function, fremove_function, insert_functions},
    merge::fmerge_files,
    unimpl::funimpl_file,
    utils::{
        fextract_verus_macro, fload_file, fprint_file, get_block_rect, print_block,
        update_verus_macros_files, FnMethod, FnMethodExt, Formatter,
    },
    *,
};
use proc_macro2::TokenStream;
use quote::ToTokens;
use serde_json::json;
use std::ops::RangeInclusive;
use std::path::PathBuf;
use std::process;
use syn_verus::{FnArg, FnArgKind, Type};

fn parse_ranges(
    s: &str,
) -> Result<std::vec::Vec<RangeInclusive<usize>>, Box<dyn std::error::Error + Send + Sync>> {
    s.split(',')
        .map(|part| {
            if part.contains('-') {
                let mut range_parts = part.split('-').map(|p| p.parse::<usize>());
                let start = range_parts
                    .next()
                    .ok_or("No start for range")?
                    .map_err(|_| "Invalid start")?;
                let end = range_parts
                    .next()
                    .ok_or("No end for range")?
                    .map_err(|_| "Invalid end")?;
                Ok(RangeInclusive::new(start, end))
            } else {
                let num = part.parse::<usize>()?;
                Ok(RangeInclusive::new(num, num))
            }
        })
        .collect()
}

#[derive(Args)]
struct GetCallsArgs {
    file: PathBuf,
    // We have to use std::vec::Vec<...> here due to how clap treats certain types
    // of arguments.
    #[clap(short, long, value_parser = parse_ranges,
        help = "Only returns function calls for the specified line(s).",
        long_help = "Only returns function calls for the specified line(s).
The format is a comma separated list of ranges, e.g. 1-3,5,7-9.")]
    line: Option<std::vec::Vec<RangeInclusive<usize>>>,
}

#[derive(Args)]
#[clap(group(ArgGroup::new("location").args(&["line", "offset", "name"]).required(true).multiple(false)))]
struct GetFuncAtArgs {
    file: PathBuf,
    #[clap(short, help = "The line number of the function.")]
    line: Option<usize>,
    #[clap(short, help = "The offset from the beginning of the file.")]
    offset: Option<usize>,
}

#[derive(Args)]
struct ParseArgs {
    file: PathBuf,
    #[clap(
        short,
        long,
        help = "Only check syntax, do not print anything",
        default_value = "false"
    )]
    check: bool,
}

#[derive(Args)]
struct DetectNLArgs {
    file: PathBuf,
    #[clap(
        short,
        long,
        help = "If set, also detect non-linear operations in function qualifiers.",
        default_value = "false"
    )]
    sig: bool,
}

#[derive(Args)]
struct FunctionArgs {
    file: PathBuf,
    function: String,
}

#[derive(Args)]
struct ExtractFunctionArgs {
    file: PathBuf,
    #[clap(
        short,
        long,
        help = "A list of comma-separated function names.",
        value_delimiter = ','
    )]
    function: Vec<String>,
    #[clap(
        short,
        help = "Only return the function body.",
        default_value = "false"
    )]
    body: bool,
}

#[derive(ClapParser)]
#[command(about)]
struct FunctionArgs2 {
    /// Original file
    file: PathBuf,
    /// File containing the functions to add
    file2: PathBuf,
    /// Replace the functions in <FILE> by the functions in <FILE2> if conflicts occur
    #[clap(short, long, default_value = "false")]
    replace: bool,
    #[clap(
        short,
        long,
        help = "A list of comma-separated function names to add in <FILE2>",
        value_delimiter = ',',
        default_value = ""
    )]
    funcs: Vec<String>,
}

#[derive(Args)]
struct PruneQualiArgs {
    /// Verus source code file
    file: PathBuf,
    /// Function name
    fname: String,
    #[clap(long, help = "Prune pre-conditions.", default_value = "false")]
    pre: bool,
    #[clap(long, help = "Prune post-conditions.", default_value = "false")]
    post: bool,
    #[clap(
        long,
        short,
        help = "Prune pre- and post-conditions. Same as --pre --post",
        default_value = "true"
    )]
    all: bool,
}

#[derive(Args)]
struct MergeArgs {
    file1: PathBuf,
    file2: PathBuf,
    #[clap(flatten)]
    opts: DeghostMode,

    #[clap(long, help = "Merge everyting.", default_value = "false")]
    all: bool,
}

#[derive(Args)]
struct DeghostArgs {
    /// Input Verus source code file
    file: PathBuf,
    #[clap(
        short,
        long,
        help = "Deghost mode: 'raw' for raw code, 'unverified' for unverified code",
        default_value = "raw"
    )]
    mode: String, // "raw", "unverified"
    #[clap(
        short,
        long,
        help = "Output file path (prints to stdout if not specified)"
    )]
    output: Option<PathBuf>,
}

#[derive(Args)]
struct UnimplArgs {
    file1: PathBuf,
    #[clap(
        short,
        long,
        help = "If set, also unimplement functions tagged with #[warn(llm4verus_target)]",
        default_value = "false"
    )]
    target: bool,
}

#[derive(Subcommand)]
enum FunctionCommands {
    #[clap[about =
            r#"Add the functions in <FILE2> to <FILE>.

Both files should contain exact one verus!{...} macro.

Use --replace will replace the function if conflicts occur;
Otherwise, an error will be thrown on conflicts.

The result will be printed to stdout."#
    ]]
    Add(FunctionArgs2),

    #[clap[about = "Extract a function in a verus source code file."]]
    Extract(ExtractFunctionArgs),

    #[clap[about = "Returns the arguments of a function in a verus source code file."]]
    GetArgs(FunctionArgs),

    Remove(FunctionArgs),

    #[clap[about = "Detect whether a function contains non-linear operations in verus assert."]]
    DetectNL(FunctionArgs),

    PruneQuali(PruneQualiArgs),
}

#[derive(Subcommand)]
enum CodeCommands {
    #[clap(about = "Get the function calls in a verus source code file.")]
    GetCalls(GetCallsArgs),
    #[clap(about = "Get the function at a specific line or offset in a verus source code file")]
    GetFunc(GetFuncAtArgs),
    #[clap(
        about = "Detect all non-linear operations in a verus source code file. Returns a list of (start, end) positions of the NL operations."
    )]
    DetectNL(ParseArgs),
    #[clap(about = "WIP: Get the target of a verus source code file.")]
    GetTarget(ParseArgs),
    #[clap(about = r#"Merge the proof code of two verus source code files.

The two files should have the exact same exec-code.
Currently only merging invariants is allowed. (use only `--invariants` flag)
Using other flags may lead to undefined behavior.

If there are conflicts in the non-merging part, <FILE1> is preferred.
"#)]
    Merge(MergeArgs),
    Unimpl(UnimplArgs),
    #[clap(about = "Remove all proof annotations from Verus code to produce pure Rust code")]
    Deghost(DeghostArgs),
}

#[derive(Subcommand)]
enum Commands {
    #[clap(
        about = r#"Compare whether two verus source code files generates the same rust code.

Various flags can be set to have fine-grained control over what ghost code should also be compared."#
    )]
    Compare(CompareArgs),
    Parse(ParseArgs),
    #[clap(
        subcommand,
        about = "Operations on functions, use func --help for more information"
    )]
    Func(FunctionCommands),
    #[clap(subcommand)]
    Code(CodeCommands),
}

#[derive(ClapParser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// Borrowed and modified from syn/src/item.rs
fn maybe_variadic_to_tokens(
    arg: &FnArg,
    arg_tokens: &mut TokenStream,
    ty_tokens: &mut TokenStream,
) -> bool {
    arg.tracked.to_tokens(arg_tokens);

    let arg = match &arg.kind {
        FnArgKind::Typed(arg) => arg,
        FnArgKind::Receiver(receiver) => {
            receiver.to_tokens(arg_tokens);
            return false;
        }
    };

    match arg.ty.as_ref() {
        Type::Verbatim(ty) if ty.to_string() == "..." => true,
        _ => {
            arg.pat.to_tokens(arg_tokens);
            arg.ty.to_tokens(ty_tokens);
            false
        }
    }
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Compare(args) => {
            let res = compare_files(&args).unwrap_or_else(|e| {
                eprintln!("{}", e);
                process::exit(1);
            });

            if !res {
                println!("Files are different");
                process::exit(1);
            }
        }
        Commands::Parse(args) => {
            let filepath = args.file;
            let check = args.check;

            let file = fload_file(&filepath).unwrap_or_else(|e| {
                eprintln!("{}", e);
                process::exit(1);
            });

            if !check {
                let pretty_file = format!("{:#?}", file).replace("    ", "  ");
                println!("{}", pretty_file);
            }
            fextract_verus_macro(&filepath)
                .map(|(files, _)| {
                    if !check {
                        for file in files {
                            let pretty_file = format!("{:#?}", file).replace("    ", "  ");
                            println!("{}", pretty_file);
                        }
                    }
                })
                .unwrap_or_else(|e| {
                    eprintln!("{}", e);
                    process::exit(1);
                });

            //println!("{}", fprint_file(&file, Formatter::Mix));
        }
        Commands::Func(fcmd) => {
            match fcmd {
                FunctionCommands::GetArgs(args) => {
                    let filepath = args.file;
                    let function = args.function;

                    let funcs = fextract_function(&filepath, &vec![function]).unwrap_or_else(|e| {
                        eprintln!("{}", e);
                        process::exit(1);
                    });

                    let func = &funcs[0];
                    let ret: serde_json::Value = func
                        .get_sig()
                        .inputs
                        .iter()
                        .map(|arg| {
                            let mut arg_token = TokenStream::new();
                            let mut ty_token = TokenStream::new();
                            if maybe_variadic_to_tokens(&arg, &mut arg_token, &mut ty_token) {
                                eprintln!("Varaidic arguments are not supported");
                                process::exit(1);
                            }

                            json!({
                                "arg": arg_token.to_string(),
                                "ty": ty_token.to_string(),
                            })
                        })
                        .collect();

                    println!("{}", ret);
                }
                FunctionCommands::Extract(args) => {
                    let filepath = args.file;
                    let funcs = args.function;
                    let body = args.body;

                    fextract_function(&filepath, &funcs)
                        .and_then(|funcs| {
                            let func = &funcs[0];

                            if !body {
                                print_block(&filepath, func.get_span_rect()).unwrap_or(());
                            } else {
                                print_block(&filepath, get_block_rect(func.get_block()))
                                    .unwrap_or(());
                            }
                            Ok(())
                        })
                        .unwrap_or_else(|e| {
                            eprintln!("{}", e);
                            process::exit(1);
                        });
                }
                FunctionCommands::Remove(args) => {
                    let filepath = args.file;
                    let function = args.function;

                    fremove_function(&filepath, function)
                        .and_then(|new_file| {
                            println!("{}", fprint_file(&new_file, Formatter::Mix));
                            Ok(())
                        })
                        .unwrap_or_else(|e| {
                            eprintln!("{}", e);
                            process::exit(1);
                        })
                }
                FunctionCommands::Add(args) => {
                    let filepath1 = args.file;
                    let filepath2 = args.file2;
                    let replace = args.replace;
                    let funcs = args.funcs;

                    // let new_funs = ffind_all_functions(&filepath2).unwrap_or_else(|e| {
                    //     eprintln!("{}", e);
                    //     process::exit(1);
                    // });

                    let new_funcs = fextract_function(&filepath2, &funcs).unwrap_or_else(|e| {
                        eprintln!("{}", e);
                        process::exit(1);
                    });

                    fextract_verus_macro(&filepath1)
                        .map(|(mut files, orig)| {
                            // We shouldn't be doing this in a loop since it'll insert the same functions multiple times
                            // Assert there is only one `verus!` macro.
                            assert!(files.len() == 1);
                            for file in &mut files {
                                insert_functions(file, new_funcs.clone(), replace).unwrap_or_else(
                                    |e| {
                                        eprintln!("{}", e);
                                        process::exit(1);
                                    },
                                );
                            }

                            let new_file = update_verus_macros_files(&orig, files);

                            println!("{}", fprint_file(&new_file, Formatter::Mix));
                        })
                        .unwrap_or_else(|e| {
                            eprintln!("{}", e);
                            process::exit(1);
                        });
                }
                FunctionCommands::DetectNL(args) => {
                    let filepath = args.file;
                    let function = args.function;

                    fextract_function(&filepath, &vec![function])
                        .and_then(|_func| {
                            unimplemented!();
                            // let nl = detect_non_linear_func(&func);
                            // println!("{}", nl);
                            // Ok(())
                        })
                        .unwrap_or_else(|e| {
                            eprintln!("{}", e);
                            process::exit(1);
                        })
                }
                FunctionCommands::PruneQuali(args) => {
                    let filepath = args.file;
                    let fname = args.fname;
                    let pre = args.pre || args.all;
                    let post = args.post || args.all;

                    fextract_function(&filepath, &vec![fname])
                        .and_then(|funcs| {
                            assert!(funcs.len() == 1);
                            let func = &funcs[0];

                            match func {
                                FnMethod::Fn(f) => {
                                    let sig = &f.sig;
                                    let new_sig = syn_verus::Signature {
                                        publish: syn_verus::Publish::Default,
                                        constness: sig.constness.clone(),
                                        asyncness: sig.asyncness.clone(),
                                        unsafety: sig.unsafety.clone(),
                                        abi: sig.abi.clone(),
                                        broadcast: sig.broadcast.clone(),
                                        mode: sig.mode.clone(),
                                        fn_token: sig.fn_token.clone(),
                                        ident: sig.ident.clone(),
                                        generics: sig.generics.clone(),
                                        paren_token: sig.paren_token.clone(),
                                        inputs: sig.inputs.clone(),
                                        variadic: sig.variadic.clone(),
                                        output: sig.output.clone(),
                                        prover: sig.prover.clone(),
                                        requires: if !pre { sig.requires.clone() } else { None }, // Removed
                                        recommends: sig.recommends.clone(),
                                        ensures: if !post { sig.ensures.clone() } else { None }, // Removed
                                        decreases: sig.decreases.clone(),
                                        invariants: sig.invariants.clone(),
                                    };

                                    let new_fn = syn_verus::ItemFn {
                                        attrs: f.attrs.clone(),
                                        vis: f.vis.clone(),
                                        sig: new_sig,
                                        block: f.block.clone(),
                                        semi_token: f.semi_token.clone(),
                                    };

                                    fextract_verus_macro(&filepath).and_then(|(mut files, orig)| {
                                        assert!(files.len() == 1);
                                        for file in &mut files {
                                            insert_functions(
                                                file,
                                                vec![FnMethod::Fn(new_fn.clone())],
                                                true,
                                            )
                                            .unwrap_or_else(|e| {
                                                eprintln!("{}", e);
                                                process::exit(1);
                                            });
                                        }

                                        let new_file = update_verus_macros_files(&orig, files);

                                        println!("{}", fprint_file(&new_file, Formatter::Mix));
                                        Ok(())
                                    })
                                }
                                FnMethod::Method(_, _m) => {
                                    unimplemented!("Method is not supported yet");
                                }
                                _ => {
                                    unimplemented!();
                                }
                            }
                        })
                        .unwrap_or_else(|e| {
                            eprintln!("{}", e);
                            process::exit(1);
                        })
                }
            }
        }
        Commands::Code(ccmd) => {
            match ccmd {
                CodeCommands::GetCalls(arg) => {
                    let filepath = arg.file;
                    let ranges = arg.line.clone();

                    let objs = get_calls_at(&filepath, ranges).unwrap_or_else(|e| {
                        eprintln!("{}", e);
                        process::exit(1);
                    });

                    println!("{}", json!(objs));
                }
                CodeCommands::GetFunc(arg) => {
                    let filepath = arg.file;
                    let line = arg.line;
                    let offset = arg.offset;

                    let result = get_func_at(&filepath, line, offset).unwrap_or_else(|e| {
                        eprintln!("{}", e);
                        process::exit(1);
                    });
                    println!("[{}]", result.join(","));
                }
                CodeCommands::DetectNL(arg) => {
                    let filepath = arg.file;

                    let result = fdetect_nl(&filepath).unwrap_or_else(|e| {
                        eprintln!("{}", e);
                        process::exit(1);
                    });

                    println!("{:?}", result);
                }
                CodeCommands::GetTarget(arg) => {
                    let filepath = arg.file;

                    let result = fget_target(&filepath).unwrap_or_else(|e| {
                        eprintln!("{}", e);
                        process::exit(1);
                    });

                    println!(
                        "[{}]",
                        result
                            .iter()
                            .map(|f| {
                                match f {
                                    FnMethod::Fn(f) => f.sig.ident.to_string(),
                                    FnMethod::Method(_, m) => m.sig.ident.to_string(),
                                    _ => unimplemented!(),
                                }
                            })
                            .collect::<Vec<String>>()
                            .join(",")
                    );
                }
                CodeCommands::Merge(arg) => {
                    let filepath1 = &arg.file1;
                    let filepath2 = &arg.file2;
                    let mode = if arg.all {
                        &DeghostMode {
                            requires: true,
                            ensures: true,
                            invariants: true,
                            spec: true,
                            asserts: true,
                            asserts_anno: true,
                            decreases: true,
                            assumes: true,
                            sig_output: true,
                        }
                    } else {
                        &arg.opts
                    };

                    // DEGHOST_MODE_OPT.with(|mode| {
                    //     mode.borrow_mut().replace_with(&arg.opts);
                    // });

                    fmerge_files(filepath1, filepath2, mode)
                        .and_then(|f| {
                            println!("{}", fprint_file(&f, Formatter::Mix));
                            Ok(())
                        })
                        .unwrap_or_else(|e| {
                            eprintln!("{}", e);
                            process::exit(1);
                        });
                }
                CodeCommands::Unimpl(arg) => {
                    let filepath = arg.file1;
                    let target = arg.target;

                    funimpl_file(&filepath, target)
                    .and_then(|f| {
                        let ret: serde_json::Value = f
                            .iter()
                            .map(|(n, f)| json!({"name":n, "code": fprint_file(&f, Formatter::Mix)}))
                            .collect();

                        println!("{}", ret);
                        Ok(())
                    })
                    .unwrap_or_else(|e| {
                        eprintln!("{}", e);
                        process::exit(1);
                    });
                }
                CodeCommands::Deghost(args) => {
                    let filepath = args.file;
                    let output_path = args.output;
                    let mode_str = args.mode.as_str();

                    // Configure DeghostMode based on the mode flag
                    let deghost_mode = match mode_str {
                        "unverified" => DeghostMode {
                            requires: true, // Keep all preconditions
                            ensures: true,  // Keep all postconditions
                            invariants: false,
                            spec: true, // Keep all spec functions
                            asserts: false,
                            asserts_anno: false,
                            decreases: false,
                            assumes: true,    // Keep all assumes
                            sig_output: true, // Keep signature output
                        },
                        "raw" => DeghostMode {
                            requires: false,
                            ensures: false,
                            invariants: false,
                            spec: false,
                            asserts: false,
                            asserts_anno: false,
                            decreases: false,
                            assumes: false,
                            sig_output: false,
                        },
                        _ => {
                            eprintln!("Invalid mode: {}. Use 'raw' or 'unverified'", mode_str);
                            process::exit(1);
                        }
                    };

                    // Read file as string
                    let code = match std::fs::read_to_string(&filepath) {
                        Ok(content) => content,
                        Err(e) => {
                            eprintln!("Error reading file: {}", e);
                            process::exit(1);
                        }
                    };

                    // Extract verus! macro content manually using string processing
                    let mut verus_start = code.find("verus! {");
                    if verus_start.is_none() {
                        // If not found, try to find "verus!{" without space
                        verus_start = code.find("verus!{");
                    }
                    // let verus_end = code.rfind("} // verus!");
                    // assuming the verus! macro is always at the end of the file
                    let verus_end = code.rfind("}"); // assuming the last closing brace is the end of the verus! macro

                    if let (Some(start), Some(end)) = (verus_start, verus_end) {
                        // Extract the content inside verus! { ... }
                        let verus_content = &code[start + 8..end]; // Skip "verus! {"
                        let non_verus_content = &code[..start];

                        // Parse only the verus content
                        match syn_verus::parse_str::<syn_verus::File>(verus_content) {
                            Ok(verus_file) => {
                                let deghosted = remove_ghost_from_file(&verus_file, &deghost_mode);
                                let deghosted_output =
                                    fprint_file(&deghosted, Formatter::VerusFmtNoMacro);

                                // Combine non-verus content with deghosted verus content
                                // Format output based on mode
                                let final_output = match mode_str {
                                    "raw" => {
                                        // comment out the `use vstd::prelude::*;` import
                                        let non_verus_content_no_import = non_verus_content
                                            .replace(
                                                "use vstd::prelude::*;",
                                                "// use vstd::prelude::*;",
                                            );

                                        // Just output the deghosted content without verus wrapper
                                        format!(
                                            "{}\n{}",
                                            non_verus_content_no_import.trim(),
                                            deghosted_output
                                        )
                                    }
                                    "unverified" => {
                                        // Keep the verus macro wrapper and imports
                                        format!(
                                            "{}verus! {{{}}}",
                                            non_verus_content, deghosted_output
                                        )
                                    }
                                    _ => unreachable!(),
                                };

                                if let Some(out_path) = output_path {
                                    match std::fs::write(&out_path, &final_output) {
                                        Ok(_) => println!(
                                            "Deghosted code written to: {}",
                                            out_path.display()
                                        ),
                                        Err(e) => {
                                            eprintln!("Error writing to file: {}", e);
                                            process::exit(1);
                                        }
                                    }
                                } else {
                                    println!("{}", final_output);
                                }
                            }
                            Err(e) => {
                                eprintln!("Error parsing verus content: {}", e);
                                process::exit(1);
                            }
                        }
                    } else {
                        eprintln!("No verus! macro found in file");
                        process::exit(1);
                    }
                }
            }
        }
    };
}
