These files are necessary for:
1. running the swtpm installed on our testbed
2. provisioning of signing keys

To start the swtpm on the testbed, execute `source start_tpm.sh` (necessary to set environment variables).

When you start the swtpm for the first time, you should then execute `./gen_policies.sh` and `./gen_keys.sh`.
This will create the signing key and its policies. These should never need to be run again, unless you want new policies or keys.

Specifically, if you only want updated policies, you can rerun the policy script any time.
If you want a new signing key for the wildcard policies, you must delete the entire `policy` folder.

If you want new TPM keys, you must delete the `key.priv`, `key.pub`, and `key.pem` files.
You must also delete the `policy.ctx` and `signing.ctx` files in your temp folder (e.g. `/tmp/policy.ctx` and `/tmp/signing.ctx`).
Using these `.ctx` files improves TPM usage speed, they are in `/tmp` because they are invalidated after TPM shutdown.
If you haven't also restarted the swtpm, they must be removed otherwise the old keys will be used.
