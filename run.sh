if [ "$1" == "--debug" ]
    then cargo run --features trace | pegviz --output ./peg-debug.html
else
    cargo run
fi
