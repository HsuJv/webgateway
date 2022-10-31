# A Remote Access Gateway
* Webassembly Terminal Services written with Rust / Wasm

## Dependencies

* rust
* cargo-make
* wasm-pack
* wasm-opt
* libssl-devel

## Build

* Debug
    - `sh run.sh d <target_server>:<port>`
* Relese
    - `sh run.sh r <target_server>:<port>`

## Milestones

* VNC Clients:
    - Basic functions work

* SSH Clients:
    - WIP

* RDP Clients:
    - A very easy client has already done
    - Further feature & bugfix is in progress
