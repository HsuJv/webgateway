set -e
cargo make install
cd build && ./webgateway-be
