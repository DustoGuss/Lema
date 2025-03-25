#!/bin/bash

set -e  

BIN_NAME="lema" 
INSTALL_DIR="/usr/local/bin" 

echo "Compiling Lema..."
cargo build --release >/dev/null 2>&1

BIN_PATH="target/release/$BIN_NAME"

if [ ! -f "$BIN_PATH" ]; then
    echo "ERROR: binary not found in $BIN_PATH"
    exit 1
fi

echo "Installing $BIN_NAME in $INSTALL_DIR..."
sudo cp "$BIN_PATH" "$INSTALL_DIR/"
sudo chmod +x "$INSTALL_DIR/$BIN_NAME"

echo "Lema is installed! you can run it with '$BIN_NAME'."

