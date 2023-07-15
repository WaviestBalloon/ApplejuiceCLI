#!/bin/bash
set -e
echo "Hello! This is the Applejuice CLI installer, keep in mind that this is still in development and may not work as expected.\nFurthermore, this installer will require sudo privileges to install the CLI to /usr/local/bin."
echo ""

echo "Moving to project directory..."
cd $( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

while true; do
	read -p "This clone might be out of date.
Do you want to pull the latest changes from the repository? (y/n): " yn
	case $yn in
		[Yy]* ) git pull; break;;
		[Nn]* ) break;;
		* ) echo "Please answer yes or no.";;
	esac
done


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

# Compile everything!!
echo "Compiling the Applejuice CLI... (With --release)"
echo "---------------------------"
cargo build --release --verbose
echo "---------------------------"

echo "Installing the Applejuice CLI to /usr/local/bin..."
sudo cp ./target/release/applejuice_cli /usr/local/bin/applejuicecli

echo "Initialising Applejuice..."
echo "---------------------------"
applejuicecli --init
echo "---------------------------"

echo ""
echo "Success: You can now run the Applejuice CLI by typing 'applejuicecli' in your terminal."
