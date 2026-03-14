#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DEFAULT_AUTHOR="Ramøn"
DEFAULT_PROJECT_NAME="Tauri Audio Starter"
DEFAULT_PATH="$(dirname "$SCRIPT_DIR")/tauri-audio-starter"
DEFAULT_BUNDLE_ID="com.example.tauri_audio_starter"

prompt_with_default() {
  local label="$1"
  local default_value="$2"
  local response

  read -r -p "$label [$default_value]: " response
  if [[ -z "$response" ]]; then
    printf '%s\n' "$default_value"
  else
    printf '%s\n' "$response"
  fi
}

slugify_package_name() {
  printf '%s' "$1" \
    | tr '[:upper:]' '[:lower:]' \
    | sed -E 's/[^a-z0-9]+/-/g; s/^-+//; s/-+$//; s/-{2,}/-/g'
}

slugify_crate_name() {
  printf '%s' "$1" \
    | tr '[:upper:]' '[:lower:]' \
    | sed -E 's/[^a-z0-9]+/_/g; s/^_+//; s/_+$//; s/_{2,}/_/g'
}

require_value() {
  local field_name="$1"
  local value="$2"
  if [[ -z "$value" ]]; then
    printf 'Error: %s cannot be empty.\n' "$field_name" >&2
    exit 1
  fi
}

replace_in_file() {
  local file_path="$1"
  local from="$2"
  local to="$3"

  perl -0pi -e 's/\Q'"$from"'\E/'"$to"'/g' "$file_path"
}

project_name="$(prompt_with_default "Project name" "$DEFAULT_PROJECT_NAME")"
author="$(prompt_with_default "Author" "$DEFAULT_AUTHOR")"
target_path="$(prompt_with_default "Path" "$DEFAULT_PATH")"
bundle_identifier="$(prompt_with_default "Bundle identifier" "$DEFAULT_BUNDLE_ID")"

require_value "Project name" "$project_name"
require_value "Author" "$author"
require_value "Path" "$target_path"
require_value "Bundle identifier" "$bundle_identifier"

package_name="$(slugify_package_name "$project_name")"
crate_name="$(slugify_crate_name "$project_name")"
lib_name="${crate_name}_lib"

if [[ -e "$target_path" ]]; then
  printf 'Error: target path already exists: %s\n' "$target_path" >&2
  exit 1
fi

mkdir -p "$(dirname "$target_path")"
rsync -a \
  --exclude '.git' \
  --exclude 'node_modules' \
  --exclude 'dist' \
  --exclude 'target' \
  "$SCRIPT_DIR/" "$target_path/"

replace_in_file "$target_path/package.json" '"name": "tauri-flucoma-fundsp-solid-starter"' '"name": "'"$package_name"'"'

replace_in_file "$target_path/src-tauri/tauri.conf.json" '"productName": "Tauri Audio Starter"' '"productName": "'"$project_name"'"'
replace_in_file "$target_path/src-tauri/tauri.conf.json" '"title": "Tauri Audio Starter"' '"title": "'"$project_name"'"'
replace_in_file "$target_path/src-tauri/tauri.conf.json" '"identifier": "com.neverenginelabs.tauri-audio-starter"' '"identifier": "'"$bundle_identifier"'"'

replace_in_file "$target_path/src-tauri/Cargo.toml" 'name = "tauri-audio-starter"' 'name = "'"$crate_name"'"'
replace_in_file "$target_path/src-tauri/Cargo.toml" 'authors = ["Cristian Vogel"]' 'authors = ["'"$author"'"]'
replace_in_file "$target_path/src-tauri/Cargo.toml" 'name = "tauri_audio_starter_lib"' 'name = "'"$lib_name"'"'

printf '\nBootstrap complete.\n'
printf 'Project: %s\n' "$project_name"
printf 'Author: %s\n' "$author"
printf 'Path: %s\n' "$target_path"
printf 'Bundle identifier: %s\n' "$bundle_identifier"
