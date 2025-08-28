#!/bin/bash

ATTACKS_PATH=$ATTACK_BIN

MOD_SCRIPT=$ATTACKS_PATH/mod_userspace_file.sh

GOOD_PATH=$ATTACKS_PATH/targ_files/cds_targs/rewrite_one_good
BAD_PATH=$ATTACKS_PATH/targ_files/cds_targs/rewrite_one_bad
TARG_PATH=$DEMO_ROOT/installed_dir/bin/rewrite_one

$MOD_SCRIPT -g  $GOOD_PATH \
            -b  $BAD_PATH  \
            -t  $TARG_PATH \
            -a
