#!/bin/bash

sh check_env_vars.sh
if [ $? -ne 0 ]; then
     exit 1
fi

ATTACKS_PATH=$ATTACK_BIN

MOD_SCRIPT=$ATTACKS_PATH/mod_userspace_file.sh

GOOD_PATH=$ATTACKS_PATH/targ_files/theorem_targs/ImportantTheorem.v
BAD_PATH=$ATTACKS_PATH/targ_files/theorem_targs/ImportantTheorem_bad_proof.v
TARG_PATH=$THEOREMS_ROOT/my_theorems/ImportantTheorem.v

$MOD_SCRIPT -g  $GOOD_PATH \
            -b  $BAD_PATH  \
            -t  $TARG_PATH \
            -a
