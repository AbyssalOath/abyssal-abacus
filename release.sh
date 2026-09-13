#!/usr/bin/env bash
set -euo pipefail

DIST="dist"

# ============================================================
# Clean dist
# ============================================================

echo "==> Cleaning dist/..."

rm -rf "$DIST"
mkdir -p "$DIST"


# ============================================================
# Build
# ============================================================

echo "==> Building Linux x86_64..."

cargo build --release \
    -p abyssal_abacus_cli \
    -p abyssal_abacus_gui \
    --target x86_64-unknown-linux-gnu


echo "==> Building Windows x86_64..."

cargo build --release \
    -p abyssal_abacus_cli \
    -p abyssal_abacus_gui \
    --target x86_64-pc-windows-gnu


# ============================================================
# Linux - CLI
# ============================================================

echo "==> Packaging abyssal-abacus-cli Linux..."

mkdir -p "$DIST/abyssal-abacus-cli-linux-x86_64"

cp \
    target/x86_64-unknown-linux-gnu/release/abyssal_abacus_cli \
    "$DIST/abyssal-abacus-cli-linux-x86_64/abyssal-abacus-cli"

tar -cJf \
    "$DIST/abyssal-abacus-cli-linux-x86_64.tar.xz" \
    -C "$DIST" \
    abyssal-abacus-cli-linux-x86_64

rm -rf "$DIST/abyssal-abacus-cli-linux-x86_64"


# ============================================================
# Linux - GUI
# ============================================================

echo "==> Packaging abyssal-abacus Linux..."

mkdir -p "$DIST/abyssal-abacus-linux-x86_64"

cp \
    target/x86_64-unknown-linux-gnu/release/abyssal_abacus_gui \
    "$DIST/abyssal-abacus-linux-x86_64/abyssal-abacus"

tar -cJf \
    "$DIST/abyssal-abacus-linux-x86_64.tar.xz" \
    -C "$DIST" \
    abyssal-abacus-linux-x86_64

rm -rf "$DIST/abyssal-abacus-linux-x86_64"


# ============================================================
# Windows - CLI
# ============================================================

echo "==> Packaging abyssal-abacus-cli Windows..."

mkdir -p "$DIST/abyssal-abacus-cli-windows-x86_64"

cp \
    target/x86_64-pc-windows-gnu/release/abyssal_abacus_cli.exe \
    "$DIST/abyssal-abacus-cli-windows-x86_64/abyssal-abacus-cli.exe"

(
    cd "$DIST"
    zip -qr \
        abyssal-abacus-cli-windows-x86_64.zip \
        abyssal-abacus-cli-windows-x86_64
)

rm -rf "$DIST/abyssal-abacus-cli-windows-x86_64"


# ============================================================
# Windows - GUI
# ============================================================

echo "==> Packaging abyssal-abacus Windows..."

mkdir -p "$DIST/abyssal-abacus-windows-x86_64"

cp \
    target/x86_64-pc-windows-gnu/release/abyssal_abacus_gui.exe \
    "$DIST/abyssal-abacus-windows-x86_64/abyssal-abacus.exe"

(
    cd "$DIST"
    zip -qr \
        abyssal-abacus-windows-x86_64.zip \
        abyssal-abacus-windows-x86_64
)

rm -rf "$DIST/abyssal-abacus-windows-x86_64"


# ============================================================
# Done
# ============================================================

echo
echo "==> Release artifacts:"
echo

find "$DIST" -maxdepth 1 -type f -printf '  %f\n' | sort

echo
echo "==> Done!"
