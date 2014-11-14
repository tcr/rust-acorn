# acorn.rs

A port of [Acorn.js](http://marijnhaverbeke.nl/acorn/) to Rust. It's half-automated and wicked janky! Be careful.

To run:

```
git clone https://github.com/tcr/acorn.rs.git
cd acorn.rs
cargo build

cat input.js
# console.log('hello world!')

./target/hello_world 2>/dev/null
# {"bodylist":[{"expression":{"callee":{"object":"console","property":"log"},"arguments":["hello world!"]}}]}
```

Edit input.js to get a different JSON parse tree.

## license

MIT. Copyright by Marijn Haverbeke!