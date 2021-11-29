set -e

if [ "r" = "$1" ]; then
    cargo make install-release
else 
    cargo make install-debug
fi

cd build && ./webgateway-be
