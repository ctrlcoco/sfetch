#!/bin/bash

PROGRAM_NAME="sfetch"

if ! cargo --version &>/dev/null; then
	echo "[ERROR] cargo not found."
	exit 2
fi

# default on current directory
BUILD_FOLDER=${1:-$(pwd)}

[ -d "$BUILD_FOLDER" ] || {
	echo "[ERROR] Directory $BUILD_FOLDER not found!"
	exit 1
}

echo "Build folder: $BUILD_FOLDER"

cargo fmt
cargo build --release --target-dir "$BUILD_FOLDER"

BUILD_RESULT=$?

# Check the result of the build
if [[ $BUILD_RESULT -eq 0 ]]; then
	if [[ -f "$BUILD_FOLDER"/release/"$PROGRAM_NAME" ]]; then
		mv "$BUILD_FOLDER"/release/"$PROGRAM_NAME" "$BUILD_FOLDER"

		cargo clean --release --target-dir "$BUILD_FOLDER"

		echo "[info] Executable fully compiled on $BUILD_FOLDER"
	else
		echo "[ERROR] Compilation succeeded, but no binary found in target/release."
	fi
else
	echo "[ERROR] Cargo project failed to compile."
	exit $BUILD_RESULT
fi
