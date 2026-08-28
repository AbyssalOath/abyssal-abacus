# Abyssal Abacus

A calculator written in Rust, available two ways from the same shared math engine:

- **Abyssal Abacus (CLI)** - a command-line calculator you type expressions into.

- **Abyssal Abacus (GUI)** - a retro, dark-themed desktop calculator you click or type numbers into.

Both support addition, subtraction, multiplication, division, and modulo.

## Features

- Addition (`+`)

- Subtraction (`-`)

- Multiplication (`*`)

- Division (`/`)

- Modulo (`%`)

- Decimal numbers

- Negative numbers

- Division and modulo by zero resolve to `0` instead of erroring

- GUI: retro black/red desktop-calculator look, full keyboard support, resizable window

- CLI: case-insensitive `exit`/`quit` commands, one-shot mode for scripts (`abyssal-abacus-calc "3+4"`), friendly error messages for invalid input

## Download

Pre-built binaries are available on the [Releases](https://github.com/LordSodomiser/abyssal-abacus/releases) page. The CLI and the GUI are packaged separately, so you only need to grab the one you actually want.

### Linux x86-64

**CLI** - download abyssal-abacus-cli-linux-x86_64.tar.xz, extract, and run:

```
tar -xJf abyssal-abacus-cli-linux-x86_64.tar.xz
chmod +x abyssal-abacus-cli
./abyssal-abacus-cli
```

To install it system-wide so you can run `abyssal-abacus-calc` from anywhere:

```
sudo cp abyssal-abacus-cli /usr/local/bin/abyssal-abacus-cli
```

**GUI** - download `abyssal-abacus-linux-x86_64.tar.xz`, extract, and either double-click the `abyssal-abacus` binary from your file manager (most will offer to run it) or launch it from a terminal:

```
tar -xJf abyssal-abacus-linux-x86_64.tar.xz 
./abyssal-abacus
```

### Windows x86-64

**CLI** - download `abyssal-abacus-cli-windows-x86_64.zip`, extract it, and run from PowerShell or Command Prompt:

```
.\abyssal-abacus-cli.exe
```

**GUI** - download `abyssal-abacus-windows-x86_64.zip`, extract it, and double-click `abyssal-abacus.exe`.

### macOS

**CLI** - download `abyssal-abacus-cli-macos-x86_64.tar.gz`, extract, and run:

```
tar -xzf abyssal-abacus-cli-macos-x86_64.tar.gz 
chmod +x abyssal-abacus-cli
./calc
```

**GUI** - download `abyssal-abacus-macos.dmg`, open it, and drag **Abyssal Abacus** into Applications, then double-click it to launch. Since it isn't notarized by Apple, the first launch will show an "unidentified developer" warning - right-click the app and choose **Open** once to bypass it; after that it opens normally.

## Usage (CLI)

Start the calculator and enter an expression:

```
> 10+5
15  
  
> 10-3
7  
  
> 6*7
42  
  
> 204
5  
  
> 10%3
1
```

Decimal numbers are supported:

```
> 10.5+2.5
13
```

Negative numbers are supported:

```
> -5+3
-2  
  
> 5*-3
-15
```

To exit the calculator:

```
> exit
Goodbye!
```

You can also use:

```
> quit
Goodbye!
```

One-shot mode, handy for scripts - pass the expression as an argument and skip the prompt entirely:

```
abyssal-abacus-cli "3 + 4"
7
```

## Usage (GUI)

Click the keypad with your mouse, or use the keyboard:

| Key(s) | Action |
| - | - |
| `0`-`9`, `.` | Digits/decimal |
| `+ - * / %` | Operators |
| `Enter` or `=` | Equals |
| `Escape` or `C` | Clear |
| `Backspace` | Delete last digit |


Drag the title bar to move the window, drag the bottom-right corner to resize it, or double-click the title bar to maximize/restore.

## Error Handling

Invalid expressions return an error instead of crashing the calculator:

```
> hello 
Error: Invalid expression: 'hello'. Use format: a+b, a-b, a*b, a/b, a%b
```

Division and modulo by zero don't error - they resolve to `0`:

```
> 10/0
0

> 10%0  
0
```

## Building from Source

### Requirements

- [Rust](https://www.rust-lang.org/tools/install)

- Cargo, which is included with Rust

- Linux only, for building the GUI: `libxkbcommon-dev libx11-dev libgl1-mesa-dev libxcb1-dev` (or your distro's equivalents)

Clone the repository:

```
git clone https://github.com/LordSodomiser/abyssal-abacus.git
cd abyssal-abacus
```

Run the shared logic's test suite:

```
cargo test -p abyssal_abacus_core
```

Build a release version of either or both:

```
cargo build --release -p abyssal_abacus_cli
cargo build --release -p abyssal_abacus_gui
```

The resulting binaries will be located at:

```
target/release/abyssal_abacus_cli   (abyssal_abacus_cli.exe on Windows)
target/release/abyssal_abacus_gui   (abyssal_abacus_gui.exe on Windows)
```

Run them directly:

```
./target/release/abyssal_abacus_cli
./target/release/abyssal_abacus_gui
```

## Project Structure

```
abyssal-abacus/
├── Cargo.toml              <- workspace root
├── README.md
├── .github/
│   └── workflows/
│       └── release.yml     <- builds & publishes Linux/macOS/Windows releases
├── abyssal_abacus_core/                <- shared math + calculator state, no UI code
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
├── abyssal_abacus_cli/                  <- command-line front end
│   ├── Cargo.toml
│   └── src/
│       └── main.rs 
└── abyssal_abacus_gui/                   <- retro GUI front end (egui/eframe)
    ├── Cargo.toml
    └── src/
        └── main.rs
```

Build artifacts and release packages are not tracked in Git:

```
target/  
dist/
```

Pre-built binaries for Linux, macOS, and Windows are published automatically to GitHub Releases by `.github/workflows/release.yml` whenever a `vX.Y.Z` tag is pushed.

## License

See the [LICENSE](LICENSE) file for license information.
