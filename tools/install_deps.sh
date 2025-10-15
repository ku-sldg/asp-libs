#!/usr/bin/env bash
set -euo pipefail

# Minimal installer script for Rust-based tools used by this repo.
# Define DEPS as a whitespace-separated list of "binary:crate" pairs.
# Example: DEPS="verusfmt:verusfmt another-bin:another-crate"

# "binary_we_need:crate_to_install ..."
DEPS="verusfmt:verusfmt"

DRY_RUN=0
AUTO_YES=0

usage() {
  echo "Usage: $0 [--dry-run|-n] [--yes|-y] [--help|-h]"
}

while (("$#")); do
  case "$1" in
    -n|--dry-run) DRY_RUN=1; shift;;
    -y|--yes) AUTO_YES=1; shift;;
    -h|--help) usage; exit 0;;
    *) echo "Unknown arg: $1"; usage; exit 1;;
  esac
done

for pair in $DEPS; do
  bin=${pair%%:*}
  crate=${pair#*:}

  if command -v "$bin" >/dev/null 2>&1; then
    echo "$bin: found"
    continue
  fi

  echo "$bin: NOT found"

  if [ "$DRY_RUN" -eq 1 ]; then
    echo "Would run: cargo install $crate"
    continue
  fi

  if [ "$AUTO_YES" -ne 1 ]; then
    read -r -p "Run 'cargo install $crate' to install $bin? (y/N) " reply
    case "$reply" in
      [Yy]) ;;
      *) echo "Skipping $bin"; continue ;;
    esac
  fi

  echo "Running: cargo install $crate"
  cargo install "$crate"
done

echo "Done."
