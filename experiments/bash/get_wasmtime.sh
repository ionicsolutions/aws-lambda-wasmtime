#!/usr/bin/env bash

# Based on https://wasmtime.dev/install.sh

get_latest_release() {
  curl -w "%{stderr}" --silent "https://api.github.com/repos/bytecodealliance/wasmtime/releases/latest" | \
    tr -d '\n' | \
    sed 's/.*tag_name": *"//' | \
    sed 's/".*//'
}

release_url() {
  echo "https://github.com/bytecodealliance/wasmtime/releases"
}

download_release_from_repo() {
  local version="$1"
  local filename="wasmtime-$version-x86_64-linux.tar.xz"
  local archive_url
  archive_url="$(release_url)/download/$version/$filename"
  local download_file="$2"

  curl --progress-bar --show-error --location --fail "$archive_url" \
       --output "$download_file" && echo "$download_file"
}

# Download the latest wasmtime release
version=$(get_latest_release)
archive_file="wasmtime-x86_64-linux.tar.xz"
download_release_from_repo "$version" "$archive_file"

# Move the binary to the right place
tar -xvf "$archive_file" "wasmtime-$version-x86_64-linux/wasmtime" --strip-components=1
chmod +x wasmtime
mv wasmtime ./runtime/wasmtime
rm "$archive_file"
