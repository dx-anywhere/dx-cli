#!/usr/bin/env sh
# dx-cli installer (Unix-like)
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/OWNER/REPO/main/scripts/install.sh | sh
#   # Optional: DXANY_VERSION=v0.1.0 DXANY_REPO_OWNER=dx-anywhere DXANY_REPO_NAME=dx-cli

set -e

REPO_OWNER="dx-anywhere"
REPO_NAME="dx-cli"
# Allow overriding via env vars for forks
[ -n "$DXANY_REPO_OWNER" ] && REPO_OWNER="$DXANY_REPO_OWNER"
[ -n "$DXANY_REPO_NAME" ] && REPO_NAME="$DXANY_REPO_NAME"

OS_RAW="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH_RAW="$(uname -m)"

# Prepare variant lists to improve compatibility with different release asset names
case "$ARCH_RAW" in
  x86_64|amd64|i686|x86)
    ARCH_VARIANTS="x86_64 amd64"
    ;;
  aarch64|arm64)
    ARCH_VARIANTS="aarch64 arm64"
    ;;
  *)
    echo "Unsupported architecture: $ARCH_RAW (supported: x86_64/amd64, aarch64/arm64)" >&2
    exit 1
    ;;
esac

case "$OS_RAW" in
  linux)
    OS_VARIANTS="linux"
    ;;
  darwin)
    # Try macos first, then darwin
    OS_VARIANTS="macos darwin"
    ;;
  msys*|mingw*|cygwin*)
    echo "Use the PowerShell installer on Windows: install.ps1" >&2
    exit 1
    ;;
  *)
    echo "Unsupported OS: $OS_RAW" >&2
    exit 1
    ;;
esac

VERSION="$DXANY_VERSION"
[ -z "$VERSION" ] && VERSION="latest"

BIN_NAME="dx"
TMP_ARCHIVE="dx_download"
EXT_VARIANTS="tar.gz zip none"

have_cmd() { command -v "$1" >/dev/null 2>&1; }

# Attempt downloads across candidate names
TRIED_URLS=""
DOWNLOADED=""
DOWNLOADED_TYPE="" # archive|binary
DOWNLOADED_EXT=""

for OSV in $OS_VARIANTS; do
  for ARV in $ARCH_VARIANTS; do
    for EXT in $EXT_VARIANTS; do
      if [ "$EXT" = "none" ]; then
        ASSET="dx-${OSV}-${ARV}"
        OUT_FILE="$BIN_NAME"
        TYPE="binary"
      else
        ASSET="dx-${OSV}-${ARV}.${EXT}"
        OUT_FILE="${TMP_ARCHIVE}.${EXT}"
        TYPE="archive"
      fi

      if [ "$VERSION" = "latest" ]; then
        URL="https://github.com/${REPO_OWNER}/${REPO_NAME}/releases/latest/download/${ASSET}"
      else
        URL="https://github.com/${REPO_OWNER}/${REPO_NAME}/releases/download/${VERSION}/${ASSET}"
      fi
      TRIED_URLS="$TRIED_URLS\n$URL"

      # Clean any leftover output file from previous attempts
      rm -f "$OUT_FILE" 2>/dev/null || true

      echo "Trying ${ASSET} from ${URL} ..."
      if have_cmd curl; then
        if curl -fL "$URL" -o "$OUT_FILE"; then
          DOWNLOADED="$OUT_FILE"; DOWNLOADED_TYPE="$TYPE"; DOWNLOADED_EXT="$EXT"; break 3
        fi
      elif have_cmd wget; then
        if wget -q -O "$OUT_FILE" "$URL"; then
          DOWNLOADED="$OUT_FILE"; DOWNLOADED_TYPE="$TYPE"; DOWNLOADED_EXT="$EXT"; break 3
        fi
      else
        echo "Neither curl nor wget is available. Please install one to download the binary." >&2
        exit 1
      fi
    done
  done
done

if [ -z "$DOWNLOADED" ] || [ ! -s "$DOWNLOADED" ]; then
  echo "Failed to download dx-cli release asset. Tried the following URLs:" >&2
  # shellcheck disable=SC2001
  echo "$(echo "$TRIED_URLS" | sed 's/^/  - /')" >&2
  echo "Hint: Set DXANY_VERSION (e.g., DXANY_VERSION=v0.1.0) if you need a specific tag, and ensure the asset naming matches your platform." >&2
  exit 1
fi

echo "Download succeeded: $DOWNLOADED"

if [ "$DOWNLOADED_TYPE" = "archive" ]; then
  echo "Extracting ${DOWNLOADED} ..."
  case "$DOWNLOADED_EXT" in
    tar.gz)
      if have_cmd tar; then
        tar -xzf "$DOWNLOADED"
      else
        echo "tar is required to extract ${DOWNLOADED}" >&2
        exit 1
      fi
      ;;
    zip)
      if have_cmd unzip; then
        unzip -o "$DOWNLOADED"
      else
        echo "unzip is required to extract ${DOWNLOADED}" >&2
        exit 1
      fi
      ;;
  esac
else
  # already the binary as $BIN_NAME
  chmod +x "$BIN_NAME" 2>/dev/null || true
fi

# Ensure final binary exists
if [ -f "$BIN_NAME" ]; then
  chmod +x "$BIN_NAME" 2>/dev/null || true
  echo "Installed ./dx"
  echo "Run: ./dx --help"
else
  # Try to find a likely binary in current dir
  if [ -f "dx-cli" ]; then
    mv dx-cli "$BIN_NAME"
    chmod +x "$BIN_NAME" 2>/dev/null || true
    echo "Installed ./dx"
    echo "Run: ./dx --help"
  else
    echo "Could not find the binary after extraction. Please check release asset contents." >&2
    exit 1
  fi
fi

# Clean up archive if any
rm -f ${TMP_ARCHIVE}.tar.gz ${TMP_ARCHIVE}.zip 2>/dev/null || true
