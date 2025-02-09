# 🐧 distrohoop 🐧

distroHoop is a CLI tool written in Rust that helps you determine your next Linux distribution. It uses the mighty force of the RNG to determine where to hop on next!

<div align="center">
    <img src="/img/intro.png" alt="Intro Image" width="600">
</div>

## 🛠️ Installation

### NixOS
Add to your *flake.nix*:
```nix
inputs = {
    distrohoop.url = "github:br0sinski/distrohoop";
}
```

Then, you can use in `home.packages` or `environment.systemPackages`:
```nix
inputs.distrohoop.packages.${pkgs.system}.default
```

### Download a Binary

Visit the [releases page](https://github.com/br0sinski/distrohoop/releases) and download a binary for your system.

### Build from Source

To build from source, you need to have Rust and Cargo installed on your system. You can install Rust and Cargo by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

Clone the repository and build the project:

```sh
git clone https://github.com/br0sinski/distrohoop.git
cd distrohoop
cargo build --release
```

Or use the `install.sh` script to build and automatically move the binary file to `/usr/local/bin`:

```sh
./install.sh
```

### From the AUR

For Arch Linux users, you can install distroHoop from the AUR:
```sh
yay -S distrohoop-bin
```
- or if you want to compile form source:

```sh
yay -S distrohoop
```

or

```sh
git clone https://aur.archlinux.org/distrohoop.git
cd distrohoop
makepkg -si
```

## 🚀 Usage

### If downloaded from the AUR / used the install script
```sh
distrohoop
```

### If You Downloaded a Binary

#### Linux

In a terminal, navigate to the directory where you downloaded the binary file and run:

```sh
./distrohoop
```

#### Windows

Simply open the `.exe` file or run in a terminal:

```sh
./distrohoop.exe
```

#### macOS

In a terminal, navigate to the directory where you downloaded the binary file and run:

```sh
./distrohoop
```

### If Compiled from Source

Run the distroHoop CLI tool:

```sh
cargo run --release
```

## 📊 Example Output

<div align="center">
    <img src="/img/example.gif" alt="Example Output" width="600">
</div>

## 🤝 Contributing

Contributions are welcome! Please open an issue or submit a pull request if you have any improvements or new features to suggest.

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgements

- [colored](https://github.com/mackwic/colored) for terminal text coloring
- [crossterm](https://github.com/crossterm-rs/crossterm) for cross-platform terminal manipulation
- [rand](https://github.com/rust-random/rand) for random number generation

