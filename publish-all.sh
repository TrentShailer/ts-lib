#!/env/bin/bash
set -e 
set -o pipefail

cargo publish -p ts-ansi
cargo publish -p ts-path
cargo publish -p ts-error
cargo publish -p ts-io
cargo publish -p ts-json
cargo publish -p ts-config
cargo publish -p ts-terminal
