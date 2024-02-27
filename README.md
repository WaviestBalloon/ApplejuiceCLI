# <img src="assets/crudejuice.png" width=85px> ApplejuiceCLI
### ApplejuiceCLI is a light-weight, fast Roblox on Linux bootstrapper that runs with Proton!

> [!WARNING]
> **Playing Roblox under Wine no longer works due to the Hyperion block being readded to Roblox [due to multiple reasons](https://devforum.roblox.com/t/why-isnt-hyperion-an-anti-cheat/2840095/33?u=waviestballoon).**
>
> **Studio will work as intended, Applejuice will still continue to be maintained.**

You can either use the interface (Soon™) or if you're big brain, use the CLI instead. (**Less bloat! Wow!**)

*Applejuice is a manager to get Roblox to run on Linux using Valve's Proton.*

> [!WARNING]
> Applejuice is still in VERY EARLY development, and may not work as expected.

> [!IMPORTANT]
> For issues you encounter while using Roblox, please refer to [this pinned issue](https://github.com/WaviestBalloon/ApplejuiceCLI/issues/1)!

## Installation

```bash
git clone https://github.com/WaviestBalloon/ApplejuiceCLI.git ; cd ApplejuiceCLI ; chmod +x ./install.sh ; bash ./install.sh
```

## Compiling from source (Read me)

When running the install script, it will compile the binary for you as of now because there are no pre-compiled binaries available yet.

> [!IMPORTANT]
> If compile fails, you might be missing dependencies: 
> - SDL (Monitor Hertz detection, FPS uncapping)
> - build-essentials/base-devel (Compiling)
> - libssl-dev/openssl (Compiling)
>
> Debian/Ubuntu: 
> ```
> sudo apt-get -y install build-essential libsdl2-dev libssl-dev
> ```
> Arch: 
> ```
> sudo pacman -Sy base-devel sdl2 openssl --noconfirm
> ```

### Using the install script

1. Clone this repository.
2. Run `./install.sh`. (You may need to run `chmod +x ./install.sh` first)
3. Launch Roblox via your application launcher or from the website!

### Manual (Not recommended - You will not get support)

1. Clone this repository.
2. Make sure you have Rust and required dependencies installed.
3. Run `cargo build --release` in the repository.
4. Copy the compiled binary from `./target/release/applejuicecli` to `/usr/local/bin`.
5. Run `applejuicecli --init` to initialise the configuration file and directories.
6. Launch Roblox via your application launcher or from the website!

### Uninstalling

Run the install script with the `--uninstall` flag.

> [!WARNING]
> **This will leave nothing left of ApplejuicCLI!**
> 
> The following WILL be lost if you uninstall:
> - Roblox configuration files
> - Roblox in-game screenshots
> - FastFlags configuration
> - ApplejuiceCLI configuration files
> - And more
> 
> **Please consider backing up ANY and ALL data you wish to keep!**
