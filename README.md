# rust-acorn

A port of [Acorn.js](http://marijnhaverbeke.nl/acorn/) to Rust. It's a half-automated translation and wicked janky! The fork is from commit ef045b971834a34e5181f8e2eeac34644e2ede41, which will eventually have to be rolled forward.

Acorn is a lightweight ECMAScript parser with support for versions 3-6. It outputs an AST equivalent to Esprima's or the Mozilla Parser AST.

To try it out:

```
git clone https://github.com/tcr/rust-acorn.git
cd rust-acorn
cargo build

echo "console.log('hello world!')" | ./target/acorn -

make test
# Runs acorn.js test suite.
```

## license

MIT. Acorn.js copyright Marijn Haverbeke
