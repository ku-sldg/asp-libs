These files are necessary for:
1. running the swtpm installed on our testbed
2. provisioning of signing keys

To start the swtpm on the testbed, execute `source asp-libs/supporting_scripts/start_tpm.sh` (necessary to set environment variables) from the home directory of the demo user.

When you start the swtpm for the first time, you should then execute `./gen_policies.sh` and `./gen_keys.sh`.
This will create the signing key and its policies. These should never need to be run again, unless you want new policies or keys.
They will put the policies and keys either in the current directory or in the directory from the environment variable `AM_TPM_DIR` if set.

Specifically, if you only want updated policies, you can rerun the policy script any time.
If you want a new signing key for the wildcard policies, you must delete the entire `policy` folder.

If you want new TPM keys, you must delete the `key.priv`, `key.pub`, and `key.pem` files.
We store two temporary files (`/tmp/policy.ctx` and `/tmp/signing.ctx`) to improve TPM usage speed. They are invalidated after TPM shutdown.
Using the scripts to restart the swtpm or generate new keys will delete these files.
If they get out of sync with the actual keys (which should be hard to do), it will cause confusing errors that signatures don't match.
