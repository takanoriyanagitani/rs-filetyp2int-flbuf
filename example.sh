#!/bin/bash

set -u

wsm="./target/wasm32-wasip1/release-wasi/filetyp2int-flbuf.wasm"

out="./filetype-string-to-enum-map.flex"
jo="${out%.*}.json"

wasmtime run "${wsm}" |
  cat > "${out}"

flatc --json --strict-json --flexbuffers "${out}" &&
  cat "${jo}" |
  jq
