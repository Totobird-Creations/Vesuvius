export CARGO_HOME=.cargo_cache

#rustup toolchain list | grep -q 'nightly' || rustup default nightly
#cargo +nightly run

rustup toolchain list | grep -q 'stable' || rustup default stable
cargo +stable run
