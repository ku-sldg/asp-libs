#!/bin/bash

MOD_SCRIPT=./mod_userspace_file.sh

ATTACKS_PATH=$ASP_BIN/../attacks

GOOD_PATH=$ATTACKS_PATH/targ_files/filter_one_config.json
BAD_PATH=$ATTACKS_PATH/targ_files/filter_one_config_bad.json
TARG_PATH=$DEMO_ROOT/cds_config/filter_one_config.json

$MOD_SCRIPT -g  $GOOD_PATH \
            -b  $BAD_PATH  \
            -t  $TARG_PATH \
            -r
