#![allow(dead_code)]
#![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![feature(if_let)]
#![feature(globs)]
#![feature(phase)]

#[phase(plugin)]
extern crate lazy_static;
extern crate serialize;

use std::str;
use std::char;
use std::io;
use serialize::{json, Encodable};

mod test;
mod helper;

static version:&'static str = "0.6.1";

fn charAt(arg:&str, n:uint) -> u16 {
	return arg.utf16_units().nth(n).unwrap();
}

fn regex_6(arg:&str) -> bool {
	let mut i:uint = 0;
	while (i < arg.len()) {
    match (charAt(arg, i)) {
      0x1680 |
      0x180e |
      0x2000 ... 0x200a |
      0x202f |
      0x205f |
      0x3000 |
      0xfeff =>
        { return true; },
      _ => {},
   }
    i = i + 1;
  }
  return false;
}

fn predicate_0( arg:&str) -> bool {
  return arg == "abstract" || arg == "boolean" || arg == "byte" ||
         arg == "char" || arg == "class" || arg == "double" || arg == "enum" ||
         arg == "export" || arg == "extends" || arg == "final" ||
         arg == "float" || arg == "goto" || arg == "implements" ||
         arg == "import" || arg == "int" || arg == "interface" ||
         arg == "long" || arg == "native" || arg == "package" ||
         arg == "private" || arg == "protected" || arg == "public" ||
         arg == "short" || arg == "static" || arg == "super" ||
         arg == "synchronized" || arg == "throws" || arg == "transient" ||
         arg == "volatile";
}

fn isIdentifierChar(code:int) -> bool {
  if (code < 48) {
    return code == 36;
  }
  if (code < 58) {
    return true;
  }
  if (code < 65) {
    return false;
  }
  if (code < 91) {
    return true;
  }
  if (code < 97) {
    return code == 95;
  }
  if (code < 123) {
    return true;
  }
  return code >= 0xaa; // && test(nonASCIIidentifier, fromCharCode(code));
}

fn main() {
    writeln!(io::stderr(), "Hello, world!");
    let mut a = test::AcornParser::new();

    let contents = io::File::open(&Path::new("input.js")).read_to_string().unwrap();
    println!("wow {}", contents);
    let result = a.parse(&contents.to_string());
    // println!("{}", result);
    // let output = helper::ProgramNode::inherit(&*result);
    println!("{}", json::encode(&*result));
}
