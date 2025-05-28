#!/bin/bash

if [ -z ${ATTACK_BIN+x} ]; then
  echo "Variable 'ATTACK_BIN' is not set" 
  echo "Run: 'export ATTACK_BIN=<path-to-asp-libs/attacks/>'"
  exit 1
fi

if [ -z ${THEOREMS_ROOT+x} ]; then
  echo "Variable 'THEOREMS_ROOT' is not set" 
  echo "Run: 'export THEOREMS_ROOT=<path-to-theorems_root>'"
  exit 1
fi