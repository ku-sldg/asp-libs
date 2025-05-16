#!/bin/bash

ATTACKS_PATH=$ATTACK_BIN

MOD_SCRIPT=$ATTACKS_PATH/mod_userspace_file.sh

GOOD_PATH=$ATTACKS_PATH/targ_files/theorem_targs/ImportantTheorem.v
BAD_PATH=$ATTACKS_PATH/targ_files/theorem_targs/ImportantTheorem_axiom.v
TARG_PATH=$THEOREMS_ROOT/my_theorems/ImportantTheorem.v

$MOD_SCRIPT -g  $GOOD_PATH \
            -b  $BAD_PATH  \
            -t  $TARG_PATH \
            -a
