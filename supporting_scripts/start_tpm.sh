#!/bin/sh
mkdir -p mytpm
echo "starting swtpm"
swtpm socket --tpmstate dir=mytpm --tpm2 --ctrl type=tcp,port=2322 --server type=tcp,port=2321 --flags not-need-init &
sleep 0.5
TPM2TOOLS_TCTI="swtpm:port=2321" tpm2_startup -c
echo "done"
echo "starting resource manager"
tpm2-abrmd -t swtpm:port=2321 &
sleep 0.5
echo "done"
export TPM2TOOLS_TCTI=tabrmd:bus_name=com.intel.tss2.Tabrmd

