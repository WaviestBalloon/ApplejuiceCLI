#!/bin/bash
set -e
git_hash=$(git rev-parse --short HEAD)

# Intro
echo "Hello! This is the Applejuice CLI installer, keep in mind that this is still in development and may not work as expected.
Furthermore, this installer will require sudo privileges to install the CLI to /usr/local/bin."
echo ""
echo "Current git hash: #$git_hash"
echo ""

# Checks
if [[ $EUID -eq 0 ]]; then
	echo "This script is restricted to run as a non-root user only. Do not run as root or sudo."
	exit 1
fi
if [[ -f /usr/local/bin/applejuicecli ]]; then
	echo "Applejuice is already installed, the binary will be overwritten but your configuration will be kept unharmed."
fi
if pgrep -x "applejuicecli" > /dev/null; then
	echo "Applejuice is already running and cannot be installed, please quit out of Roblox and Applejuice in order to continue!"
	exit 1
fi

echo "Moving to project directory..."
cd $( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

while true; do
	read -p "Do you want to pull the latest changes from the repository? (y/n): " yn
	case $yn in
		[Yy]* ) git pull --force; break;;
		[Nn]* ) break;;
		* ) echo "Please answer y or n.";;
	esac
done

echo "Checking if cargo is usable..."
if ! command -v cargo &> /dev/null
then
	echo "Cargo could not be found, please install Rust from https://rustup.rs/."
	exit 1
fi

# Compile everything!!
echo "Compiling the Applejuice CLI... (With --release)"
echo "---------------------------"
cargo build --release
echo "---------------------------"

# Install it!!
echo "Installing the Applejuice CLI to /usr/local/bin..."
sudo cp ./target/release/applejuice_cli /usr/local/bin/applejuicecli || doas cp ./target/release/applejuice_cli /usr/local/bin/applejuicecli

# Applejuice initalisation also downloads these asset files if missing as a fallback
echo "Copying asset files..."
mkdir -p ~/.local/share/applejuice
mkdir -p ~/.local/share/applejuice/assets
cp -r ./assets/* ~/.local/share/applejuice/assets

echo "Initialising Applejuice..."
echo "---------------------------"
applejuicecli --init
echo "---------------------------"

# Finish line: 
echo ""
echo "Success: You can now run the Applejuice CLI by typing 'applejuicecli' in your terminal."
