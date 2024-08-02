cargo build
perf record -F99 --call-graph dwarf cargo run > /dev/null
