use clap::Args;
use std::cell::RefCell;
use std::path::PathBuf;

pub mod code;
pub mod deghost;
pub mod extract;
pub mod func;
pub mod merge;
pub mod unimpl;
pub mod utils;

use crate::deghost::*;
use crate::utils::*;
pub use extract::{extract_implementation, extract_spec_signatures, 
    strip_proof_functions, strip_verifier_attributes};

/// When a flag is set, the corresponding ghost code will not be removed by the
/// deghost functions.
#[derive(Clone, Args, Debug)]
pub struct DeghostMode {
    #[clap(long, help = "Compare requires")]
    pub requires: bool,
    #[clap(long, help = "Compare ensures")]
    pub ensures: bool,
    #[clap(long, help = "Compare invariants")]
    pub invariants: bool,
    #[clap(long, help = "Compare spec functions")]
    pub spec: bool,
    #[clap(long, help = "Compare asserts")]
    pub asserts: bool,
    #[clap(
        long,
        help = "Compare asserts with annotations(e.g. #[warn(llm_do_not_change)])"
    )]
    pub asserts_anno: bool,
    #[clap(long, help = "Compare decreases")]
    pub decreases: bool,
    #[clap(long, help = "Compare assumes")]
    pub assumes: bool,
    #[clap(long, help = "Compare signature output")]
    pub sig_output: bool,
    #[clap(long, help = "Strip implementation related code")]
    pub strip_bodies: bool,
}

impl DeghostMode {
    pub fn replace_with(&mut self, other: &DeghostMode) {
        self.requires = other.requires;
        self.ensures = other.ensures;
        self.invariants = other.invariants;
        self.spec = other.spec;
        self.asserts = other.asserts;
        self.asserts_anno = other.asserts_anno;
        self.decreases = other.decreases;
        self.assumes = other.assumes;
        self.strip_bodies = other.strip_bodies;
    }
}

thread_local! {
    static DEGHOST_MODE_OPT: RefCell<DeghostMode> = RefCell::new(DeghostMode::default());
}

/// By default, all flags are set to false so that all ghost code will be removed.
impl Default for DeghostMode {
    fn default() -> Self {
        Self {
            requires: false,
            ensures: false,
            invariants: false,
            spec: false,
            asserts: false,
            asserts_anno: false,
            decreases: false,
            assumes: false,
            sig_output: false,
            strip_bodies: false,
        }
    }
}

#[derive(Args, Debug, Clone, Default)]
pub struct CompareArgs {
    pub file1: PathBuf,
    pub file2: PathBuf,

    #[clap(
        short,
        long,
        action,
        long_help = "(Deprecated)Target mode. If set, the comparison will be more strict on the qualifier and spec function.
 This flag may be extended futher in the future."
    )]
    pub target: bool,

    #[clap(flatten)]
    pub opts: DeghostMode,

    #[clap(
        short,
        long,
        help = "If set, the two compared files after deghosting will be printed out.",
        default_value = "false"
    )]
    pub verbose: bool,
}

// Compare two verus source code files. Return true if the rust part of the files are the same.
pub fn compare_files(args: &CompareArgs) -> Result<bool, Error> {
    let (f1, f2) = (args.file1.clone(), args.file2.clone());

    let target_mode = args.target;

    let mode = if target_mode {
        let mut m = args.opts.clone();

        m.requires = true;
        m.ensures = true;
        m.assumes = true;
        m.decreases = true;

        m
    } else {
        args.opts.clone()
    };

    fextract_pure_rust(f1, &mode).and_then(|result1| {
        fextract_pure_rust(f2, &mode).and_then(|result2| {
            if args.verbose {
                println!("{}", fprint_file(&result1, Formatter::VerusFmt));
                println!("{}", fprint_file(&result2, Formatter::VerusFmt));
            }
            Ok(result1 == result2)
        })
    })
}
