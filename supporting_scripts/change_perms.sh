#!/bin/bash

run_command_on_file() {
    local relative_path="$1"
    local target_file="$(dirname "$(realpath "$0")")/$relative_path"

    # Check if the target file exists
    if [[ ! -f "$target_file" ]]; then
        echo "Error: Target file '$target_file' not found."
        return 1
    fi

    chown root:root "$target_file"
    chmod u+s "$target_file"

    if [[ $? -ne 0 ]]; then
        echo "Error: Commands failed on '$target_file'."
        return 2
    fi

    echo "Commands executed successfully on '$target_file'"
}

# Example usage
run_command_on_file "../bin/r_invary_get_measurement_id"
run_command_on_file "../bin/r_selinux_pol_dump"
