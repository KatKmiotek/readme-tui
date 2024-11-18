#!/bin/bash

set -e

OWNER="KatKmiotek"
REPO="readme-tui"
BINARY_NAME="cli-doc"
INSTALL_DIR="/usr/local/bin"

GREEN='\033[0;32m'
NC='\033[0m'

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

echo -e "${GREEN}Successfully installed $BINARY_NAME! 🎉${NC}"
echo "Run '$BINARY_NAME' to use this TUI."
