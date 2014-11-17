#![allow(dead_code)]
#![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(unused_must_use)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(unused_unsafe)]
#![allow(unused_imports)]

#![feature(if_let)]
#![feature(globs)]
#![feature(phase)]

#[phase(plugin)]
extern crate lazy_static;
extern crate serialize;

use std::str;
use std::char;
use std::io;
use std::os;
use serialize::{json, Encodable};

mod acorn;
mod types;

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
    let args:Vec<String> = os::args();

    if args.len() < 2 {
        panic!("Usage: acorn filepath.js OR acorn -")
    }

    // Collect our input.
    let contents = if args[1].as_slice() == "-" {
        io::stdin().read_to_string()
    } else {
        io::File::open(&Path::new(&args[1])).read_to_string()
    }.unwrap();

    let mut a = acorn::AcornParser::new();
    let result = a.parse(&contents.to_string());

    println!("{}", json::encode(&*result));
}
