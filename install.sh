#!/bin/bash
set -e
echo "Hello! This is the Applejuice CLI installer, keep in mind that this is still in development and may not work as expected.\nFurthermore, this installer will require sudo privileges to install the CLI to /usr/local/bin."
echo ""

if [[ $EUID -eq 0 ]]; then
	echo "This script is restricted to run as a non-root user only. Do not run as root or sudo."
	exit 1
fi

echo "Checking if cargo is usable..."
if ! command -v cargo &> /dev/null
then
	echo "Cargo could not be found, please install Rust from https://rustup.rs/."
	exit 1
fi
echo "^ Done!"

# Compile everything!!
echo "Compiling the Applejuice CLI... (With --release)"
cargo build --release
echo "^ Done!"

echo "Installing the Applejuice CLI to /usr/local/bin..."
sudo cp ./target/release/applejuice_cli /usr/local/bin/applejuicecli
echo "^ Done!"

echo "Initialising Applejuice..."
echo "---------------------------"
applejuicecli --init
echo "---------------------------"
echo "^ Done!"

echo ""
echo "Success: You can now run the Applejuice CLI by typing 'applejuicecli' in your terminal."
