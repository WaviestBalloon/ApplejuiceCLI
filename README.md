# ApplejuiceCLI
ApplejuiceCLI is the backbone and bootstrapper of Applejuice, you can either use the interface or if you're big brain, use the CLI instead. (**Less bloat! Wow!**)

*Applejuice is a manager to get Roblox to run on Linux using Valve's Proton.*

> [!WARNING]
> Applejuice is still in VERY EARLY development, and may not work as expected.

> [!IMPORTANT]
> For issues you encounter while using Roblox, please refer to [this pinned issue](https://github.com/WaviestBalloon/ApplejuiceCLI/issues/1)!

## Installation

> [!IMPORTANT]
> If compile fails, you might be missing a dependency with SDL. So far, Ubuntu seems to be the one that has issues with compiling, make sure you run `sudo apt-get -y install libsdl2-dev` before installing and it should successfully compile.

```bash
git clone https://github.com/WaviestBalloon/ApplejuiceCLI.git ; cd ApplejuiceCLI ; chmod +x ./install.sh ; bash ./install.sh
```

### Using the install script

1. Clone this repository.
2. Run `./install.sh`. (You may need to run `chmod +x ./install.sh` first)
3. Enter your sudo password to continue after the binary compiles.
4. Run `applejuicecli --install player` to install the Roblox Player!

### Manual

1. Clone this repository.
2. Make sure you have Rust installed.
3. Run `cargo build --release` in the repository.
4. Copy the compiled binary from `./target/release/applejuice_cli` to `/usr/local/bin` with the name `applejuicecli`.
5. Run `applejuicecli --init` to initialise the configuration directory.
6. Run `applejuicecli --install player` to install the Roblox Player!
