# A Remote Access Gateway
* Webassembly Terminal Services written with Rust / Wasm

## Dependencies

* rust
* cargo-make
* wasm-pack

## Build

* Debug
    - `sh run.sh d <target_server>:<port>`
* Relese
    - `sh run.sh r <target_server>:<port>`

## Milestones

* VNC Clients:
    - Raw encoding support
    - Tight encoding support (Default)
    - ZRLE encoding support
    - Other encoding support (WIP)

* SSH Clients:
    - WIP

* RDP Clients:
    - WIP
