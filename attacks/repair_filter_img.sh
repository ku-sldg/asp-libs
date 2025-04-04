#!/bin/bash

MOD_SCRIPT=./mod_userspace_file.sh

ATTACKS_PATH=$ASP_BIN/../attacks

GOOD_PATH=$ATTACKS_PATH/targ_files/filter_one_good
BAD_PATH=$ATTACKS_PATH/targ_files/filter_one_bad
TARG_PATH=$DEMO_ROOT/installed_dir/bin/filter_one

$MOD_SCRIPT -g  $GOOD_PATH \
            -b  $BAD_PATH  \
            -t  $TARG_PATH \
            -r