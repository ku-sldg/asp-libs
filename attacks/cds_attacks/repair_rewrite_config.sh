#!/bin/bash

ATTACKS_PATH=$ATTACK_BIN

MOD_SCRIPT=$ATTACKS_PATH/mod_userspace_file.sh

GOOD_PATH=$ATTACKS_PATH/targ_files/cds_targs/rewrite_one_config.json
BAD_PATH=$ATTACKS_PATH/targ_files/cds_targs/rewrite_one_config_bad.json
TARG_PATH=$DEMO_ROOT/cds_config/rewrite_one_config.json

$MOD_SCRIPT -g  $GOOD_PATH \
            -b  $BAD_PATH  \
            -t  $TARG_PATH \
            -r
