#!/usr/bin/env bash

set -eo pipefail
here="$(dirname "$0")"
# pacify shellcheck: cannot follow dynamic path
# shellcheck disable=SC1090,SC1091
source "$here"/patch-crates-functions.sh

usage() {
  cat <<EOF >&2
USAGE:
    $0 <AGAVE_PATH> <ATLAS_SDK_PATH>

ARGS:
    <AGAVE_PATH>        Path to the root of an agave repo
    <ATLAS_SDK_PATH>   Path to the root of a atlas-sdk repo
EOF
}

agave_path="$1"
if [ -z "$agave_path" ]; then
  usage
  exit 1
fi

atlas_sdk_path="$2"
if [ -z "$atlas_sdk_path" ]; then
  usage
  exit 1
fi

update_atlas_sdk_dependencies "$agave_path" "$atlas_sdk_path"
patch_crates_io_atlas_sdk_no_header "$agave_path"/Cargo.toml "$atlas_sdk_path"
