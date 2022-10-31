export CARGO_HOME=.cargo_cache
if rustup toolchain list | grep -q 'nightly' then
    cargo +nightly run
fi
