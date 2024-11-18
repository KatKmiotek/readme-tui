#!/bin/bash

set -e

OWNER="KatKmiotek"
REPO="readme-tui"
BINARY_NAME="cli-doc"
INSTALL_DIR="/usr/local/bin"

echo "📦 Installing $BINARY_NAME..."

echo "🔍 Fetching latest release..."

VERSION=$(curl -s "https://api.github.com/repos/$OWNER/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

echo "The latest version is $VERSION"
DOWNLOAD_URL="https://github.com/$OWNER/$REPO/releases/download/$VERSION/$BINARY_NAME.zip"
echo "Download url is $DOWNLOAD_URL"

TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

echo "⬇️  Downloading latest release..."
curl -sL --progress-bar "$DOWNLOAD_URL" -o "$TMP_DIR/$BINARY_NAME.zip"
unzip -q "$TMP_DIR/$BINARY_NAME.zip" -d "$TMP_DIR"

echo "📝 Installing $BINARY_NAME to $INSTALL_DIR..."
sudo mv "$TMP_DIR/$BINARY_NAME" "$INSTALL_DIR/"
sudo chmod +x "$INSTALL_DIR/$BINARY_NAME"

SHELL_CONFIG=""
if [ -n "$ZSH_VERSION" ]; then
    SHELL_CONFIG="$HOME/.zshrc"
elif [ -n "$BASH_VERSION" ]; then
    SHELL_CONFIG="$HOME/.bash_profile"
    if [ ! -f "$SHELL_CONFIG" ]; then
        SHELL_CONFIG="$HOME/.bashrc"
    fi
fi

if [ -n "$SHELL_CONFIG" ]; then
    if ! echo $PATH | grep -q "/usr/local/bin"; then
        echo 'export PATH="/usr/local/bin:$PATH"' >> "$SHELL_CONFIG"
        echo "Added /usr/local/bin to PATH in $SHELL_CONFIG"
        echo "Run 'source $SHELL_CONFIG' to update your current session"
    fi
fi

echo "Successfully installed $BINARY_NAME! 🎉"
echo "Run '$BINARY_NAME' to use this TUI."
