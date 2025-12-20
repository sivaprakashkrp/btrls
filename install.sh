#! /bin/bash

cargo build --release

path="$(pwd)/target/release"

BASHRC_FILE="$HOME/.bashrc"

# Check if the path already exists in .bashrc to prevent duplicates
if ! grep -q "export PATH=.*$path" "$BASHRC_FILE"; then
    echo "export PATH=\"\$PATH:$path\"" >> "$BASHRC_FILE"
    echo "Path added to $BASHRC_FILE."
else
    echo "Path already exists in $BASHRC_FILE."
fi

# Source .bashrc to apply changes immediately in the current session
source "$BASHRC_FILE"
echo "Changes applied to the current shell session."

btrls