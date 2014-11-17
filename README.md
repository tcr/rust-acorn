# rust-acorn

A port of [Acorn.js](http://marijnhaverbeke.nl/acorn/) to Rust. It's a half-automated translation and wicked janky! (The fork is from commit ef045b, which will eventually be rolled forward.)

Acorn is a lightweight ECMAScript parser with support for versions 3-6. It outputs an AST equivalent to Esprima's or the Mozilla Parser AST.

To try it out:

```
git clone https://github.com/tcr/rust-acorn.git
cd rust-acorn
cargo build
echo "console.log('hello world!')" | ./target/acorn -
```

```js
{"_type":"Program","loc":{"start":{"line":1,"column":0},"end":{"line":1,"column":28}},"body":[{"_type":"ExpressionStatement","loc":{"start":{"line":1,"column":0},"end":{"line":1,"column":28}},"expression":{"_type":"CallExpression","loc":null,"callee":{"_type":"MemberExpression","loc":null,"object":{"_type":"Identifier","loc":{"start":{"line":1,"column":0},"end":{"line":1,"column":7}},"name":"console"},"property":{"_type":"Identifier","loc":{"start":{"line":1,"column":8},"end":{"line":1,"column":11}},"name":"log"},"computed":false},"arguments":[{"_type":"Literal","loc":{"start":{"line":1,"column":12},"end":{"line":1,"column":27}},"value":"hello worldls","raw":"'hello worldls'"}]}}]}
```

You can also specify a filename `./target/acorn input.js`, or run the acorn.js test suite with `make test`.

## license

MIT. Acorn.js copyright Marijn Haverbeke
