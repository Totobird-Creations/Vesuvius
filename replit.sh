export CARGO_HOME=.cargo_cache
rustup toolchain list | grep -q 'nightly' || rustup default nightly
cargo +nightly run
