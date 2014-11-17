use types::*;
use std::mem;
use std::io;
use std;
use std::f64;
use std::fmt;
use serialize::{Encodable, Encoder};
use serialize::json::ToJson;
use serialize::json;


// JS primitives. TODO: simplify, make generic

pub fn toString (arg:&str) -> &str {
    return arg;
}

pub fn fromCharCode (i:u32) -> String {
    return String::from_char(1, std::char::from_u32(i).unwrap());
}

pub fn fromCharCode2 (i:u32, j:u32) -> String {
    return String::from_chars([std::char::from_u32(i).unwrap(), std::char::from_u32(j).unwrap()]);
}

pub fn push (mut nodes:Vec<Box<Node>>, node:Box<Node>) {
    nodes.push(node);
}

pub fn charCodeAt(arg:&str, n:uint) -> int {
    return arg.utf16_units().nth(n).unwrap() as int;
}

pub fn convert_to_Node_C (arg:&mut Box<Node>) -> &'static str {
    return "";
}

pub fn jsparse_callback_close (arg:&str) {
    writeln!(io::stderr(), "close: {}", arg);
}

pub fn ISNULL(arg:&str) -> bool {
    return true;
}

pub fn jsparse_callback_open (arg:&str) {
    writeln!(io::stderr(), "open: {}", arg);
}

pub fn raise (start:int, message:&str){
    // printf("ERROR: %s %d\n", message.c_str(), start);
    panic!("At char {} message {}", start, message);
    // exit(1);
}

pub fn indexOf(arg:&str, needle:&str, start:int) -> int {
    return -1;
}

pub fn setOptions (arg:options_t) {
}

pub fn isUseStrict (arg:&mut Box<Node>) -> bool {
    false
}

pub fn toAssignable(node:Box<Node>, a:bool, b:bool) -> Box<Node>
{
    node
}

pub fn slice (value:&str, start:int, end:int) -> &str {
    // TODO check negative indices
    value.slice_chars(start as uint, end as uint)
}

pub fn exec (regex: |arg:&str| -> bool, val:&str ) -> Option<Box<Vec<String>>> {
    if regex(val) {
        Some(box vec![val.to_string()])
    } else {
        None
    }
}

pub fn ISNOTNULL (arg:int) -> bool {
    return false;
}

pub fn isNaN (arg:f64) -> bool {
    return arg.is_nan();
}

pub fn parseInt (arg:&str, base:int) -> i32 {
    return from_str::<i32>(arg).unwrap();
}

pub fn parseFloat (arg:&str) -> f64 {
    return from_str::<f64>(arg).unwrap();
}

pub fn test (regex: |arg:&str| -> bool, val:&str ) -> bool {
    return regex(val);
}

pub fn checkLVal(arg:&Box<Node>) {

}

pub fn parseTemplate() -> Box<Node> {
    return nullptr.clone()
}

pub struct thisval {
    pub line:uint,
    pub column:uint,
}

pub static THIS:thisval = thisval { line: 0, column: 0 };

pub fn charAt(arg:&str, n:uint) -> String {
    return arg.slice_chars(n, n+1).to_string();
}

pub fn fakeregexp (arg:&str) -> bool {
    return false;
}

pub fn RegExp(content:&str, mods:&str) -> String {
    return content.to_string();
}

pub static NaN:f64 = f64::NAN;
pub static Infinity:f64 = f64::INFINITY;

// Regexes and string matching

fn predicate_0(arg:&str) -> bool { return arg == "abstract" || arg == "boolean" || arg == "byte" || arg == "char" || arg == "class" || arg == "double" || arg == "enum" || arg == "export" || arg == "extends" || arg == "final" || arg == "float" || arg == "goto" || arg == "implements" || arg == "import" || arg == "int" || arg == "interface" || arg == "long" || arg == "native" || arg == "package" || arg == "private" || arg == "protected" || arg == "public" || arg == "short" || arg == "static" || arg == "super" || arg == "synchronized" || arg == "throws" || arg == "transient" || arg == "volatile"; }
fn predicate_1(arg:&str) -> bool { return arg == "class" || arg == "enum" || arg == "extends" || arg == "super" || arg == "const" || arg == "export" || arg == "import"; }
fn predicate_2(arg:&str) -> bool { return arg == "implements" || arg == "interface" || arg == "let" || arg == "package" || arg == "private" || arg == "protected" || arg == "public" || arg == "static" || arg == "yield"; }
fn predicate_3(arg:&str) -> bool { return arg == "eval" || arg == "arguments"; }
fn predicate_4(arg:&str) -> bool { return arg == "break" || arg == "case" || arg == "catch" || arg == "continue" || arg == "debugger" || arg == "default" || arg == "do" || arg == "else" || arg == "finally" || arg == "for" || arg == "function" || arg == "if" || arg == "return" || arg == "switch" || arg == "throw" || arg == "try" || arg == "var" || arg == "while" || arg == "with" || arg == "null" || arg == "true" || arg == "false" || arg == "instanceof" || arg == "typeof" || arg == "void" || arg == "delete" || arg == "new" || arg == "in" || arg == "this"; }
fn predicate_5(arg:&str) -> bool { return arg == "break" || arg == "case" || arg == "catch" || arg == "continue" || arg == "debugger" || arg == "default" || arg == "do" || arg == "else" || arg == "finally" || arg == "for" || arg == "function" || arg == "if" || arg == "return" || arg == "switch" || arg == "throw" || arg == "try" || arg == "var" || arg == "while" || arg == "with" || arg == "null" || arg == "true" || arg == "false" || arg == "instanceof" || arg == "typeof" || arg == "void" || arg == "delete" || arg == "new" || arg == "in" || arg == "this" || arg == "let" || arg == "const" || arg == "class" || arg == "extends" || arg == "export" || arg == "import" || arg == "yield"; }
fn regex_6 (arg:&str) -> bool { for c in arg.utf16_units() { match c {  0x1680 | 0x180e | 0x2000 ... 0x200a | 0x202f | 0x205f | 0x3000 | 0xfeff => { return true; }, _ => { break;  } } }  return false; }
fn regex_7 (arg:&str) -> bool { for c in arg.utf16_units() { match c {  0xaa | 0xb5 | 0xba | 0xc0 ... 0xd6 | 0xd8 ... 0xf6 | 0xf8 ... 0x2c1 | 0x2c6 ... 0x2d1 | 0x2e0 ... 0x2e4 | 0x2ec | 0x2ee | 0x370 ... 0x374 | 0x376 | 0x377 | 0x37a ... 0x37d | 0x37f | 0x386 | 0x388 ... 0x38a | 0x38c | 0x38e ... 0x3a1 | 0x3a3 ... 0x3f5 | 0x3f7 ... 0x481 | 0x48a ... 0x52f | 0x531 ... 0x556 | 0x559 | 0x561 ... 0x587 | 0x5d0 ... 0x5ea | 0x5f0 ... 0x5f2 | 0x620 ... 0x64a | 0x66e | 0x66f | 0x671 ... 0x6d3 | 0x6d5 | 0x6e5 | 0x6e6 | 0x6ee | 0x6ef | 0x6fa ... 0x6fc | 0x6ff | 0x710 | 0x712 ... 0x72f | 0x74d ... 0x7a5 | 0x7b1 | 0x7ca ... 0x7ea | 0x7f4 | 0x7f5 | 0x7fa | 0x800 ... 0x815 | 0x81a | 0x824 | 0x828 | 0x840 ... 0x858 | 0x8a0 ... 0x8b2 | 0x904 ... 0x939 | 0x93d | 0x950 | 0x958 ... 0x961 | 0x971 ... 0x980 | 0x985 ... 0x98c | 0x98f | 0x990 | 0x993 ... 0x9a8 | 0x9aa ... 0x9b0 | 0x9b2 | 0x9b6 ... 0x9b9 | 0x9bd | 0x9ce | 0x9dc | 0x9dd | 0x9df ... 0x9e1 | 0x9f0 | 0x9f1 | 0xa05 ... 0xa0a | 0xa0f | 0xa10 | 0xa13 ... 0xa28 | 0xa2a ... 0xa30 | 0xa32 | 0xa33 | 0xa35 | 0xa36 | 0xa38 | 0xa39 | 0xa59 ... 0xa5c | 0xa5e | 0xa72 ... 0xa74 | 0xa85 ... 0xa8d | 0xa8f ... 0xa91 | 0xa93 ... 0xaa8 | 0xaaa ... 0xab0 | 0xab2 | 0xab3 | 0xab5 ... 0xab9 | 0xabd | 0xad0 | 0xae0 | 0xae1 | 0xb05 ... 0xb0c | 0xb0f | 0xb10 | 0xb13 ... 0xb28 | 0xb2a ... 0xb30 | 0xb32 | 0xb33 | 0xb35 ... 0xb39 | 0xb3d | 0xb5c | 0xb5d | 0xb5f ... 0xb61 | 0xb71 | 0xb83 | 0xb85 ... 0xb8a | 0xb8e ... 0xb90 | 0xb92 ... 0xb95 | 0xb99 | 0xb9a | 0xb9c | 0xb9e | 0xb9f | 0xba3 | 0xba4 | 0xba8 ... 0xbaa | 0xbae ... 0xbb9 | 0xbd0 | 0xc05 ... 0xc0c | 0xc0e ... 0xc10 | 0xc12 ... 0xc28 | 0xc2a ... 0xc39 | 0xc3d | 0xc58 | 0xc59 | 0xc60 | 0xc61 | 0xc85 ... 0xc8c | 0xc8e ... 0xc90 | 0xc92 ... 0xca8 | 0xcaa ... 0xcb3 | 0xcb5 ... 0xcb9 | 0xcbd | 0xcde | 0xce0 | 0xce1 | 0xcf1 | 0xcf2 | 0xd05 ... 0xd0c | 0xd0e ... 0xd10 | 0xd12 ... 0xd3a | 0xd3d | 0xd4e | 0xd60 | 0xd61 | 0xd7a ... 0xd7f | 0xd85 ... 0xd96 | 0xd9a ... 0xdb1 | 0xdb3 ... 0xdbb | 0xdbd | 0xdc0 ... 0xdc6 | 0xe01 ... 0xe30 | 0xe32 | 0xe33 | 0xe40 ... 0xe46 | 0xe81 | 0xe82 | 0xe84 | 0xe87 | 0xe88 | 0xe8a | 0xe8d | 0xe94 ... 0xe97 | 0xe99 ... 0xe9f | 0xea1 ... 0xea3 | 0xea5 | 0xea7 | 0xeaa | 0xeab | 0xead ... 0xeb0 | 0xeb2 | 0xeb3 | 0xebd | 0xec0 ... 0xec4 | 0xec6 | 0xedc ... 0xedf | 0xf00 | 0xf40 ... 0xf47 | 0xf49 ... 0xf6c | 0xf88 ... 0xf8c | 0x1000 ... 0x102a | 0x103f | 0x1050 ... 0x1055 | 0x105a ... 0x105d | 0x1061 | 0x1065 | 0x1066 | 0x106e ... 0x1070 | 0x1075 ... 0x1081 | 0x108e | 0x10a0 ... 0x10c5 | 0x10c7 | 0x10cd | 0x10d0 ... 0x10fa | 0x10fc ... 0x1248 | 0x124a ... 0x124d | 0x1250 ... 0x1256 | 0x1258 | 0x125a ... 0x125d | 0x1260 ... 0x1288 | 0x128a ... 0x128d | 0x1290 ... 0x12b0 | 0x12b2 ... 0x12b5 | 0x12b8 ... 0x12be | 0x12c0 | 0x12c2 ... 0x12c5 | 0x12c8 ... 0x12d6 | 0x12d8 ... 0x1310 | 0x1312 ... 0x1315 | 0x1318 ... 0x135a | 0x1380 ... 0x138f | 0x13a0 ... 0x13f4 | 0x1401 ... 0x166c | 0x166f ... 0x167f | 0x1681 ... 0x169a | 0x16a0 ... 0x16ea | 0x16ee ... 0x16f8 | 0x1700 ... 0x170c | 0x170e ... 0x1711 | 0x1720 ... 0x1731 | 0x1740 ... 0x1751 | 0x1760 ... 0x176c | 0x176e ... 0x1770 | 0x1780 ... 0x17b3 | 0x17d7 | 0x17dc | 0x1820 ... 0x1877 | 0x1880 ... 0x18a8 | 0x18aa | 0x18b0 ... 0x18f5 | 0x1900 ... 0x191e | 0x1950 ... 0x196d | 0x1970 ... 0x1974 | 0x1980 ... 0x19ab | 0x19c1 ... 0x19c7 | 0x1a00 ... 0x1a16 | 0x1a20 ... 0x1a54 | 0x1aa7 | 0x1b05 ... 0x1b33 | 0x1b45 ... 0x1b4b | 0x1b83 ... 0x1ba0 | 0x1bae | 0x1baf | 0x1bba ... 0x1be5 | 0x1c00 ... 0x1c23 | 0x1c4d ... 0x1c4f | 0x1c5a ... 0x1c7d | 0x1ce9 ... 0x1cec | 0x1cee ... 0x1cf1 | 0x1cf5 | 0x1cf6 | 0x1d00 ... 0x1dbf | 0x1e00 ... 0x1f15 | 0x1f18 ... 0x1f1d | 0x1f20 ... 0x1f45 | 0x1f48 ... 0x1f4d | 0x1f50 ... 0x1f57 | 0x1f59 | 0x1f5b | 0x1f5d | 0x1f5f ... 0x1f7d | 0x1f80 ... 0x1fb4 | 0x1fb6 ... 0x1fbc | 0x1fbe | 0x1fc2 ... 0x1fc4 | 0x1fc6 ... 0x1fcc | 0x1fd0 ... 0x1fd3 | 0x1fd6 ... 0x1fdb | 0x1fe0 ... 0x1fec | 0x1ff2 ... 0x1ff4 | 0x1ff6 ... 0x1ffc | 0x2071 | 0x207f | 0x2090 ... 0x209c | 0x2102 | 0x2107 | 0x210a ... 0x2113 | 0x2115 | 0x2119 ... 0x211d | 0x2124 | 0x2126 | 0x2128 | 0x212a ... 0x212d | 0x212f ... 0x2139 | 0x213c ... 0x213f | 0x2145 ... 0x2149 | 0x214e | 0x2160 ... 0x2188 | 0x2c00 ... 0x2c2e | 0x2c30 ... 0x2c5e | 0x2c60 ... 0x2ce4 | 0x2ceb ... 0x2cee | 0x2cf2 | 0x2cf3 | 0x2d00 ... 0x2d25 | 0x2d27 | 0x2d2d | 0x2d30 ... 0x2d67 | 0x2d6f | 0x2d80 ... 0x2d96 | 0x2da0 ... 0x2da6 | 0x2da8 ... 0x2dae | 0x2db0 ... 0x2db6 | 0x2db8 ... 0x2dbe | 0x2dc0 ... 0x2dc6 | 0x2dc8 ... 0x2dce | 0x2dd0 ... 0x2dd6 | 0x2dd8 ... 0x2dde | 0x2e2f | 0x3005 ... 0x3007 | 0x3021 ... 0x3029 | 0x3031 ... 0x3035 | 0x3038 ... 0x303c | 0x3041 ... 0x3096 | 0x309d ... 0x309f | 0x30a1 ... 0x30fa | 0x30fc ... 0x30ff | 0x3105 ... 0x312d | 0x3131 ... 0x318e | 0x31a0 ... 0x31ba | 0x31f0 ... 0x31ff | 0x3400 ... 0x4db5 | 0x4e00 ... 0x9fcc | 0xa000 ... 0xa48c | 0xa4d0 ... 0xa4fd | 0xa500 ... 0xa60c | 0xa610 ... 0xa61f | 0xa62a | 0xa62b | 0xa640 ... 0xa66e | 0xa67f ... 0xa69d | 0xa6a0 ... 0xa6ef | 0xa717 ... 0xa71f | 0xa722 ... 0xa788 | 0xa78b ... 0xa78e | 0xa790 ... 0xa7ad | 0xa7b0 | 0xa7b1 | 0xa7f7 ... 0xa801 | 0xa803 ... 0xa805 | 0xa807 ... 0xa80a | 0xa80c ... 0xa822 | 0xa840 ... 0xa873 | 0xa882 ... 0xa8b3 | 0xa8f2 ... 0xa8f7 | 0xa8fb | 0xa90a ... 0xa925 | 0xa930 ... 0xa946 | 0xa960 ... 0xa97c | 0xa984 ... 0xa9b2 | 0xa9cf | 0xa9e0 ... 0xa9e4 | 0xa9e6 ... 0xa9ef | 0xa9fa ... 0xa9fe | 0xaa00 ... 0xaa28 | 0xaa40 ... 0xaa42 | 0xaa44 ... 0xaa4b | 0xaa60 ... 0xaa76 | 0xaa7a | 0xaa7e ... 0xaaaf | 0xaab1 | 0xaab5 | 0xaab6 | 0xaab9 ... 0xaabd | 0xaac0 | 0xaac2 | 0xaadb ... 0xaadd | 0xaae0 ... 0xaaea | 0xaaf2 ... 0xaaf4 | 0xab01 ... 0xab06 | 0xab09 ... 0xab0e | 0xab11 ... 0xab16 | 0xab20 ... 0xab26 | 0xab28 ... 0xab2e | 0xab30 ... 0xab5a | 0xab5c ... 0xab5f | 0xab64 | 0xab65 | 0xabc0 ... 0xabe2 | 0xac00 ... 0xd7a3 | 0xd7b0 ... 0xd7c6 | 0xd7cb ... 0xd7fb | 0xf900 ... 0xfa6d | 0xfa70 ... 0xfad9 | 0xfb00 ... 0xfb06 | 0xfb13 ... 0xfb17 | 0xfb1d | 0xfb1f ... 0xfb28 | 0xfb2a ... 0xfb36 | 0xfb38 ... 0xfb3c | 0xfb3e | 0xfb40 | 0xfb41 | 0xfb43 | 0xfb44 | 0xfb46 ... 0xfbb1 | 0xfbd3 ... 0xfd3d | 0xfd50 ... 0xfd8f | 0xfd92 ... 0xfdc7 | 0xfdf0 ... 0xfdfb | 0xfe70 ... 0xfe74 | 0xfe76 ... 0xfefc | 0xff21 ... 0xff3a | 0xff41 ... 0xff5a | 0xff66 ... 0xffbe | 0xffc2 ... 0xffc7 | 0xffca ... 0xffcf | 0xffd2 ... 0xffd7 | 0xffda ... 0xffdc => { return true; }, _ => { break;  } } }  return false; }
fn regex_8 (arg:&str) -> bool { for c in arg.utf16_units() { match c {  0xaa | 0xb5 | 0xba | 0xc0 ... 0xd6 | 0xd8 ... 0xf6 | 0xf8 ... 0x2c1 | 0x2c6 ... 0x2d1 | 0x2e0 ... 0x2e4 | 0x2ec | 0x2ee | 0x370 ... 0x374 | 0x376 | 0x377 | 0x37a ... 0x37d | 0x37f | 0x386 | 0x388 ... 0x38a | 0x38c | 0x38e ... 0x3a1 | 0x3a3 ... 0x3f5 | 0x3f7 ... 0x481 | 0x48a ... 0x52f | 0x531 ... 0x556 | 0x559 | 0x561 ... 0x587 | 0x5d0 ... 0x5ea | 0x5f0 ... 0x5f2 | 0x620 ... 0x64a | 0x66e | 0x66f | 0x671 ... 0x6d3 | 0x6d5 | 0x6e5 | 0x6e6 | 0x6ee | 0x6ef | 0x6fa ... 0x6fc | 0x6ff | 0x710 | 0x712 ... 0x72f | 0x74d ... 0x7a5 | 0x7b1 | 0x7ca ... 0x7ea | 0x7f4 | 0x7f5 | 0x7fa | 0x800 ... 0x815 | 0x81a | 0x824 | 0x828 | 0x840 ... 0x858 | 0x8a0 ... 0x8b2 | 0x904 ... 0x939 | 0x93d | 0x950 | 0x958 ... 0x961 | 0x971 ... 0x980 | 0x985 ... 0x98c | 0x98f | 0x990 | 0x993 ... 0x9a8 | 0x9aa ... 0x9b0 | 0x9b2 | 0x9b6 ... 0x9b9 | 0x9bd | 0x9ce | 0x9dc | 0x9dd | 0x9df ... 0x9e1 | 0x9f0 | 0x9f1 | 0xa05 ... 0xa0a | 0xa0f | 0xa10 | 0xa13 ... 0xa28 | 0xa2a ... 0xa30 | 0xa32 | 0xa33 | 0xa35 | 0xa36 | 0xa38 | 0xa39 | 0xa59 ... 0xa5c | 0xa5e | 0xa72 ... 0xa74 | 0xa85 ... 0xa8d | 0xa8f ... 0xa91 | 0xa93 ... 0xaa8 | 0xaaa ... 0xab0 | 0xab2 | 0xab3 | 0xab5 ... 0xab9 | 0xabd | 0xad0 | 0xae0 | 0xae1 | 0xb05 ... 0xb0c | 0xb0f | 0xb10 | 0xb13 ... 0xb28 | 0xb2a ... 0xb30 | 0xb32 | 0xb33 | 0xb35 ... 0xb39 | 0xb3d | 0xb5c | 0xb5d | 0xb5f ... 0xb61 | 0xb71 | 0xb83 | 0xb85 ... 0xb8a | 0xb8e ... 0xb90 | 0xb92 ... 0xb95 | 0xb99 | 0xb9a | 0xb9c | 0xb9e | 0xb9f | 0xba3 | 0xba4 | 0xba8 ... 0xbaa | 0xbae ... 0xbb9 | 0xbd0 | 0xc05 ... 0xc0c | 0xc0e ... 0xc10 | 0xc12 ... 0xc28 | 0xc2a ... 0xc39 | 0xc3d | 0xc58 | 0xc59 | 0xc60 | 0xc61 | 0xc85 ... 0xc8c | 0xc8e ... 0xc90 | 0xc92 ... 0xca8 | 0xcaa ... 0xcb3 | 0xcb5 ... 0xcb9 | 0xcbd | 0xcde | 0xce0 | 0xce1 | 0xcf1 | 0xcf2 | 0xd05 ... 0xd0c | 0xd0e ... 0xd10 | 0xd12 ... 0xd3a | 0xd3d | 0xd4e | 0xd60 | 0xd61 | 0xd7a ... 0xd7f | 0xd85 ... 0xd96 | 0xd9a ... 0xdb1 | 0xdb3 ... 0xdbb | 0xdbd | 0xdc0 ... 0xdc6 | 0xe01 ... 0xe30 | 0xe32 | 0xe33 | 0xe40 ... 0xe46 | 0xe81 | 0xe82 | 0xe84 | 0xe87 | 0xe88 | 0xe8a | 0xe8d | 0xe94 ... 0xe97 | 0xe99 ... 0xe9f | 0xea1 ... 0xea3 | 0xea5 | 0xea7 | 0xeaa | 0xeab | 0xead ... 0xeb0 | 0xeb2 | 0xeb3 | 0xebd | 0xec0 ... 0xec4 | 0xec6 | 0xedc ... 0xedf | 0xf00 | 0xf40 ... 0xf47 | 0xf49 ... 0xf6c | 0xf88 ... 0xf8c | 0x1000 ... 0x102a | 0x103f | 0x1050 ... 0x1055 | 0x105a ... 0x105d | 0x1061 | 0x1065 | 0x1066 | 0x106e ... 0x1070 | 0x1075 ... 0x1081 | 0x108e | 0x10a0 ... 0x10c5 | 0x10c7 | 0x10cd | 0x10d0 ... 0x10fa | 0x10fc ... 0x1248 | 0x124a ... 0x124d | 0x1250 ... 0x1256 | 0x1258 | 0x125a ... 0x125d | 0x1260 ... 0x1288 | 0x128a ... 0x128d | 0x1290 ... 0x12b0 | 0x12b2 ... 0x12b5 | 0x12b8 ... 0x12be | 0x12c0 | 0x12c2 ... 0x12c5 | 0x12c8 ... 0x12d6 | 0x12d8 ... 0x1310 | 0x1312 ... 0x1315 | 0x1318 ... 0x135a | 0x1380 ... 0x138f | 0x13a0 ... 0x13f4 | 0x1401 ... 0x166c | 0x166f ... 0x167f | 0x1681 ... 0x169a | 0x16a0 ... 0x16ea | 0x16ee ... 0x16f8 | 0x1700 ... 0x170c | 0x170e ... 0x1711 | 0x1720 ... 0x1731 | 0x1740 ... 0x1751 | 0x1760 ... 0x176c | 0x176e ... 0x1770 | 0x1780 ... 0x17b3 | 0x17d7 | 0x17dc | 0x1820 ... 0x1877 | 0x1880 ... 0x18a8 | 0x18aa | 0x18b0 ... 0x18f5 | 0x1900 ... 0x191e | 0x1950 ... 0x196d | 0x1970 ... 0x1974 | 0x1980 ... 0x19ab | 0x19c1 ... 0x19c7 | 0x1a00 ... 0x1a16 | 0x1a20 ... 0x1a54 | 0x1aa7 | 0x1b05 ... 0x1b33 | 0x1b45 ... 0x1b4b | 0x1b83 ... 0x1ba0 | 0x1bae | 0x1baf | 0x1bba ... 0x1be5 | 0x1c00 ... 0x1c23 | 0x1c4d ... 0x1c4f | 0x1c5a ... 0x1c7d | 0x1ce9 ... 0x1cec | 0x1cee ... 0x1cf1 | 0x1cf5 | 0x1cf6 | 0x1d00 ... 0x1dbf | 0x1e00 ... 0x1f15 | 0x1f18 ... 0x1f1d | 0x1f20 ... 0x1f45 | 0x1f48 ... 0x1f4d | 0x1f50 ... 0x1f57 | 0x1f59 | 0x1f5b | 0x1f5d | 0x1f5f ... 0x1f7d | 0x1f80 ... 0x1fb4 | 0x1fb6 ... 0x1fbc | 0x1fbe | 0x1fc2 ... 0x1fc4 | 0x1fc6 ... 0x1fcc | 0x1fd0 ... 0x1fd3 | 0x1fd6 ... 0x1fdb | 0x1fe0 ... 0x1fec | 0x1ff2 ... 0x1ff4 | 0x1ff6 ... 0x1ffc | 0x2071 | 0x207f | 0x2090 ... 0x209c | 0x2102 | 0x2107 | 0x210a ... 0x2113 | 0x2115 | 0x2119 ... 0x211d | 0x2124 | 0x2126 | 0x2128 | 0x212a ... 0x212d | 0x212f ... 0x2139 | 0x213c ... 0x213f | 0x2145 ... 0x2149 | 0x214e | 0x2160 ... 0x2188 | 0x2c00 ... 0x2c2e | 0x2c30 ... 0x2c5e | 0x2c60 ... 0x2ce4 | 0x2ceb ... 0x2cee | 0x2cf2 | 0x2cf3 | 0x2d00 ... 0x2d25 | 0x2d27 | 0x2d2d | 0x2d30 ... 0x2d67 | 0x2d6f | 0x2d80 ... 0x2d96 | 0x2da0 ... 0x2da6 | 0x2da8 ... 0x2dae | 0x2db0 ... 0x2db6 | 0x2db8 ... 0x2dbe | 0x2dc0 ... 0x2dc6 | 0x2dc8 ... 0x2dce | 0x2dd0 ... 0x2dd6 | 0x2dd8 ... 0x2dde | 0x2e2f | 0x3005 ... 0x3007 | 0x3021 ... 0x3029 | 0x3031 ... 0x3035 | 0x3038 ... 0x303c | 0x3041 ... 0x3096 | 0x309d ... 0x309f | 0x30a1 ... 0x30fa | 0x30fc ... 0x30ff | 0x3105 ... 0x312d | 0x3131 ... 0x318e | 0x31a0 ... 0x31ba | 0x31f0 ... 0x31ff | 0x3400 ... 0x4db5 | 0x4e00 ... 0x9fcc | 0xa000 ... 0xa48c | 0xa4d0 ... 0xa4fd | 0xa500 ... 0xa60c | 0xa610 ... 0xa61f | 0xa62a | 0xa62b | 0xa640 ... 0xa66e | 0xa67f ... 0xa69d | 0xa6a0 ... 0xa6ef | 0xa717 ... 0xa71f | 0xa722 ... 0xa788 | 0xa78b ... 0xa78e | 0xa790 ... 0xa7ad | 0xa7b0 | 0xa7b1 | 0xa7f7 ... 0xa801 | 0xa803 ... 0xa805 | 0xa807 ... 0xa80a | 0xa80c ... 0xa822 | 0xa840 ... 0xa873 | 0xa882 ... 0xa8b3 | 0xa8f2 ... 0xa8f7 | 0xa8fb | 0xa90a ... 0xa925 | 0xa930 ... 0xa946 | 0xa960 ... 0xa97c | 0xa984 ... 0xa9b2 | 0xa9cf | 0xa9e0 ... 0xa9e4 | 0xa9e6 ... 0xa9ef | 0xa9fa ... 0xa9fe | 0xaa00 ... 0xaa28 | 0xaa40 ... 0xaa42 | 0xaa44 ... 0xaa4b | 0xaa60 ... 0xaa76 | 0xaa7a | 0xaa7e ... 0xaaaf | 0xaab1 | 0xaab5 | 0xaab6 | 0xaab9 ... 0xaabd | 0xaac0 | 0xaac2 | 0xaadb ... 0xaadd | 0xaae0 ... 0xaaea | 0xaaf2 ... 0xaaf4 | 0xab01 ... 0xab06 | 0xab09 ... 0xab0e | 0xab11 ... 0xab16 | 0xab20 ... 0xab26 | 0xab28 ... 0xab2e | 0xab30 ... 0xab5a | 0xab5c ... 0xab5f | 0xab64 | 0xab65 | 0xabc0 ... 0xabe2 | 0xac00 ... 0xd7a3 | 0xd7b0 ... 0xd7c6 | 0xd7cb ... 0xd7fb | 0xf900 ... 0xfa6d | 0xfa70 ... 0xfad9 | 0xfb00 ... 0xfb06 | 0xfb13 ... 0xfb17 | 0xfb1d | 0xfb1f ... 0xfb28 | 0xfb2a ... 0xfb36 | 0xfb38 ... 0xfb3c | 0xfb3e | 0xfb40 | 0xfb41 | 0xfb43 | 0xfb44 | 0xfb46 ... 0xfbb1 | 0xfbd3 ... 0xfd3d | 0xfd50 ... 0xfd8f | 0xfd92 ... 0xfdc7 | 0xfdf0 ... 0xfdfb | 0xfe70 ... 0xfe74 | 0xfe76 ... 0xfefc | 0xff21 ... 0xff3a | 0xff41 ... 0xff5a | 0xff66 ... 0xffbe | 0xffc2 ... 0xffc7 | 0xffca ... 0xffcf | 0xffd2 ... 0xffd7 | 0xffda ... 0xffdc | 0x300 ... 0x36f | 0x483 ... 0x487 | 0x591 ... 0x5bd | 0x5bf | 0x5c1 | 0x5c2 | 0x5c4 | 0x5c5 | 0x5c7 | 0x610 ... 0x61a | 0x64b ... 0x669 | 0x670 | 0x6d6 ... 0x6dc | 0x6df ... 0x6e4 | 0x6e7 | 0x6e8 | 0x6ea ... 0x6ed | 0x6f0 ... 0x6f9 | 0x711 | 0x730 ... 0x74a | 0x7a6 ... 0x7b0 | 0x7c0 ... 0x7c9 | 0x7eb ... 0x7f3 | 0x816 ... 0x819 | 0x81b ... 0x823 | 0x825 ... 0x827 | 0x829 ... 0x82d | 0x859 ... 0x85b | 0x8e4 ... 0x903 | 0x93a ... 0x93c | 0x93e ... 0x94f | 0x951 ... 0x957 | 0x962 | 0x963 | 0x966 ... 0x96f | 0x981 ... 0x983 | 0x9bc | 0x9be ... 0x9c4 | 0x9c7 | 0x9c8 | 0x9cb ... 0x9cd | 0x9d7 | 0x9e2 | 0x9e3 | 0x9e6 ... 0x9ef | 0xa01 ... 0xa03 | 0xa3c | 0xa3e ... 0xa42 | 0xa47 | 0xa48 | 0xa4b ... 0xa4d | 0xa51 | 0xa66 ... 0xa71 | 0xa75 | 0xa81 ... 0xa83 | 0xabc | 0xabe ... 0xac5 | 0xac7 ... 0xac9 | 0xacb ... 0xacd | 0xae2 | 0xae3 | 0xae6 ... 0xaef | 0xb01 ... 0xb03 | 0xb3c | 0xb3e ... 0xb44 | 0xb47 | 0xb48 | 0xb4b ... 0xb4d | 0xb56 | 0xb57 | 0xb62 | 0xb63 | 0xb66 ... 0xb6f | 0xb82 | 0xbbe ... 0xbc2 | 0xbc6 ... 0xbc8 | 0xbca ... 0xbcd | 0xbd7 | 0xbe6 ... 0xbef | 0xc00 ... 0xc03 | 0xc3e ... 0xc44 | 0xc46 ... 0xc48 | 0xc4a ... 0xc4d | 0xc55 | 0xc56 | 0xc62 | 0xc63 | 0xc66 ... 0xc6f | 0xc81 ... 0xc83 | 0xcbc | 0xcbe ... 0xcc4 | 0xcc6 ... 0xcc8 | 0xcca ... 0xccd | 0xcd5 | 0xcd6 | 0xce2 | 0xce3 | 0xce6 ... 0xcef | 0xd01 ... 0xd03 | 0xd3e ... 0xd44 | 0xd46 ... 0xd48 | 0xd4a ... 0xd4d | 0xd57 | 0xd62 | 0xd63 | 0xd66 ... 0xd6f | 0xd82 | 0xd83 | 0xdca | 0xdcf ... 0xdd4 | 0xdd6 | 0xdd8 ... 0xddf | 0xde6 ... 0xdef | 0xdf2 | 0xdf3 | 0xe31 | 0xe34 ... 0xe3a | 0xe47 ... 0xe4e | 0xe50 ... 0xe59 | 0xeb1 | 0xeb4 ... 0xeb9 | 0xebb | 0xebc | 0xec8 ... 0xecd | 0xed0 ... 0xed9 | 0xf18 | 0xf19 | 0xf20 ... 0xf29 | 0xf35 | 0xf37 | 0xf39 | 0xf3e | 0xf3f | 0xf71 ... 0xf84 | 0xf86 | 0xf87 | 0xf8d ... 0xf97 | 0xf99 ... 0xfbc | 0xfc6 | 0x102b ... 0x103e | 0x1040 ... 0x1049 | 0x1056 ... 0x1059 | 0x105e ... 0x1060 | 0x1062 ... 0x1064 | 0x1067 ... 0x106d | 0x1071 ... 0x1074 | 0x1082 ... 0x108d | 0x108f ... 0x109d | 0x135d ... 0x135f | 0x1712 ... 0x1714 | 0x1732 ... 0x1734 | 0x1752 | 0x1753 | 0x1772 | 0x1773 | 0x17b4 ... 0x17d3 | 0x17dd | 0x17e0 ... 0x17e9 | 0x180b ... 0x180d | 0x1810 ... 0x1819 | 0x18a9 | 0x1920 ... 0x192b | 0x1930 ... 0x193b | 0x1946 ... 0x194f | 0x19b0 ... 0x19c0 | 0x19c8 | 0x19c9 | 0x19d0 ... 0x19d9 | 0x1a17 ... 0x1a1b | 0x1a55 ... 0x1a5e | 0x1a60 ... 0x1a7c | 0x1a7f ... 0x1a89 | 0x1a90 ... 0x1a99 | 0x1ab0 ... 0x1abd | 0x1b00 ... 0x1b04 | 0x1b34 ... 0x1b44 | 0x1b50 ... 0x1b59 | 0x1b6b ... 0x1b73 | 0x1b80 ... 0x1b82 | 0x1ba1 ... 0x1bad | 0x1bb0 ... 0x1bb9 | 0x1be6 ... 0x1bf3 | 0x1c24 ... 0x1c37 | 0x1c40 ... 0x1c49 | 0x1c50 ... 0x1c59 | 0x1cd0 ... 0x1cd2 | 0x1cd4 ... 0x1ce8 | 0x1ced | 0x1cf2 ... 0x1cf4 | 0x1cf8 | 0x1cf9 | 0x1dc0 ... 0x1df5 | 0x1dfc ... 0x1dff | 0x200c | 0x200d | 0x203f | 0x2040 | 0x2054 | 0x20d0 ... 0x20dc | 0x20e1 | 0x20e5 ... 0x20f0 | 0x2cef ... 0x2cf1 | 0x2d7f | 0x2de0 ... 0x2dff | 0x302a ... 0x302f | 0x3099 | 0x309a | 0xa620 ... 0xa629 | 0xa66f | 0xa674 ... 0xa67d | 0xa69f | 0xa6f0 | 0xa6f1 | 0xa802 | 0xa806 | 0xa80b | 0xa823 ... 0xa827 | 0xa880 | 0xa881 | 0xa8b4 ... 0xa8c4 | 0xa8d0 ... 0xa8d9 | 0xa8e0 ... 0xa8f1 | 0xa900 ... 0xa909 | 0xa926 ... 0xa92d | 0xa947 ... 0xa953 | 0xa980 ... 0xa983 | 0xa9b3 ... 0xa9c0 | 0xa9d0 ... 0xa9d9 | 0xa9e5 | 0xa9f0 ... 0xa9f9 | 0xaa29 ... 0xaa36 | 0xaa43 | 0xaa4c | 0xaa4d | 0xaa50 ... 0xaa59 | 0xaa7b ... 0xaa7d | 0xaab0 | 0xaab2 ... 0xaab4 | 0xaab7 | 0xaab8 | 0xaabe | 0xaabf | 0xaac1 | 0xaaeb ... 0xaaef | 0xaaf5 | 0xaaf6 | 0xabe3 ... 0xabea | 0xabec | 0xabed | 0xabf0 ... 0xabf9 | 0xfb1e | 0xfe00 ... 0xfe0f | 0xfe20 ... 0xfe2d | 0xfe33 | 0xfe34 | 0xfe4d ... 0xfe4f | 0xff10 ... 0xff19 | 0xff3f => { return true; }, _ => { break;  } } }  return false; }
fn regex_9 (arg:&str) -> bool { for c in arg.utf16_units() { match c {  0xa | 0xd | 0x2028 | 0x2029 => { return true; }, _ => { break;  } } }  return false; }
fn regex_10 (arg:&str) -> bool { for c in arg.utf16_units() { match c {  0x72 | 0xa | 0x7c | 0x5b | 0xd | 0x2028 | 0x2029 => { return true; }, _ => { break;  } } }  return false; }
fn regex_11 (arg:&str) -> bool { for c in arg.utf16_units() { match c {  0x67 | 0x6d | 0x73 | 0x69 | 0x79 => { break; }, _ => { return false;  } } }  return true; }
fn regex_12 (arg:&str) -> bool { for c in arg.utf16_units() { match c {  0x38 | 0x39 => { return true; }, _ => { break;  } } }  return false; }
fn regex_13 (arg:&str) -> bool { for c in arg.utf16_units() { match c {  0x30 ... 0x37 => { return true; }, _ => { return false;  } } }  return false; }

const version:&'static str = "0.6.1";

pub struct AcornParser {
    options:options_t,
    input:String,
    sourceFile:String,

    tokVal:Option<js_any_type>,
    labels:Vec<label_t>,
    tokType:Option<keyword_t>,
    tokPos:uint,
    tokStart:int,
    tokEnd:int,
    tokStartLoc:int,
    tokEndLoc:int,
    tokRegexpAllowed:bool,
    tokCurLine:uint,
    tokLineStart:uint,
    lastStart:int,
    lastEnd:int,
    lastEndLoc:int,
    inFunction:bool,
    inGenerator:bool,
    _strict:bool,
    metParenL:int,
    inTemplate:bool,

    containsEsc:bool,
}

lazy_static! {
    static ref empty:Vec<Box<Node>> = vec![]; 

    static ref loopLabel:label_t = label_t {kind: "loop".to_string(), name: "".to_string()};
    static ref switchLabel:label_t = label_t {kind: "switch".to_string(), name: "".to_string()}; 
}


const defaultOptions:options_t = options_t {ecmaVersion: 5, strictSemicolons: false, allowTrailingCommas: true, forbidReserved: "", allowReturnOutsideFunction: false, locations: false, ranges: true, program: None, sourceFile: "", directSourceFile: ""}; 

const _num:keyword_t = keyword_t {_id: 2, atomValue: ATOM_NULL, beforeExpr: false, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "", postfix: false, prefix: false, _type : "num"};  const _regexp:keyword_t = keyword_t {_id: 3, atomValue: ATOM_NULL, beforeExpr: false, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "", postfix: false, prefix: false, _type : "regexp"};  const _string:keyword_t = keyword_t {_id: 4, atomValue: ATOM_NULL, beforeExpr: false, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "", postfix: false, prefix: false, _type : "string"}; 
const _name:keyword_t = keyword_t {_id: 5, atomValue: ATOM_NULL, beforeExpr: false, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "", postfix: false, prefix: false, _type : "name"};  const _eof:keyword_t = keyword_t {_id: 6, atomValue: ATOM_NULL, beforeExpr: false, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "", postfix: false, prefix: false, _type : "eof"}; 
const _break:keyword_t = keyword_t {_id: 7, atomValue: ATOM_NULL, beforeExpr: false, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "break", postfix: false, prefix: false, _type : ""};  const _case:keyword_t = keyword_t {_id: 8, atomValue: ATOM_NULL, beforeExpr: true, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "case", postfix: false, prefix: false, _type : ""};  const _catch:keyword_t = keyword_t {_id: 9, atomValue: ATOM_NULL, beforeExpr: false, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "catch", postfix: false, prefix: false, _type : ""}; 
const _continue:keyword_t = keyword_t {_id: 10, atomValue: ATOM_NULL, beforeExpr: false, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "continue", postfix: false, prefix: false, _type : ""};  const _debugger:keyword_t = keyword_t {_id: 11, atomValue: ATOM_NULL, beforeExpr: false, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "debugger", postfix: false, prefix: false, _type : ""};  const _default:keyword_t = keyword_t {_id: 12, atomValue: ATOM_NULL, beforeExpr: false, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "default", postfix: false, prefix: false, _type : ""}; 
const _do:keyword_t = keyword_t {_id: 13, atomValue: ATOM_NULL, beforeExpr: false, binop: -1, isAssign: false, isLoop: true, isUpdate: false, keyword: "do", postfix: false, prefix: false, _type : ""};  const _else:keyword_t = keyword_t {_id: 14, atomValue: ATOM_NULL, beforeExpr: true, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "else", postfix: false, prefix: false, _type : ""}; 
const _finally:keyword_t = keyword_t {_id: 15, atomValue: ATOM_NULL, beforeExpr: false, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "finally", postfix: false, prefix: false, _type : ""};  const _for:keyword_t = keyword_t {_id: 16, atomValue: ATOM_NULL, beforeExpr: false, binop: -1, isAssign: false, isLoop: true, isUpdate: false, keyword: "for", postfix: false, prefix: false, _type : ""};  const _function:keyword_t = keyword_t {_id: 17, atomValue: ATOM_NULL, beforeExpr: false, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "function", postfix: false, prefix: false, _type : ""}; 
const _if:keyword_t = keyword_t {_id: 18, atomValue: ATOM_NULL, beforeExpr: false, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "if", postfix: false, prefix: false, _type : ""};  const _return:keyword_t = keyword_t {_id: 19, atomValue: ATOM_NULL, beforeExpr: true, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "return", postfix: false, prefix: false, _type : ""};  const _switch:keyword_t = keyword_t {_id: 20, atomValue: ATOM_NULL, beforeExpr: false, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "switch", postfix: false, prefix: false, _type : ""}; 
const _throw:keyword_t = keyword_t {_id: 21, atomValue: ATOM_NULL, beforeExpr: true, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "throw", postfix: false, prefix: false, _type : ""};  const _try:keyword_t = keyword_t {_id: 22, atomValue: ATOM_NULL, beforeExpr: false, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "try", postfix: false, prefix: false, _type : ""};  const _var:keyword_t = keyword_t {_id: 23, atomValue: ATOM_NULL, beforeExpr: false, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "var", postfix: false, prefix: false, _type : ""}; 
const _let:keyword_t = keyword_t {_id: 24, atomValue: ATOM_NULL, beforeExpr: false, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "let", postfix: false, prefix: false, _type : ""};  const _const:keyword_t = keyword_t {_id: 25, atomValue: ATOM_NULL, beforeExpr: false, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "const", postfix: false, prefix: false, _type : ""}; 
const _while:keyword_t = keyword_t {_id: 26, atomValue: ATOM_NULL, beforeExpr: false, binop: -1, isAssign: false, isLoop: true, isUpdate: false, keyword: "while", postfix: false, prefix: false, _type : ""};  const _with:keyword_t = keyword_t {_id: 27, atomValue: ATOM_NULL, beforeExpr: false, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "with", postfix: false, prefix: false, _type : ""};  const _new:keyword_t = keyword_t {_id: 28, atomValue: ATOM_NULL, beforeExpr: true, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "new", postfix: false, prefix: false, _type : ""}; 
const _this:keyword_t = keyword_t {_id: 29, atomValue: ATOM_NULL, beforeExpr: false, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "this", postfix: false, prefix: false, _type : ""}; 
const _class:keyword_t = keyword_t {_id: 30, atomValue: ATOM_NULL, beforeExpr: false, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "class", postfix: false, prefix: false, _type : ""};  const _extends:keyword_t = keyword_t {_id: 31, atomValue: ATOM_NULL, beforeExpr: true, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "extends", postfix: false, prefix: false, _type : ""}; 
const _export:keyword_t = keyword_t {_id: 32, atomValue: ATOM_NULL, beforeExpr: false, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "export", postfix: false, prefix: false, _type : ""};  const _import:keyword_t = keyword_t {_id: 33, atomValue: ATOM_NULL, beforeExpr: false, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "import", postfix: false, prefix: false, _type : ""}; 
const _yield:keyword_t = keyword_t {_id: 34, atomValue: ATOM_NULL, beforeExpr: true, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "yield", postfix: false, prefix: false, _type : ""}; 
const _null:keyword_t = keyword_t {_id: 35, atomValue: ATOM_NULL, beforeExpr: false, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "null", postfix: false, prefix: false, _type : ""};
const _true:keyword_t = keyword_t {_id: 36, atomValue: ATOM_TRUE, beforeExpr: false, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "true", postfix: false, prefix: false, _type : ""}; 
const _false:keyword_t = keyword_t {_id: 37, atomValue: ATOM_FALSE, beforeExpr: false, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "false", postfix: false, prefix: false, _type : ""}; 
const _in:keyword_t = keyword_t {_id: 38, atomValue: ATOM_NULL, beforeExpr: true, binop: 7, isAssign: false, isLoop: false, isUpdate: false, keyword: "in", postfix: false, prefix: false, _type : ""}; 
const _typeof:keyword_t = keyword_t {_id: 39, atomValue: ATOM_NULL, beforeExpr: true, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "typeof", postfix: false, prefix: true, _type : ""}; 
const _instanceof:keyword_t = keyword_t {_id: 40, atomValue: ATOM_NULL, beforeExpr: true, binop: 7, isAssign: false, isLoop: false, isUpdate: false, keyword: "instanceof", postfix: false, prefix: false, _type : ""}; 
const _void:keyword_t = keyword_t {_id: 41, atomValue: ATOM_NULL, beforeExpr: true, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "void", postfix: false, prefix: true, _type : ""}; 
const _delete:keyword_t = keyword_t {_id: 42, atomValue: ATOM_NULL, beforeExpr: true, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "delete", postfix: false, prefix: true, _type : ""}; 

fn keywordTypes(arg:&str) -> keyword_t { if (arg == "break") { return _break; } if (arg == "case") { return _case; } if (arg == "catch") { return _catch; } if (arg == "continue") { return _continue; } if (arg == "debugger") { return _debugger; } if (arg == "default") { return _default; } if (arg == "do") { return _do; } if (arg == "else") { return _else; } if (arg == "finally") { return _finally; } if (arg == "for") { return _for; } if (arg == "function") { return _function; } if (arg == "if") { return _if; } if (arg == "return") { return _return; } if (arg == "switch") { return _switch; } if (arg == "throw") { return _throw; } if (arg == "try") { return _try; } if (arg == "var") { return _var; } if (arg == "let") { return _let; } if (arg == "const") { return _const; } if (arg == "while") { return _while; } if (arg == "with") { return _with; } if (arg == "null") { return _null; } if (arg == "true") { return _true; } if (arg == "false") { return _false; } if (arg == "new") { return _new; } if (arg == "in") { return _in; } if (arg == "instanceof") { return _instanceof; } if (arg == "this") { return _this; } if (arg == "typeof") { return _typeof; } if (arg == "void") { return _void; } if (arg == "delete") { return _delete; } if (arg == "class") { return _class; } if (arg == "extends") { return _extends; } if (arg == "export") { return _export; } if (arg == "import") { return _import; } if (arg == "yield") { return _yield; } assert!(false); return _eof; } 

const _bracketL:keyword_t = keyword_t {_id: 43, atomValue: ATOM_NULL, beforeExpr: true, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "", postfix: false, prefix: false, _type : "["};  const _bracketR:keyword_t = keyword_t {_id: 44, atomValue: ATOM_NULL, beforeExpr: false, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "", postfix: false, prefix: false, _type : "]"};  const _braceL:keyword_t = keyword_t {_id: 45, atomValue: ATOM_NULL, beforeExpr: true, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "", postfix: false, prefix: false, _type : "{"}; 
const _braceR:keyword_t = keyword_t {_id: 46, atomValue: ATOM_NULL, beforeExpr: false, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "", postfix: false, prefix: false, _type : "}"};  const _parenL:keyword_t = keyword_t {_id: 47, atomValue: ATOM_NULL, beforeExpr: true, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "", postfix: false, prefix: false, _type : "("};  const _parenR:keyword_t = keyword_t {_id: 48, atomValue: ATOM_NULL, beforeExpr: false, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "", postfix: false, prefix: false, _type : ")"}; 
const _comma:keyword_t = keyword_t {_id: 49, atomValue: ATOM_NULL, beforeExpr: true, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "", postfix: false, prefix: false, _type : ","};  const _semi:keyword_t = keyword_t {_id: 50, atomValue: ATOM_NULL, beforeExpr: true, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "", postfix: false, prefix: false, _type : ";"}; 
const _colon:keyword_t = keyword_t {_id: 51, atomValue: ATOM_NULL, beforeExpr: true, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "", postfix: false, prefix: false, _type : ":"};  const _dot:keyword_t = keyword_t {_id: 52, atomValue: ATOM_NULL, beforeExpr: false, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "", postfix: false, prefix: false, _type : "."};  const _ellipsis:keyword_t = keyword_t {_id: 53, atomValue: ATOM_NULL, beforeExpr: false, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "", postfix: false, prefix: false, _type : "..."};  const _question:keyword_t = keyword_t {_id: 54, atomValue: ATOM_NULL, beforeExpr: true, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "", postfix: false, prefix: false, _type : "?"}; 
const _arrow:keyword_t = keyword_t {_id: 55, atomValue: ATOM_NULL, beforeExpr: true, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "", postfix: false, prefix: false, _type : "=>"};  const _bquote:keyword_t = keyword_t {_id: 56, atomValue: ATOM_NULL, beforeExpr: false, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "", postfix: false, prefix: false, _type : "`"};  const _dollarBraceL:keyword_t = keyword_t {_id: 57, atomValue: ATOM_NULL, beforeExpr: true, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "", postfix: false, prefix: false, _type : "${"}; 
const _slash:keyword_t = keyword_t {_id: 58, atomValue: ATOM_NULL, beforeExpr: true, binop: 10, isAssign: false, isLoop: false, isUpdate: false, keyword: "", postfix: false, prefix: false, _type : ""};  const _eq:keyword_t = keyword_t {_id: 59, atomValue: ATOM_NULL, beforeExpr: true, binop: -1, isAssign: true, isLoop: false, isUpdate: false, keyword: "", postfix: false, prefix: false, _type : ""}; 
const _assign:keyword_t = keyword_t {_id: 60, atomValue: ATOM_NULL, beforeExpr: true, binop: -1, isAssign: true, isLoop: false, isUpdate: false, keyword: "", postfix: false, prefix: false, _type : ""}; 
const _incDec:keyword_t = keyword_t {_id: 61, atomValue: ATOM_NULL, beforeExpr: false, binop: -1, isAssign: false, isLoop: false, isUpdate: true, keyword: "", postfix: true, prefix: true, _type : ""};  const _prefix:keyword_t = keyword_t {_id: 62, atomValue: ATOM_NULL, beforeExpr: true, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "", postfix: false, prefix: true, _type : ""}; 
const _logicalOR:keyword_t = keyword_t {_id: 63, atomValue: ATOM_NULL, beforeExpr: true, binop: 1, isAssign: false, isLoop: false, isUpdate: false, keyword: "", postfix: false, prefix: false, _type : ""}; 
const _logicalAND:keyword_t = keyword_t {_id: 64, atomValue: ATOM_NULL, beforeExpr: true, binop: 2, isAssign: false, isLoop: false, isUpdate: false, keyword: "", postfix: false, prefix: false, _type : ""}; 
const _bitwiseOR:keyword_t = keyword_t {_id: 65, atomValue: ATOM_NULL, beforeExpr: true, binop: 3, isAssign: false, isLoop: false, isUpdate: false, keyword: "", postfix: false, prefix: false, _type : ""}; 
const _bitwiseXOR:keyword_t = keyword_t {_id: 66, atomValue: ATOM_NULL, beforeExpr: true, binop: 4, isAssign: false, isLoop: false, isUpdate: false, keyword: "", postfix: false, prefix: false, _type : ""}; 
const _bitwiseAND:keyword_t = keyword_t {_id: 67, atomValue: ATOM_NULL, beforeExpr: true, binop: 5, isAssign: false, isLoop: false, isUpdate: false, keyword: "", postfix: false, prefix: false, _type : ""}; 
const _equality:keyword_t = keyword_t {_id: 68, atomValue: ATOM_NULL, beforeExpr: true, binop: 6, isAssign: false, isLoop: false, isUpdate: false, keyword: "", postfix: false, prefix: false, _type : ""}; 
const _relational:keyword_t = keyword_t {_id: 69, atomValue: ATOM_NULL, beforeExpr: true, binop: 7, isAssign: false, isLoop: false, isUpdate: false, keyword: "", postfix: false, prefix: false, _type : ""}; 
const _bitShift:keyword_t = keyword_t {_id: 70, atomValue: ATOM_NULL, beforeExpr: true, binop: 8, isAssign: false, isLoop: false, isUpdate: false, keyword: "", postfix: false, prefix: false, _type : ""}; 
const _plusMin:keyword_t = keyword_t {_id: 71, atomValue: ATOM_NULL, beforeExpr: true, binop: 9, isAssign: false, isLoop: false, isUpdate: false, keyword: "", postfix: false, prefix: true, _type : ""}; 
const _modulo:keyword_t = keyword_t {_id: 72, atomValue: ATOM_NULL, beforeExpr: true, binop: 10, isAssign: false, isLoop: false, isUpdate: false, keyword: "", postfix: false, prefix: false, _type : ""}; 
const _star:keyword_t = keyword_t {_id: 73, atomValue: ATOM_NULL, beforeExpr: true, binop: 10, isAssign: false, isLoop: false, isUpdate: false, keyword: "", postfix: false, prefix: false, _type : ""}; 
const tokTypes:TokTypes = TokTypes {bracketL: _bracketL, bracketR: _bracketR, braceL: _braceL, braceR: _braceR, parenL: _parenL, parenR: _parenR, comma: _comma, semi: _semi, colon: _colon, dot: _dot, ellipsis: _ellipsis, question: _question, slash: _slash, eq: _eq, name: _name, eof: _eof, num: _num, regexp: _regexp, string: _string, arrow: _arrow, bquote: _bquote, dollarBraceL: _dollarBraceL}; 

const isReservedWord3:|arg:&str|:'static -> bool = predicate_0; 
const isReservedWord5:|arg:&str|:'static -> bool = predicate_1; 
const isStrictReservedWord:|arg:&str|:'static -> bool = predicate_2; 
const isStrictBadIdWord:|arg:&str|:'static -> bool = predicate_3; 
const isEcma5AndLessKeyword:|arg:&str|:'static -> bool = predicate_4; 
const isEcma6Keyword:|arg:&str|:'static -> bool = predicate_5; 
const isKeyword:|arg:&str|:'static -> bool = isEcma5AndLessKeyword; 
const nonASCIIwhitespace:|arg:&str|:'static -> bool = regex_6; 
const nonASCIIidentifierStart:|arg:&str|:'static -> bool = regex_7; 
const nonASCIIidentifier:|arg:&str|:'static -> bool = regex_8; 
const newline:|arg:&str|:'static -> bool = regex_9; 
const lineBreak:|arg:&str|:'static -> bool = regex_10; 

fn isIdentifierStart(_code:int) -> bool {
    let mut code:u32 = _code.to_u32().unwrap();
    if (code < 65) {
return code==36;
}
    if (code < 91) {
return true;
}
    if (code < 97) {
return code==95;
}
    if (code < 123) {
return true;
}
    return code >= 0xaa && test(nonASCIIidentifierStart, fromCharCode(code).as_slice());
}


fn isIdentifierChar(_code:int) -> bool {
    let mut code:u32 = _code.to_u32().unwrap();
    if (code < 48) {
return code==36;
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
return code==95;
}
    if (code < 123) {
return true;
}
    return code >= 0xaa && test(nonASCIIidentifier, fromCharCode(code).as_slice());
}

static mut _input:&'static str = "";

fn set_input(me:&str) {
    unsafe {
        let val:&'static str = mem::transmute(me);
        _input = val;
    }
}

fn get_input() -> &'static str {
    unsafe {
        return _input;
    }
}


fn new_node (start:uint) -> Box<Node> {
    box Node {
        _type: "".to_string(),
        start: start,
        end: 0,

        sourceFile: "".to_string(),
        range: vec![],
        body: None,
        bodylist: vec![],
        label: None,
        test: None,
        declaration: None,
        source: None,
        specifiers: vec![],
        consequent: None,
        consequents: vec![],
        defaults: vec![],
        alternate: None,
        argument: None,
        discriminant: None,
        cases: vec![],
        block:None,
        handler:None,
        guardedHandlers: vec![],
        finalizer:None,
        object:None,
        expression:None,
        init:None,
        update:None,
        left:None,
        right:None,
        declarations: vec![],
        kind: "".to_string(),
        expressions: vec![],
        prefix:false,
        property:None,
        computed:false,
        callee:None,
        arguments: vec![],
        key:None,
        value: JS_NULL,
        raw:"".to_string(),
        elements: vec![],
        properties: vec![],
        id:None,
        param:None,
        params: vec![],
        blocks: vec![],
        rest:None,
        guard:None,
        name:"".to_string(),
        generator:false,
        of:false,
        quasi: None,
        quasis: vec![],
        tag: None,
        delegate:false,
        _default:false,
        _static:false,
        _operator:"".to_string(),
        filter: None,
        method:false,
        tail:false,
        shorthand:false,
        superClass: None,
        isExpression: false,
    }
}

impl AcornParser {
    pub fn new() -> AcornParser {
        return AcornParser {
            options: options_t {ecmaVersion: 5, strictSemicolons: false, allowTrailingCommas: true, forbidReserved: "", allowReturnOutsideFunction: false, locations: false, ranges: false, program: None, sourceFile: "", directSourceFile: ""},
            input: "".to_string(),
            sourceFile: "".to_string(),

            tokVal: None,
            labels: Vec::new(),
            tokType: None,
            tokPos: 0,
            tokStart: 0i,
            tokEnd: 0i,
            tokStartLoc: 0i,
            tokEndLoc: 0i,
            tokRegexpAllowed: false,
            tokCurLine: 0,
            tokLineStart: 0,
            lastStart: 0i,
            lastEnd: 0i,
            lastEndLoc: 0i,
            inFunction: false,
            inGenerator: false,
            _strict: false,
            metParenL: 0i,
            inTemplate: false,

            containsEsc: false,
        };
    }

pub fn parse<'a>(&mut self, inpt:&'a String) -> Box<Node> {
    let input:&'a str = inpt.as_slice();
    unsafe { set_input(input) };
    setOptions(defaultOptions);
    self.initTokenState();
    let mut program:Option<Box<Node>> = None;
    return self.parseTopLevel(program);
}

// fn Position() -> () {
//     THIS.line = self.tokCurLine;
//     THIS.column = self.tokPos - self.tokLineStart;
// }
    

fn initTokenState(&mut self) -> int {
    self.tokCurLine = 1;
    self.tokPos = 0;
    self.tokLineStart = 0;
    self.tokRegexpAllowed = true;
    self.metParenL = 0;
    self.inTemplate = false;
    self.skipSpace();
    return 0;
}

fn finishToken(&mut self, _type :keyword_t, val:js_any_type) -> int {
    self.tokEnd = self.tokPos as int;
    
    self.tokType = Some(_type);
    if (_type !=_bquote || self.inTemplate) {
        self.skipSpace();
    }
    self.tokVal = Some(val);
    self.tokRegexpAllowed = _type.beforeExpr;
    return 0;
}

fn skipBlockComment(&mut self) -> int {
    self.tokPos += 2;
    let mut start:int = self.tokPos as int;
    let mut end:int = indexOf(get_input(), "*/", self.tokPos as int);
    if end == -1 {
        raise((self.tokPos as int) - 2, "Unterminated comment");
    }
    self.tokPos = (end as uint) + 2;
    return 0;
}

fn skipLineComment(&mut self) {
    let mut start:int = self.tokPos as int;
    self.tokPos += 2;
    let mut ch:int = charCodeAt(get_input(), self.tokPos) as int; 
    while self.tokPos < get_input().len() && ch!=10 && ch!=13 && ch!=8232 && ch!=8233{
        self.tokPos+= 1;
        ch = charCodeAt(get_input(), self.tokPos) as int;
    }
}

fn skipSpace(&mut self) {
    while self.tokPos < get_input().len(){
{
        let mut ch:int = charCodeAt(get_input(), self.tokPos) as int;
        if (ch==32) {
{
            self.tokPos+= 1;
        }
} else {if (ch==13) {
{
            self.tokPos+= 1;
            let mut next:int = charCodeAt(get_input(), self.tokPos) as int; 
            if (next==10) {
{
                self.tokPos+= 1;
            }
}
            
        }
} else {if (ch==10 || ch==8232 || ch==8233) {
{
            self.tokPos+= 1;
            
        }
} else {if (ch > 8 && ch < 14) {
{
            self.tokPos+= 1;
        }
} else {if (ch==47) {
{
            let mut next:int = charCodeAt(get_input(), self.tokPos + 1) as int; 
            if (next==42) {
{
                self.skipBlockComment();
            }
} else {if (next==47) {
{
                self.skipLineComment();
            }
} else {break;}}
        }
} else {if (ch==160) {
{
            self.tokPos+= 1;
        }
} else {if (ch >= 5760 && test(nonASCIIwhitespace, fromCharCode(ch as u32).as_slice())) {
{
            self.tokPos+= 1;
        }
} else {{
            break;
        }}}}}}}}
    }
}
}
fn readToken_dot(&mut self) -> int {
    let mut next:int = charCodeAt(get_input(), self.tokPos + 1) as int; 
    if (next >= 48 && next <= 57) {
return self.readNumber(true);
}
    let mut next2:int = charCodeAt(get_input(), self.tokPos + 2) as int; 
    if (self.options.ecmaVersion >= 6 && next==46 && next2==46) {
{
        self.tokPos += 3;
        return self.finishToken(_ellipsis, JS_NULL);
    }
} else {{
        self.tokPos+= 1;
        return self.finishToken(_dot, JS_NULL);
    }}
}
fn readToken_slash(&mut self) -> int {
    let mut next:int = charCodeAt(get_input(), self.tokPos + 1) as int; 
    if (self.tokRegexpAllowed) {
{
        self.tokPos+= 1;
        return self.readRegexp();
    }
}
    if (next==61) {
return self.finishOp(_assign, 2);
}
    return self.finishOp(_slash, 1);
}
fn readToken_mult_modulo(&mut self, code:int) -> int {
    let mut next:int = charCodeAt(get_input(), self.tokPos + 1) as int; 
    if (next==61) {
return self.finishOp(_assign, 2);
}
    return self.finishOp(if code==42 { _star } else { _modulo }, 1);
}
fn readToken_pipe_amp(&mut self, code:int) -> int {
    let mut next:int = charCodeAt(get_input(), self.tokPos + 1) as int; 
    if (next==code) {
return self.finishOp(if code==124 { _logicalOR } else { _logicalAND }, 2);
}
    if (next==61) {
return self.finishOp(_assign, 2);
}
    return self.finishOp(if code==124 { _bitwiseOR } else { _bitwiseAND }, 1);
}
fn readToken_caret(&mut self) -> int {
    let mut next:int = charCodeAt(get_input(), self.tokPos + 1) as int; 
    if (next==61) {
return self.finishOp(_assign, 2);
}
    return self.finishOp(_bitwiseXOR, 1);
}
fn readToken_plus_min(&mut self, code:int) -> int {
    let mut next:int = charCodeAt(get_input(), self.tokPos + 1) as int; 
    if (next==code) {
{
        if (next == 45 && charCodeAt(get_input(), self.tokPos + 2) == 62 && test(newline, slice(get_input(), self.lastEnd, self.tokPos as int))) {
{
            self.tokPos += 3;
            self.skipLineComment();
            self.skipSpace();
            return self.readToken(false);
        }
}
        return self.finishOp(_incDec, 2);
    }
}
    if (next==61) {
return self.finishOp(_assign, 2);
}
    return self.finishOp(_plusMin, 1);
}
fn readToken_lt_gt(&mut self, code:int) -> int {
    let mut next:int = charCodeAt(get_input(), self.tokPos + 1) as int; 
    let mut size:int = 1; 
    if (next==code) {
{
        size = if code==62 && charCodeAt(get_input(), self.tokPos + 2)==62 { 3 } else { 2 };
        if (charCodeAt(get_input(), self.tokPos + size as uint)==61) {
return self.finishOp(_assign, size + 1);
}
        return self.finishOp(_bitShift, size);
    }
}
    if (next == 33 && code == 60 && charCodeAt(get_input(), self.tokPos + 2) == 45 && charCodeAt(get_input(), self.tokPos + 3) == 45) {
{
        self.tokPos += 4;
        self.skipLineComment();
        self.skipSpace();
        return self.readToken(false);
    }
}
    if (next==61) {
size = if charCodeAt(get_input(), self.tokPos + 2)==61 { 3 } else { 2 };
}
    return self.finishOp(_relational, size);
}
fn readToken_eq_excl(&mut self, code:int) -> int {
    let mut next:int = charCodeAt(get_input(), self.tokPos + 1) as int; 
    if (next==61) {
        let tokPos:uint = self.tokPos;
return self.finishOp(_equality, if charCodeAt(get_input(), tokPos + 2)==61 { 3 } else { 2 });
}
    if (code==61 && next==62 && self.options.ecmaVersion >= 6) {
{
        self.tokPos += 2;
        return self.finishToken(_arrow, JS_NULL);
    }
}
    return self.finishOp(if code==61 { _eq } else { _prefix }, 1);
}
fn getTokenFromCode(&mut self, code:int) -> bool {
    if (self.inTemplate) {
{
        if (self.tokType.unwrap()==_string) {
{
            if (code==96) {
{
                self.tokPos+= 1;
                self.finishToken(_bquote, JS_NULL);
                return true;
            }
}
            if (code==36 && charCodeAt(get_input(), self.tokPos + 1)==123) {
{
                self.tokPos += 2;
                self.finishToken(_dollarBraceL, JS_NULL);
                return true;
            }
}
        }
}
        self.readString(-1);
        return true;
    }
}
    match code{
46 => {self.readToken_dot();;
return true;},
40 => {self.tokPos+= 1;;
self.finishToken(_parenL, JS_NULL);;
return true;},
41 => {self.tokPos+= 1;;
self.finishToken(_parenR, JS_NULL);;
return true;},
59 => {self.tokPos+= 1;;
self.finishToken(_semi, JS_NULL);;
return true;},
44 => {self.tokPos+= 1;;
self.finishToken(_comma, JS_NULL);;
return true;},
91 => {self.tokPos+= 1;;
self.finishToken(_bracketL, JS_NULL);;
return true;},
93 => {self.tokPos+= 1;;
self.finishToken(_bracketR, JS_NULL);;
return true;},
123 => {self.tokPos+= 1;;
self.finishToken(_braceL, JS_NULL);;
return true;},
125 => {self.tokPos+= 1;;
self.finishToken(_braceR, JS_NULL);;
return true;},
58 => {self.tokPos+= 1;;
self.finishToken(_colon, JS_NULL);;
return true;},
63 => {self.tokPos+= 1;;
self.finishToken(_question, JS_NULL);;
return true;},
96 => {if (self.options.ecmaVersion >= 6) {
{
                self.tokPos+= 1;
                self.finishToken(_bquote, JS_NULL);
                return true;
            }
}},
48 => {
    let mut next:int = charCodeAt(get_input(), self.tokPos + 1) as int;

    if next == 120 || next == 88 {{
        self.readRadixNumber(16);
        return true;
    }}
    if self.options.ecmaVersion >= 6 {{
        if (next==111 || next==79) {{
            self.readRadixNumber(8);
            return true;
        }}
        if (next==98 || next==66) {{
            self.readRadixNumber(2);
            return true;
        }}
    }}
    self.readNumber(false);
    return true;
},
49 |
50 |
51 |
52 |
53 |
54 |
55 |
56 |
57 => {self.readNumber(false);;
return true;},
34 |
39 => {self.readString(code);;
return true;},
47 => {self.readToken_slash();;
return true;},
37 |
42 => {self.readToken_mult_modulo(code);;
return true;},
124 |
38 => {self.readToken_pipe_amp(code);;
return true;},
94 => {self.readToken_caret();;
return true;},
43 |
45 => {self.readToken_plus_min(code);;
return true;},
60 |
62 => {self.readToken_lt_gt(code);;
return true;},
61 |
33 => {self.readToken_eq_excl(code);;
return true;},
126 => {self.finishOp(_prefix, 1);;
return true;},
_ => { } }
    return false;
}
fn readToken(&mut self, forceRegexp:bool) -> int {
    if (!forceRegexp) {
self.tokStart = self.tokPos as int;
} else {self.tokPos = self.tokStart as uint + 1;}
    
    if (forceRegexp) {
return self.readRegexp();
}
    if (self.tokPos >= get_input().len()) {
return self.finishToken(_eof, JS_NULL);
}
    let mut code:int = charCodeAt(get_input(), self.tokPos) as int; 
    if (!self.inTemplate && (isIdentifierStart(code) || code==92)) {
return self.readWord();
}
    let mut tok:bool = self.getTokenFromCode(code); 
    if (tok==false) {
{
        let mut ch = fromCharCode(code as u32); 
        if (ch.as_slice()=="\\" || test(nonASCIIidentifierStart, ch.as_slice())) {
return self.readWord();
}
        raise(self.tokPos as int, ("Unexpected character '".to_string() + ch + "'").as_slice());
    }
}
return 0;
}
fn finishOp(&mut self, _type :keyword_t, size:int) -> int {
    let mut str:&str = slice(get_input(), self.tokPos as int, self.tokPos as int + size); 
    self.tokPos += size as uint;
    self.finishToken(_type, JS_STRING(str.to_string()));
    return 0;
}
fn readRegexp(&mut self) -> int {
    let mut content:&str = "";  let mut escaped:bool = false;  let mut inClass:bool = false;  let mut start:int = self.tokPos as int; 
    ;
loop{{
        if (self.tokPos >= get_input().len()) {
raise(start, "Unterminated regular expression");
}
        let ch:String = charAt(get_input(), self.tokPos);
        if (test(newline, ch.as_slice())) {
raise(start, "Unterminated regular expression");
}
        if (!escaped) {
{
            if (ch.as_slice()=="[") {
inClass = true;
} else {if (ch.as_slice()=="]" && inClass) {
inClass = false;
} else {if (ch.as_slice()=="/" && !inClass) {
break;
}}}
            escaped = ch.as_slice()=="\\";
        }
} else {escaped = false;}
        self.tokPos+= 1;
    };
}
    content = slice(get_input(), start, self.tokPos as int);
    self.tokPos+= 1;
    let mut mods:String = self.readWord1(); 
    if (mods.len() > 0 && !test(regex_11, mods.as_slice())) {
        raise(start, "Invalid regular expression flag");
    }
    let mut value = RegExp(content, mods.as_slice()); 
     if (value.len() == 0) { raise(start, "Error parsing regular expression."); } 
    return self.finishToken(_regexp, JS_REGEXP(value.to_string()));
}

fn readInt(&mut self, radix:int, len:int) -> f64 {
    let mut start:int = self.tokPos as int;  let mut total:f64 = 0f64; 
    let mut i:int = 0; ;
    while len == 0 || i < len{
        let mut code:int = charCodeAt(get_input(), self.tokPos) as int;
        let mut val:f64 = 0.0; 
        if (code >= 97) {
            val = (code - 97 + 10) as f64;
        } else if (code >= 65) {
            val = (code - 65 + 10) as f64;
        } else if (code >= 48 && code <= 57) {
            val = (code - 48) as f64;
        } else {
            val = Infinity;
        }
        if (val >= radix as f64) {
            break;
        }
        self.tokPos += 1;
        total = total * (radix as f64) + val;
    }
    if (self.tokPos==start as uint || (len > 0 && self.tokPos as int - start !=len)) {
        return NaN;
    }
    return total;
}

fn readRadixNumber(&mut self, radix:int) -> int {
    self.tokPos += 2;
    let mut val:f64 = self.readInt(radix, 0); 
    if (isNaN(val)) {
        raise(self.tokStart + 2, ("Expected number in radix ".to_string() + radix.to_string()).as_slice());
    }
    if (isIdentifierStart(charCodeAt(get_input(), self.tokPos) as int)) {
        raise(self.tokPos as int, "Identifier directly after number");
    }
    return self.finishToken(_num, JS_DOUBLE(val as f64));
}

fn readNumber(&mut self, startsWithDot:bool) -> int {
    let mut start:int = self.tokPos as int;
    let mut isFloat:bool = false;
    let mut octal:bool = charCodeAt(get_input(), self.tokPos)==48; 
    if (!startsWithDot && isNaN(self.readInt(10, 0))) {
        raise(start, "Invalid number");
    }
    if (charCodeAt(get_input(), self.tokPos)==46) {{
        self.tokPos+= 1;
        self.readInt(10, 0);
        isFloat = true;
    }}
    let mut next:int = charCodeAt(get_input(), self.tokPos) as int; 
    if (next==69 || next==101) {{
        self.tokPos+= 1;
        next = charCodeAt(get_input(), self.tokPos) as int;
        if (next==43 || next==45) {
            self.tokPos+= 1;
        }
        if (isNaN(self.readInt(10, 0))) {
            raise(start, "Invalid number");
        }
        isFloat = true;
    }}
    if (isIdentifierStart(charCodeAt(get_input(), self.tokPos) as int)) {
        raise(self.tokPos as int, "Identifier directly after number");
    }
    let mut str:&str = slice(get_input(), start, self.tokPos as int);
    let mut val:f64 = 0f64; 
    if (isFloat) {
        val = parseFloat(str);
    } else {if (!octal || str.len()==1) {
        val = parseInt(str, 10) as f64;
    } else {if (test(regex_12, str) || self._strict) {
        raise(start, "Invalid number");
    } else {
        val = parseInt(str, 8) as f64;
    }}}
    return self.finishToken(_num, JS_DOUBLE(val));
}

fn readCodePoint(&mut self) -> String {
    let mut ch:int = charCodeAt(get_input(), self.tokPos) as int;  let mut code:int = 0i; 
    if (ch==123) {
{
        if (self.options.ecmaVersion < 6) {
self.expected(None);
}
        self.tokPos+= 1;
        let pos = self.tokPos as int;
        code = self.readHexChar(indexOf(get_input(), "}", pos) - pos);
        self.tokPos+= 1;
        if (code > 0x10FFFF) {
self.expected(None);
}
    }
} else {{
        code = self.readHexChar(4);
    }}
    if (code <= 0xFFFF) {
{
        return fromCharCode(code as u32);
    }
}
    let mut cu1:int = ((code - 0x10000) >> 10) + 0xD800; 
    let mut cu2:int = ((code - 0x10000) & 1023) + 0xDC00; 
    return fromCharCode2(cu1 as u32, cu2 as u32);
}

fn readString(&mut self, quote:int) -> int {
    if !self.inTemplate {
        self.tokPos += 1;
    }
    let mut out:String = "".to_string();

    loop{{
        if (self.tokPos >= get_input().len()) {
            raise(self.tokStart, "Unterminated string constant");
        }
        let mut ch:int = charCodeAt(get_input(), self.tokPos) as int; 
        if (self.inTemplate) {{
            if (ch==96 || ch==36 && charCodeAt(get_input(), self.tokPos + 1)==123) {
                return self.finishToken(_string, JS_STRING(out.to_string()));
            }
        }} else {if (ch==quote) {{
            self.tokPos+= 1;
            return self.finishToken(_string, JS_STRING(out.to_string()));
        }}}
        if (ch==92) {{
            self.tokPos+= 1;
            ch = charCodeAt(get_input(), self.tokPos) as int;
            let mut octalmatch = exec(regex_13, slice(get_input(), self.tokPos as int, self.tokPos as int + 3)); 
            self.tokPos+= 1;
            if octalmatch.is_some() {{
                let mut octal:String = octalmatch.unwrap()[0].clone();
                while octal.len() > 0 && parseInt(octal.as_slice(), 8) > 255 {
                    octal = slice(octal.as_slice(), 0, -1).to_string();
                }
                if (octal.as_slice()=="0") {
                    out.push('\0');
                } else {
                    if (self._strict) {
                        raise(self.tokPos as int - 2, "Octal literal in self._strict mode");
                    }
                    out.push_str(fromCharCode(parseInt(octal.as_slice(), 8) as u32).as_slice());
                    self.tokPos += octal.len() - 1;
                }
            }} else {{
                match ch{
                    110 => {out.push('\n');; },
                    114 => {out.push('\r');;
                    break;},
                    120 => {out.push_str(fromCharCode(self.readHexChar(2) as u32).as_slice());;
                    break;},
                    117 => {out.push_str(self.readCodePoint().as_slice());;
                    break;},
                    85 => {out.push_str(fromCharCode(self.readHexChar(8) as u32).as_slice());;
                    break;},
                    116 => {out.push('\t');;
                    break;},
                    98 => {out.push('\u0008');;
                    break;},
                    118 => {out.push('\u000b');;
                    break;},
                    102 => {out.push('\u000c');;
                    break;},
                    48 => {out.push('\u0000');;
                    break;},
                    13 => {
                        if (charCodeAt(get_input(), self.tokPos)==10) {
                            self.tokPos+= 1;
                        }
                    },
                    10 => { },
                    _ => {out.push_str(fromCharCode(ch as u32).as_slice());;
                    break;}
                }
            }}
        }} else {{
            self.tokPos+= 1;
            if (test(newline, fromCharCode(ch as u32).as_slice())) {{
                if (self.inTemplate) {{
                    if (ch==13 && charCodeAt(get_input(), self.tokPos)==10) {{
                        self.tokPos+= 1;
                        ch = 10;
                    }}
                }} else {{
                    raise(self.tokStart, "Unterminated string constant");
                }}
            }}
            out.push_str(fromCharCode(ch as u32).as_slice());
        }}
    }}
    return 0;
}

fn readHexChar(&mut self, len:int) -> int {
    let mut n:f64 = self.readInt(16, len); 
    if (isNaN(n)) {
raise(self.tokStart, "Bad character escape sequence");
}
    return n as int;
}

fn readWord1(&mut self) -> String {
    self.containsEsc = false;
    let mut word:String = "".to_string();
    let mut first:bool = true;
    let mut start:int = self.tokPos as int;
    loop {
        let mut ch:int = charCodeAt(get_input(), self.tokPos); 
        if (isIdentifierChar(ch)) {
            if (self.containsEsc) {
                word.push_str(charAt(get_input(), self.tokPos).as_slice());
            }
            self.tokPos+= 1;
        } else {
            if (ch==92) {
                if (!self.containsEsc) {
                    word = slice(get_input(), start, self.tokPos as int).to_string();
                }
                self.containsEsc = true;
                self.tokPos+= 1;
                if (charCodeAt(get_input(), self.tokPos) != 117) {
                    raise(self.tokPos as int, "Expecting Unicode escape sequence \\uXXXX");
                }
                self.tokPos+= 1;
                let mut esc:int = self.readHexChar(4); 
                let mut escStr:String = fromCharCode(esc as u32);
                if (escStr.len() == 0) {
                    raise(self.tokPos as int - 1, "Invalid Unicode escape");
                }
                if (!(if first { isIdentifierStart(esc) } else { isIdentifierChar(esc) })) {
                    raise(self.tokPos as int - 4, "Invalid Unicode escape");
                }
                word.push_str(escStr.as_slice());
            } else {
                break;
            }
        }
        first = false;
    }
    if self.containsEsc {
        word
    } else {
        slice(get_input(), start, self.tokPos as int).to_string()
    }
}

fn readWord(&mut self) -> int {
    let mut word:String = self.readWord1(); 
    let mut _type :keyword_t = _name; 
    if (!self.containsEsc && isKeyword(word.as_slice())) {
_type = keywordTypes(word.as_slice());
}
    return self.finishToken(_type, JS_STRING(word.to_string()));
}
fn next(&mut self) -> int {
    self.lastStart = self.tokStart;
    self.lastEnd = self.tokEnd;
    self.lastEndLoc = self.tokEndLoc;
    self.readToken(false);
    return 0;
}
fn setStrict(&mut self, strct:bool) -> int {
    self._strict = strct;
    self.tokPos = self.tokStart as uint;
    
    self.skipSpace();
    self.readToken(false);
    return 0;
}

    
    


    


fn startNode(&mut self) -> Box<Node> {
    let mut node:Box<Node> = new_node(self.tokStart as uint);
    
    if (self.options.directSourceFile.len() > 0) {
        node.sourceFile = self.options.directSourceFile.to_string();
    }
    if (self.options.ranges) {
        node.range = vec![self.tokStart, 0];
    }
    return node;
}
fn startNodeFrom(&mut self, other:&Box<Node>) -> Box<Node> {
    let mut node:Box<Node> = new_node(other.start);
    
    if (self.options.ranges) {
node.range = vec![other.range[0], 0];
}
    return node;
}
fn enterNode<'a>(&mut self, node:&'a mut Box<Node>, _type :&str) -> &'a mut Box<Node> {
    node._type = _type.to_string();
    return node;
}
fn finishNode(&mut self, mut node:Box<Node>) -> Box<Node> {
    node.end = self.lastEnd as uint;
    
    if (self.options.ranges) {
node.range[1] = self.lastEnd;
}
     jsparse_callback_close(convert_to_Node_C(&mut node));
    return node;
}

fn eat(&mut self, _type :keyword_t) -> bool {
    write!(io::stderr(), "-> eat {} expecting {}", self.tokType.unwrap(), _type);
    if (self.tokType.unwrap() == _type) {
        self.next();
        return true;
    } else {
        return false;
    }
}

fn canInsertSemicolon(&mut self) -> bool {
    return !self.options.strictSemicolons && (self.tokType.unwrap() ==_eof || self.tokType.unwrap()==_braceR || test(newline, slice(get_input(), self.lastEnd, self.tokStart)));
}

fn semicolon(&mut self) -> int {
    if (!self.eat(_semi) && !self.canInsertSemicolon()) {
        self.expected(None);
    }
    return 0;
}

fn expect(&mut self, _type :keyword_t) -> int {
    if !self.eat(_type) { self.expected(None); }
    return 0;
}

fn expected(&mut self, pos:Option<int>) -> int {
    raise(match pos { Some(p) => p, None => self.tokStart }, "Unexpected token");
    return 0;
}

fn checkSpreadAssign<'a>(&'a mut self, node:&'a mut Box<Node>) -> int {
    if (node._type.as_slice()!="Identifier" && node._type.as_slice()!="ArrayPattern") {
        self.expected(Some(node.start as int));
    }
    return 0;
}

fn parseTopLevel(&mut self, mut program:Option<Box<Node>>) -> Box<Node> {
    self.lastEnd = self.tokPos as int;
    self.lastStart = self.tokPos as int;
    writeln!(io::stderr(), "hey");
    
    self._strict = false;
    self.inGenerator = false;
    self.inFunction = false;
    self.labels = vec![];
    self.readToken(false);
    let hasprogram = program.is_some();
    let mut node:Box<Node> = if program.is_none() { self.startNode() } else { program.unwrap() };
    let mut first:bool = true; 
    if !hasprogram {
        node.bodylist = vec![];
    }
    while self.tokType.unwrap() !=_eof{
        let mut stmt:Box<Node> = self.parseStatement(); 
        if (first && isUseStrict(&mut stmt)) {
            self.setStrict(true);
        }
        node.bodylist.push(stmt);
        first = false;
    }
    writeln!(io::stderr(), "ho");
    self.enterNode(&mut node, "Program");
    writeln!(io::stderr(), "done");
    return self.finishNode(node);
}

fn parseStatement(&mut self) -> Box<Node> {
    jsparse_callback_open("parseStatement"); 
    let tok = self.tokType.clone().unwrap();
    let val = self.tokVal.clone().unwrap();
    if (tok==_slash || tok==_assign && val.to_string().as_slice() == "/=") {
        self.readToken(true);
    }
    let mut starttype:keyword_t = tok;
    let node:&mut Box<Node> = &mut self.startNode(); 
    match starttype{
        _break |
        _continue => {return self.parseBreakContinueStatement(node, starttype.keyword);},
        _debugger => {return self.parseDebuggerStatement(node);},
        _do => {return self.parseDoStatement(node);},
        _for => {return self.parseForStatement(node);},
        _function => {return self.parseFunctionStatement(node);},
        _class => {return self.parseClass(node, true);},
        _if => {return self.parseIfStatement(node);},
        _return => {return self.parseReturnStatement(node);},
        _switch => {return self.parseSwitchStatement(node);},
        _throw => {return self.parseThrowStatement(node);},
        _try => {return self.parseTryStatement(node);},
        _var |
        _let |
        _const => {return self.parseVarStatement(node, starttype.keyword);},
        _while => {return self.parseWhileStatement(node);},
        _with => {return self.parseWithStatement(node);},
        _braceL => {return self.parseBlock(false);},
        _semi => {return self.parseEmptyStatement(node);},
        _export => {return self.parseExport(node);},
        _import => {return self.parseImport(node);},
        _ => {
            let mut maybeName = val;
            let mut expr:Box<Node> = self.parseExpression(false, false);
            if starttype==_name && expr._type.as_slice()=="Identifier" && self.eat(_colon) {
                return self.parseLabeledStatement(node, maybeName.to_string().as_slice(), &mut expr);
            } else {
                return self.parseExpressionStatement(node, expr);
            }
        }
    }
}

fn parseBreakContinueStatement<'a>(&'a mut self, node:&'a mut Box<Node>, keyword:&str) -> Box<Node> {
    let mut isBreak:bool = keyword == "break"; 
    self.next();
    if (self.eat(_semi) || self.canInsertSemicolon()) {
        node.label = None;
    } else {
        if (self.tokType.unwrap()!=_name) {
            self.expected(None);
} else {{
        node.label = Some(self.parseIdent(false));
        self.semicolon();
    }}}
    let mut i:uint = 0; ;
while i < self.labels.len(){{
        let lab:&mut label_t = &mut self.labels[i];
        if node.label.is_none() || lab.name==node.label.clone().unwrap().name {
{
            if (lab.kind.as_slice() != null && (isBreak || lab.kind.as_slice()=="loop")) {
break;
}
            if (node.label.is_some() && isBreak) {
break;
}
        }
}
    };
}
    if (i==self.labels.len()) {
raise(node.start as int, ("Unsyntactic ".to_string() + keyword).as_slice());
}
    self.enterNode(node, if isBreak { "BreakStatement" } else { "ContinueStatement" });
    return self.finishNode(node.clone());
}
fn parseDebuggerStatement<'a>(&'a mut self, node:&'a mut Box<Node>) -> Box<Node> {
    self.next();
    self.semicolon();
    self.enterNode(node, "DebuggerStatement");
    return self.finishNode(node.clone());
}
fn parseDoStatement<'a>(&'a mut self, node:&'a mut Box<Node>) -> Box<Node> {
self.    next();
    self.labels.push((*loopLabel).clone());
    node.body = Some(self.parseStatement());
    self.labels.pop();
    self.expect(_while);
    node.test = Some(self.parseParenExpression());
    self.semicolon();
    self.enterNode(node, "DoWhileStatement");
    return self.finishNode(node.clone());
}
fn parseForStatement<'a>(&'a mut self, node:&'a mut Box<Node>) -> Box<Node> {
    self.next();
    self.labels.push((*loopLabel).clone());
    self.expect(_parenL);
    if (self.tokType.unwrap()==_semi) {
return self.parseFor(node, (*nullptr).clone());
}
    if (self.tokType.unwrap()==_var || self.tokType.unwrap()==_let) {
{
        let mut init:Box<Node> = self.startNode();  let mut varKind:&str = self.tokType.unwrap().keyword;  let mut isLet:bool = self.tokType.unwrap()==_let; 
        self.next();
        self.parseVar(&mut init, true, varKind);
        self.enterNode(&mut init, "VariableDeclaration");
        init = self.finishNode(init);
        if ((self.tokType.unwrap()==_in || (self.tokType.unwrap()==_name && self.tokVal.clone().unwrap().to_string().as_slice()=="of")) && init.declarations.len()==1 && !(isLet && init.declarations[0].init.is_some())) {
return self.parseForIn(node, init);
}
        return self.parseFor(node, init);
    }
}
    let mut init:Box<Node> = self.parseExpression(false, true); 
    if (self.tokType.unwrap()==_in || (self.tokType.unwrap()==_name && self.tokVal.clone().unwrap().to_string().as_slice()=="of")) {
{
        {
        }
        return self.parseForIn(node, init);
    }
}
    return self.parseFor(node, init);
}
fn parseFunctionStatement<'a>(&'a mut self, node:&'a mut Box<Node>) -> Box<Node> {
     jsparse_callback_open("function-declaration"); 
    self.next();
    return self.parseFunction(node, true, false);
}
fn parseIfStatement<'a>(&'a mut self, node:&'a mut Box<Node>) -> Box<Node> {
     jsparse_callback_open("if-start"); 
    self.enterNode(node, "IfStatement");
    self.next();
     jsparse_callback_open("if-test"); 
    node.test = Some(self.parseParenExpression());
     jsparse_callback_open("if-consequent"); 
    node.consequent = Some(self.parseStatement());
    if (self.eat(_else)) {
{
         jsparse_callback_open("if-alternate"); 
        node.alternate = Some(self.parseStatement());
    }
} else {{
         jsparse_callback_open("if-no-alternate"); 
        node.alternate = None;
    }}
     jsparse_callback_open("if-end"); 
    return self.finishNode(node.clone());
}
fn parseReturnStatement<'a>(&'a mut self, node:&'a mut Box<Node>) -> Box<Node> {
     jsparse_callback_open("parseReturnStatement"); 
    if (!self.inFunction && !self.options.allowReturnOutsideFunction) {
raise(self.tokStart, "'return' outside of function");
}
    self.next();
    if (self.eat(_semi) || self.canInsertSemicolon()) {
node.argument = None;
} else {{
        node.argument = Some(self.parseExpression(false, false));
        self.semicolon();
    }}
    if (node.argument.is_none()) {
{
         jsparse_callback_open("return-no-argument"); 
    }
} else {{
         jsparse_callback_open("return-argument"); 
    }}
    self.enterNode(node, "ReturnStatement");
    return self.finishNode(node.clone());
}
fn parseSwitchStatement<'a>(&'a mut self, node:&'a mut Box<Node>) -> Box<Node> {
    self.next();
    node.discriminant = Some(self.parseParenExpression());
    node.cases = vec![];
    self.expect(_braceL);
    self.labels.push((*switchLabel).clone());
    let mut cur:Option<Box<Node>> = None;  let mut sawDefault:bool = false;
    while self.tokType.unwrap() != _braceR {
        if (self.tokType.unwrap()==_case || self.tokType.unwrap()==_default) {
            let mut isCase:bool = self.tokType.unwrap()==_case; 
            if (cur.is_some()) {
                let mut node2 = cur.unwrap();
                self.enterNode(&mut node2, "SwitchCase");
                node.cases.push(self.finishNode(node2));
            }
            let mut node:Box<Node> = self.startNode();
            node.consequents = vec![];
            self.next();
            if (isCase) {
                node.test = Some(self.parseExpression(false, false));
            } else {
                if (sawDefault) {
                    raise(self.lastStart, "Multiple default clauses");
                }
                sawDefault = true;
                node.test = None;
            }
            self.expect(_colon);
            cur = Some(node);
        } else {
            match cur {
                Some(node) => {
                    let mut node2 = node.clone();
                    node2.consequents.push(self.parseStatement());
                    cur = Some(node2);
                },
                None => { self.expected(None); }
            }
        }
    }
    if (cur.is_some()) {
        let mut node2 = cur.unwrap();
        self.enterNode(&mut node2, "SwitchCase");
        node.cases.push(self.finishNode(node2));
    }
    self.next();
    self.labels.pop();
    self.enterNode(node, "SwitchStatement");
    return self.finishNode(node.clone());
}

fn parseThrowStatement<'a>(&'a mut self, node:&'a mut Box<Node>) -> Box<Node> {
    self.next();
    if (test(newline, slice(get_input(), self.lastEnd, self.tokStart))) {
raise(self.lastEnd, "Illegal newline after throw");
}
    node.argument = Some(self.parseExpression(false, false));
    self.semicolon();
    self.enterNode(node, "ThrowStatement");
    return self.finishNode(node.clone());
}
fn parseTryStatement<'a>(&'a mut self, node:&'a mut Box<Node>) -> Box<Node> {
    self.next();
    node.block = Some(self.parseBlock(false));
    node.handler = None;
    if (self.tokType.unwrap()==_catch) {
{
        let mut clause:Box<Node> = self.startNode(); 
        self.next();
        self.expect(_parenL);
        clause.param = Some(self.parseIdent(false));
        let param = clause.param.clone().unwrap();
        if (self._strict && isStrictBadIdWord(param.name.as_slice())) {
raise(param.start as int, ("Binding ".to_string() + param.name + " in self._strict mode").as_slice());
}
        self.expect(_parenR);
        clause.guard = None;
        clause.body = Some(self.parseBlock(false));
        self.enterNode(&mut clause, "CatchClause");
        node.handler = Some(self.finishNode(clause));
    }
}
    node.guardedHandlers = (*empty).clone();
    node.finalizer = if self.eat(_finally) { Some(self.parseBlock(false)) } else { None };
    if (node.handler.is_none() && node.finalizer.is_none()) {
raise(node.start as int, "Missing catch or finally clause");
}
    self.enterNode(node, "TryStatement");
    return self.finishNode(node.clone());
}
fn parseVarStatement<'a>(&'a mut self, node:&'a mut Box<Node>, kind:&str) -> Box<Node> {
    self.next();
    self.parseVar(node, false, kind);
    self.semicolon();
    self.enterNode(node, "VariableDeclaration");
    return self.finishNode(node.clone());
}
fn parseWhileStatement<'a>(&'a mut self, node:&'a mut Box<Node>) -> Box<Node> {
    self.next();
     jsparse_callback_open("while-test"); 
    node.test = Some(self.parseParenExpression());
    self.labels.push((*loopLabel).clone());
     jsparse_callback_open("while-body"); 
    node.body = Some(self.parseStatement());
    self.labels.pop();
    self.enterNode(node, "WhileStatement");
     jsparse_callback_open("while-end"); 
    return self.finishNode(node.clone());
}
fn parseWithStatement<'a>(&'a mut self, node:&'a mut Box<Node>) -> Box<Node> {
    if (self._strict) {
raise(self.tokStart, "'with' in self._strict mode");
}
    self.next();
    node.object = Some(self.parseParenExpression());
    node.body = Some(self.parseStatement());
    self.enterNode(node, "WithStatement");
    return self.finishNode(node.clone());
}
fn parseEmptyStatement<'a>(&'a mut self, node:&'a mut Box<Node>) -> Box<Node> {
    self.next();
    self.enterNode(node, "EmptyStatement");
    return self.finishNode(node.clone());
}
fn parseLabeledStatement<'a>(&'a mut self, node:&'a mut Box<Node>, maybeName:&str, expr:&mut Box<Node>) -> Box<Node> {
    let mut i:uint = 0; ;
while i < self.labels.len(){if (self.labels[i].name.as_slice()==maybeName) {
raise(expr.start as int, ("Label '".to_string() + maybeName + "' is already declared").as_slice());
};
i = i + 1;
}
    let mut kind:&str = if self.tokType.unwrap().isLoop { "loop" } else { if self.tokType.unwrap()==_switch { "switch" } else { null } }; 
    self.labels.push(label_t {kind: kind.to_string(), name: maybeName.to_string()});
    node.body = Some(self.parseStatement());
    self.labels.pop();
    node.label = Some(expr.clone());
    self.enterNode(node, "LabeledStatement");
    return self.finishNode(node.clone());
}
fn parseExpressionStatement<'a>(&'a mut self, node:&'a mut Box<Node>, expr:Box<Node>) -> Box<Node> {
    node.expression = Some(expr);
    self.semicolon();
    self.enterNode(node, "ExpressionStatement");
    return self.finishNode(node.clone());
}
fn parseParenExpression(&mut self) -> Box<Node> {
    self.expect(_parenL);
    let val:Box<Node> = self.parseExpression(false, false); 
    self.expect(_parenR);
    return val;
}
fn parseBlock(&mut self, allowStrict:bool) -> Box<Node> {
    let node:&mut Box<Node> = &mut self.startNode();
    let mut first:bool = true;
    let mut isstrict:bool = false;
    let mut oldStrict:bool = false; 
    node.bodylist = vec![];
    self.expect(_braceL);
    while !self.eat(_braceR){
{
        let mut stmt:Box<Node> = self.parseStatement(); 
        if (first && allowStrict && isUseStrict(&mut stmt)) {
            oldStrict = isstrict;
            isstrict = true;
            self.setStrict(isstrict);
        }
        node.bodylist.push(stmt);
        first = false;
    }
}
    if (isstrict && !oldStrict) {
self.setStrict(false);
}
    self.enterNode(node, "BlockStatement");
    return self.finishNode(node.clone());
}
fn parseFor<'a>(&'a mut self, node:&'a mut Box<Node>, init:Box<Node>) -> Box<Node> {
    node.init = Some(init);
    self.expect(_semi);
     jsparse_callback_open("for-test"); 
    node.test = if self.tokType.unwrap()==_semi { None } else { Some(self.parseExpression(false, false)) };
    self.expect(_semi);
     jsparse_callback_open("for-update"); 
    node.update = if self.tokType.unwrap()==_parenR { None } else { Some(self.parseExpression(false, false)) };
    self.expect(_parenR);
     jsparse_callback_open("for-body"); 
    node.body = Some(self.parseStatement());
    self.labels.pop();
     jsparse_callback_open("for-end"); 
    self.enterNode(node, "ForStatement");
    return self.finishNode(node.clone());
}
fn parseForIn<'a>(&'a mut self, node:&'a mut Box<Node>, init:Box<Node>) -> Box<Node> {
    let mut _type :&str = if self.tokType.unwrap()==_in { "ForInStatement" } else { "ForOfStatement" }; 
    self.next();
    node.left = Some(init);
    node.right = Some(self.parseExpression(false, false));
    self.expect(_parenR);
    node.body = Some(self.parseStatement());
    self.labels.pop();
    self.enterNode(node, _type);
    return self.finishNode(node.clone());
}
fn parseVar<'a>(&mut self, node:&'a mut Box<Node>, noIn:bool, kind:&str) -> &'a mut Box<Node> {
    node.declarations = vec![];
    node.kind = kind.to_string();
    ;
loop{{
         jsparse_callback_open("var-declarator"); 
        let mut decl:Box<Node> = self.startNode(); 
        decl.id = Some(if self.options.ecmaVersion >= 6 { toAssignable(self.parseExprAtom(), false, false) } else { self.parseIdent(false) });
        {
        }
        if (self.eat(_eq)) {
{
             jsparse_callback_open("var-declarator-assign"); 
            decl.init = Some(self.parseExpression(true, noIn));
        }
} else {if (kind==_const.keyword) {
{
            self.expected(None);
        }
} else {{
             jsparse_callback_open("var-declarator-no-assign"); 
            decl.init = None;
        }}}
        self.enterNode(&mut decl, "VariableDeclarator");
        node.declarations.push(self.finishNode(decl));
        if (!self.eat(_comma)) {
break;
}
    };
}
    return node;
}
fn parseExpression(&mut self, noComma:bool, noIn:bool) -> Box<Node> {
    jsparse_callback_open("parseExpression"); 
    let mut expr:Box<Node> = self.parseMaybeAssign(noIn); 

    if (!noComma && self.tokType.unwrap()==_comma) {
        let node:&mut Box<Node> = &mut self.startNodeFrom(&expr);
        node.expressions = vec![expr];
        while self.eat(_comma){
            node.expressions.push(self.parseMaybeAssign(noIn));
        }
        self.enterNode(node, "SequenceExpression");
        return self.finishNode(node.clone());
    }
    return expr;
}
fn parseMaybeAssign(&mut self, noIn:bool) -> Box<Node> {
    let mut left:Box<Node> = self.parseMaybeConditional(noIn); 
    if (self.tokType.unwrap().isAssign) {
{
    let tokVal = self.tokVal.clone().unwrap();
         jsparse_callback_open(tokVal.to_string().as_slice()); 
        let node:&mut Box<Node> = &mut self.startNodeFrom(&left); 
        node._operator = tokVal.to_string();
        node.left = Some(if self.tokType.unwrap()==_eq { toAssignable(left, false, false) } else { left });
        {
        }
        self.next();
        node.right = Some(self.parseMaybeAssign(noIn));
        self.enterNode(node, "AssignmentExpression");
        return self.finishNode(node.clone());
    }
}
    return left;
}
fn parseMaybeConditional(&mut self, noIn:bool) -> Box<Node> {
    let mut expr:Box<Node> = self.parseExprOps(noIn); 
    if (self.eat(_question)) {
{
        let node:&mut Box<Node> = &mut self.startNodeFrom(&expr); 
        node.test = Some(expr);
         jsparse_callback_open("ternary-consequent"); 
        node.consequent = Some(self.parseExpression(true, false));
        self.expect(_colon);
         jsparse_callback_open("ternary-alternate"); 
        node.alternate = Some(self.parseExpression(true, noIn));
        self.enterNode(node, "ConditionalExpression");
        return self.finishNode(node.clone());
    }
}
    return expr;
}

fn parseExprOps(&mut self, noIn:bool) -> Box<Node> {
    let left = self.parseMaybeUnary();
    return self.parseExprOp(left, -1, noIn);
}

fn parseExprOp(&mut self, left:Box<Node>, minPrec:int, noIn:bool) -> Box<Node> {
    let mut prec:int = self.tokType.unwrap().binop;
    if (prec >= 0 && (!noIn || self.tokType.unwrap()!=_in)) {
        if (prec > minPrec) {
            let node:&mut Box<Node> = &mut self.startNodeFrom(&left); 
            node.left = Some(left);
            let val = self.tokVal.clone().unwrap();
            node._operator = match val {
                JS_STRING(ref s) => s.clone(),
                _ => "".to_string()
            };
             jsparse_callback_open(val.to_string().as_slice());
            let mut op:keyword_t = self.tokType.unwrap();
            self.next();
            let left = self.parseMaybeUnary();
            node.right = Some(self.parseExprOp(left, prec, noIn));
            self.enterNode(node, if op==_logicalOR || op==_logicalAND { "LogicalExpression" } else { "BinaryExpression" });
            let mut exprNode:Box<Node> = self.finishNode(node.clone()); 
            return self.parseExprOp(exprNode, minPrec, noIn);
        }
    }
    return left;
}
fn parseMaybeUnary(&mut self) -> Box<Node> {
    let tokVal = self.tokVal.clone().unwrap();
    if (self.tokType.unwrap().keyword == "function") { jsparse_callback_open(tokVal.to_string().as_slice()); } 
    if (self.tokType.unwrap().prefix) {
{
         jsparse_callback_open(("unary-".to_string() + tokVal.to_string()).as_slice()); 
        let node:&mut Box<Node> = &mut self.startNode();  let mut update:bool = self.tokType.unwrap().isUpdate; 
        node._operator = tokVal.to_string();
        node.prefix = true;
        self.tokRegexpAllowed = true;
        self.next();
        self.enterNode(node, if update { "UpdateExpression" } else { "UnaryExpression" });
        node.argument = Some(self.parseMaybeUnary());
        if (update) {
checkLVal(&node.argument.clone().unwrap());
} else {if (self._strict && node._operator.as_slice()=="delete" && node.argument.clone().unwrap()._type.as_slice()=="Identifier") {
raise(node.start as int, "Deleting local variable in self._strict mode");
}}
        return self.finishNode(node.clone());
    }
}
    let mut expr:Box<Node> = self.parseExprSubscripts(); 
    while self.tokType.unwrap().postfix && !self.canInsertSemicolon(){
{
        let tokval = self.tokVal.clone().unwrap();
         jsparse_callback_open(tokval.to_string().as_slice()); 
        let node:&mut Box<Node> = &mut self.startNodeFrom(&expr); 
        node._operator = tokval.to_string();
        node.prefix = false;
        node.argument = Some(expr);
        {
        }
        self.next();
        self.enterNode(node, "UpdateExpression");
        expr = self.finishNode(node.clone());
    }
}
    return expr;
}

fn parseExprSubscripts(&mut self) -> Box<Node> {
    let left = self.parseExprAtom();
    return self.parseSubscripts(left, false);
}

fn parseSubscripts(&mut self, base:Box<Node>, noCalls:bool) -> Box<Node> {
     jsparse_callback_open("subscripts"); 
    if (self.eat(_dot)) {
{
        let node:&mut Box<Node> = &mut self.startNodeFrom(&base); 
        self.enterNode(node, "MemberExpression");
        node.object = Some(base);
        node.property = Some(self.parseIdent(true));
        node.computed = false;
        let left = self.finishNode(node.clone());
        return self.parseSubscripts(left, noCalls);
    }
} else {if (self.eat(_bracketL)) {
{
         jsparse_callback_open("member-var-open"); 
        let node:&mut Box<Node> = &mut self.startNodeFrom(&base); 
        node.object = Some(base);
        node.property = Some(self.parseExpression(false, false));
        node.computed = true;
        self.expect(_bracketR);
         jsparse_callback_open("member-var-close"); 
        self.enterNode(node, "MemberExpression");
        let left = self.finishNode(node.clone());
        return self.parseSubscripts(left, noCalls);
    }
} else {if (!noCalls && self.eat(_parenL)) {
{
         jsparse_callback_open("call-open"); 
        let node:&mut Box<Node> = &mut self.startNodeFrom(&base); 
        self.enterNode(node, "CallExpression");
        node.callee = Some(base);
        node.arguments = self.parseExprList(_parenR, false, false);
        let left = self.finishNode(node.clone());
        return self.parseSubscripts(left, noCalls);
    }
} else {if (self.tokType.unwrap()==_bquote) {
{
        let node:&mut Box<Node> = &mut self.startNodeFrom(&base); 
        node.tag = Some(base);
        node.quasi = Some(parseTemplate());
        self.enterNode(node, "TaggedTemplateExpression");
        let left = self.finishNode(node.clone());
        return self.parseSubscripts(left, noCalls);
    }
}}}}
    return base;
}

fn parseExprAtom(&mut self) -> Box<Node> {
    match self.tokType.unwrap() {
        _this => {
            let node:&mut Box<Node> = &mut self.startNode();
            self.next();
            self.enterNode(node, "ThisExpression");
            return self.finishNode(node.clone());
        },
        _yield => {
            if (self.inGenerator) {
                return self.parseYield();
            }
        },
        _name => {
            let isliberal = self.tokType.unwrap()!=_name;
            let mut id:Box<Node> = self.parseIdent(isliberal);
        if (self.eat(_arrow)) {
        {
                let node2 = &mut self.startNodeFrom(&id);
                        return self.parseArrowExpression(node2, vec![id]);
                    }
        };
        return id;},
        _num |
        _string |
        _regexp => {
            let node:&mut Box<Node> = &mut self.startNode();
            node.value = self.tokVal.clone().unwrap();
            node.raw = slice(get_input(), self.tokStart, self.tokEnd).to_string();
            self.next();
            self.enterNode(node, "Literal");
            return self.finishNode(node.clone());
        },
        _null |
        _true |
        _false => {
            let node:&mut Box<Node> = &mut self.startNode();
            node.value = match self.tokType.unwrap().atomValue {
                ATOM_NULL => JS_NULL,
                ATOM_TRUE => JS_BOOLEAN(true),
                ATOM_FALSE => JS_BOOLEAN(false),
            };
            node.raw = self.tokType.unwrap().keyword.to_string();
            self.next();
            self.enterNode(node, "Literal");
            return self.finishNode(node.clone());
        },
        _parenL => {let mut tokStartLoc1:int = self.tokStartLoc; 
            let mut tokStart1:int = self.tokStart; 
            let mut val:Box<Node> = (*nullptr).clone();
            let mut exprList:Vec<Box<Node>> = Vec::new(); ;
        self.next();;
        if (self.options.ecmaVersion >= 6 && self.tokType.unwrap()==_for) {
        {
            let node2 = &mut self.startNode();
                        val = self.parseComprehension(node2, true);
                    }
        } else {{
                self.metParenL+= 1;
                        let mut oldParenL:int = self.metParenL; 
                        if (self.tokType.unwrap()!=_parenR) {
        {
                            val = self.parseExpression(false, false);
                            exprList = if val._type.as_slice()=="SequenceExpression" { val.expressions.clone() } else { vec![val.clone()] };
                        }
        } else {{
                            exprList = vec![];
                        }}
                        self.expect(_parenR);
                        if (self.metParenL==oldParenL && self.eat(_arrow)) {
        {
            let node2 = &mut self.startNode();
                            val = self.parseArrowExpression(node2, exprList);
                        }
        } else {{
                            if (val == (*nullptr)) {
                                let lastStart = self.lastStart;
        self.expected(Some(lastStart));
        }
                            if (self.options.ecmaVersion >= 6) {
        {
                                let mut i:uint = 0; ;
        while i < exprList.len(){{
                                    if (exprList[i]._type.as_slice()=="SpreadElement") {
        self.expected(None);
        }
        i = i + 1;
                                };
        }
                            }
        }
                        }}
                    }};
        val.start = tokStart1 as uint;
        val.end = self.lastEnd as uint;

        if (self.options.ranges) {
        {
                        val.range = vec![tokStart1, self.lastEnd];
                    }
        };
        return val;},
        _bracketL => {let node:&mut Box<Node> = &mut self.startNode(); ;
        self.next();;
        if (self.options.ecmaVersion >= 6 && self.tokType.unwrap()==_for) {
        {
                        return self.parseComprehension(node, false);
                    }
        };
        node.elements = self.parseExprList(_bracketR, true, true);;
        self.enterNode(node, "ArrayExpression");;
        return self.finishNode(node.clone());},
        _braceL => {return self.parseObj();},
        _function => {let node:&mut Box<Node> = &mut self.startNode(); ;
        self.next();;
        return self.parseFunction(node, false, false);},
        _class => {
        let left = &mut self.startNode();
            return self.parseClass(left, false);},
        _new => {return self.parseNew();},
        _ellipsis => {return self.parseSpread();},
        _bquote => {return parseTemplate();},
        _ => {self.expected(None);}}
    return (*nullptr).clone();
}

fn parseNew(&mut self) -> Box<Node> {
     jsparse_callback_open("new-open"); 
    let node:&mut Box<Node> = &mut self.startNode(); 
    self.next();
    let left = self.parseExprAtom();
    node.callee = Some(self.parseSubscripts(left, true));
     jsparse_callback_open("new-args"); 
    if (self.eat(_parenL)) {
node.arguments = self.parseExprList(_parenR, false, false);
} else {node.arguments = (*empty).clone();}
     jsparse_callback_open("new-close"); 
    self.enterNode(node, "NewExpression");
    return self.finishNode(node.clone());
}
fn parseSpread(&mut self) -> Box<Node> {
    let node:&mut Box<Node> = &mut self.startNode(); 
    self.next();
    node.argument = Some(self.parseExpression(true, false));
    self.enterNode(node, "SpreadElement");
    return self.finishNode(node.clone());
}

fn parseObj(&mut self) -> Box<Node> {
     jsparse_callback_open("object-literal"); 
    let node:&mut Box<Node> = &mut self.startNode();  
    let mut first:bool = true;  
    let mut propHash:&str = ""; 
    node.properties = vec![];
    self.next();
    while !self.eat(_braceR){
{
        if (!first) {
{
            self.expect(_comma);
            if (self.options.allowTrailingCommas && self.eat(_braceR)) {
break;
}
        }
} else {first = false;}
        let mut prop:Box<Node> = self.startNode();  
        let mut kind:String = "".to_string();  
        let mut isGenerator:bool = false; 
        if (self.options.ecmaVersion >= 6) {
{
            prop.method = false;
            prop.shorthand = false;
            isGenerator = self.eat(_star);
        }
}
         jsparse_callback_open("object-literal-key"); 
        self.parsePropertyName(&mut prop);
        if (self.eat(_colon)) {
{
             jsparse_callback_open("object-literal-value"); 
            prop.value = JS_OBJECT(self.parseExpression(true, false));
            kind = "init".to_string(); prop.kind = kind;
        }
} else {if (self.options.ecmaVersion >= 6 && self.tokType.unwrap()==_parenL) {
{
            kind = "init".to_string();
        prop.kind = kind;
            prop.method = true;
            prop.value = JS_OBJECT(self.parseMethod(isGenerator));
        }
} else {if (self.options.ecmaVersion >= 5 && !prop.computed && prop.key.clone().unwrap()._type.as_slice()=="Identifier" && (prop.key.clone().unwrap().name.as_slice()=="get" || prop.key.clone().unwrap().name.as_slice()=="set")) {
{
            if (isGenerator) {
self.expected(None);
}
    prop.kind = prop.key.clone().unwrap().name;
            kind = prop.kind.clone();
            self.parsePropertyName(&mut prop);
            prop.value = JS_OBJECT(self.parseMethod(false));
        }
} else {if (self.options.ecmaVersion >= 6 && !prop.computed && prop.key.clone().unwrap()._type.as_slice()=="Identifier") {
{
            kind = "init".to_string(); prop.kind = kind;
            //TODO prop.value = prop.key;
            prop.shorthand = true;
        }
} else {self.expected(None);}}}}
        {
        }
        self.enterNode(&mut prop, "Property");
         jsparse_callback_open("object-literal-push"); 
        node.properties.push(self.finishNode(prop));
    }
}
    self.enterNode(node, "ObjectExpression");
    return self.finishNode(node.clone());
}

fn parsePropertyName(&mut self, prop:&mut Box<Node>) -> int {
    if (self.options.ecmaVersion >= 6) {
{
        if (self.eat(_bracketL)) {
{
            prop.computed = true;
            prop.key = Some(self.parseExpression(false, false));
            self.expect(_bracketR);
            return 0;
        }
} else {{
            prop.computed = false;
        }}
    }
}
    prop.key = Some(if self.tokType.unwrap()==_num || self.tokType.unwrap()==_string { self.parseExprAtom() } else { self.parseIdent(true) });
    return 0;
}

fn initFunction<'a>(&'a mut self, node:&'a mut Box<Node>) -> int {
    node.id = None;
    node.params = vec![];
    if (self.options.ecmaVersion >= 6) {
        node.defaults = vec![];
        node.rest = None;
        node.generator = false;
    }
    return 0;
}

fn parseFunction<'a>(&'a mut self, node:&'a mut Box<Node>, isStatement:bool, allowExpressionBody:bool) -> Box<Node> {
    self.initFunction(node);
    if (self.options.ecmaVersion >= 6) {
        node.generator = self.eat(_star);
    }
    if (isStatement || self.tokType.unwrap()==_name) {
        node.id = Some(self.parseIdent(false));
    }
    jsparse_callback_open("function-params"); 
    self.parseFunctionParams(node);
    jsparse_callback_open("function-body"); 
    self.parseFunctionBody(node, allowExpressionBody);
    self.enterNode(node, if isStatement { "FunctionDeclaration" } else { "FunctionExpression" });
    return self.finishNode(node.clone());
}

fn parseMethod(&mut self, isGenerator:bool) -> Box<Node> {
    let node:&mut Box<Node> = &mut self.startNode(); 
    self.initFunction(node);
    self.parseFunctionParams(node);
    let mut allowExpressionBody:bool = false; 
    if (self.options.ecmaVersion >= 6) {
        node.generator = isGenerator;
        allowExpressionBody = true;
    } else {
        allowExpressionBody = false;
    }
    self.parseFunctionBody(node, allowExpressionBody);
    self.enterNode(node, "FunctionExpression");
    return self.finishNode(node.clone());
}

fn parseArrowExpression<'a>(&'a mut self, node:&'a mut Box<Node>, mut params:Vec<Box<Node>>) -> Box<Node> {
    self.initFunction(node);
    let mut hasDefaults:bool = false; 
    let mut i:uint = 0;  let mut lastI:uint = params.len() - 1; ;
while i <= lastI{
        let mut param:Box<Node> = params[i].clone();
        if (param._type.as_slice()=="AssignmentExpression" && param._operator.as_slice()=="=") {
            hasDefaults = true;
            params[i] = param.left.clone().unwrap();
            node.defaults.push(param.right.clone().unwrap());
        } else {
            toAssignable(param.clone(), i==lastI, true);
            node.defaults.push((*nullptr).clone());
            if (param._type.as_slice()=="SpreadElement") {
                params.pop();
                node.rest = param.argument;
                break;
            }
        }
    }
    node.params = params;
    if (!hasDefaults) {
        node.defaults = vec![];
    }
    self.parseFunctionBody(node, true);
    self.enterNode(node, "ArrowFunctionExpression");
    return self.finishNode(node.clone());
}

fn parseFunctionParams<'a>(&'a mut self, node:&'a mut Box<Node>) -> int {
    let mut defaults:Vec<Box<Node>> = vec![];
    let mut hasDefaults:bool = false; 
    self.expect(_parenL);
loop{{
        if (self.eat(_parenR)) {
{
            break;
        }
} else {if (self.options.ecmaVersion >= 6 && self.eat(_ellipsis)) {
{
            node.rest = Some(toAssignable(self.parseExprAtom(), false, true));
            self.checkSpreadAssign(&mut node.rest.clone().unwrap());
            self.expect(_parenR);
            break;
        }
} else {{
             jsparse_callback_open("function-param"); 
            node.params.push(if self.options.ecmaVersion >= 6 { toAssignable(self.parseExprAtom(), false, true) } else { self.parseIdent(false) });
            if (self.options.ecmaVersion >= 6 && self.tokType.unwrap()==_eq) {
{
                self.next();
                hasDefaults = true;
                defaults.push(self.parseExpression(true, false));
            }
}
            if (!self.eat(_comma)) {
{
                self.expect(_parenR);
                break;
            }
}
        }}}
    };
}
    if (hasDefaults) {
node.defaults = defaults;
}
return 0;
}
fn parseFunctionBody<'a>(&'a mut self, node:&'a mut Box<Node>, allowExpression:bool) -> int {
    let mut isExpression:bool = allowExpression && self.tokType.unwrap()!=_braceL; 
    if (isExpression) {
{
        node.body = Some(self.parseExpression(true, false));
        node.isExpression = true;
    }
} else {{
        let mut oldInFunc:bool = self.inFunction;  let mut oldInGen:bool = self.inGenerator;
        let mut oldLabels:Vec<label_t> = self.labels.clone(); 
        self.inFunction = true;
        self.inGenerator = node.generator;
        self.labels = vec![];
        node.body = Some(self.parseBlock(true));
        node.isExpression = false;
        self.inFunction = oldInFunc;
        self.inGenerator = oldInGen;
        self.labels = oldLabels;
    }}
    if (self._strict || !isExpression && node.body.clone().unwrap().bodylist.len() > 0 && isUseStrict(&mut node.body.clone().unwrap().bodylist[0])) {
{
        let mut nameHash:&str = ""; 
        if (node.id.is_some()) {
{
        }
}
        let mut i:uint = 0; ;
while i < node.params.len(){{
        };
}
        if (node.rest.is_some()) {
{
        }
}
i = i + 1;
    }
}
return 0;
}
fn parseClass<'a>(&'a mut self, node:&'a mut Box<Node>, isStatement:bool) -> Box<Node> {
    self.next();
    if (self.tokType.unwrap()==_name) {
{
        node.id = Some(self.parseIdent(false));
    }
} else {if (isStatement) {
{
        self.expected(None);
    }
} else {{
        node.id = None;
    }}}
    node.superClass = if self.eat(_extends) { Some(self.parseExpression(false, false)) } else { None };
    let mut classBody:Box<Node> = self.startNode();  let mut methodHash:&str = "";  let mut staticMethodHash:&str = ""; 
    classBody.bodylist = vec![];
    self.expect(_braceL);
    while !self.eat(_braceR){
{
        let mut method:Box<Node> = self.startNode(); 
        if (self.tokType.unwrap()==_name && self.tokVal.clone().unwrap().to_string().as_slice()=="static") {
{
            self.next();
            method._static = true;
        }
} else {{
            method._static = false;
        }}
        let mut isGenerator:bool = self.eat(_star); 
        self.parsePropertyName(&mut method);
        if (self.tokType.unwrap()==_name && !method.computed && method.key.clone().unwrap()._type.as_slice()=="Identifier" && (method.key.clone().unwrap().name.as_slice()=="get" || method.key.clone().unwrap().name.as_slice()=="set")) {
{
            if (isGenerator) {
self.expected(None);
}
            method.kind = method.key.clone().unwrap().name.to_string();
            self.parsePropertyName(&mut method);
        }
} else {{
            method.kind = "".to_string();
        }}
        //TODO method.value = parseMethod(isGenerator);
        {
        }
        self.enterNode(&mut method, "MethodDefinition");
        classBody.bodylist.push(self.finishNode(method));
        self.eat(_semi);
    }
}
    self.enterNode(&mut classBody, "ClassBody");
    node.body = Some(self.finishNode(classBody));
    self.enterNode(node, if isStatement { "ClassDeclaration" } else { "ClassExpression" });
    return self.finishNode(node.clone());
}
fn parseExprList(&mut self, close:keyword_t, allowTrailingComma:bool, allowEmpty:bool) -> Vec<Box<Node>> {
    let mut elts:Vec<Box<Node>> = vec![];  let mut first:bool = true; 
    while !self.eat(close) {
        if (!first) {
            self.expect(_comma);
            jsparse_callback_open("parseExprList-next"); 
            if (allowTrailingComma && self.options.allowTrailingCommas && self.eat(close)) {
                break;
            }
        } else {
            first = false;
        }
        if (allowEmpty && self.tokType.unwrap()==_comma) {
            elts.push((*nullptr).clone());
        } else {
            elts.push(self.parseExpression(true, false));
        }
    }
    return elts;
}

fn parseIdent(&mut self, mut liberal:bool) -> Box<Node> {
    let node:&mut Box<Node> = &mut self.startNode(); 
    if liberal && self.options.forbidReserved == "everywhere" {
        liberal = false;
    }
    if (self.tokType.unwrap() == _name) {
        let tokval = self.tokVal.clone().unwrap();
        if (!liberal && (self.options.forbidReserved.len() > 0 && (if self.options.ecmaVersion==3 { isReservedWord3 } else { isReservedWord5 })(tokval.to_string().as_slice()) || self._strict && isStrictReservedWord(tokval.to_string().as_slice())) && indexOf(slice(get_input(), self.tokStart, self.tokEnd), "\\", 0) == -1) {
            raise(self.tokStart, ("The keyword '".to_string() + tokval.to_string() + "' is reserved").as_slice());
        }
        node.name = tokval.to_string();
    } else {
        if (liberal && self.tokType.unwrap().keyword.len() > 0) {
            node.name = self.tokType.unwrap().keyword.to_string();
        } else {
            self.expected(None);
        }
    }
    self.tokRegexpAllowed = false;
    self.next();
    self.enterNode(node, "Identifier");
    return self.finishNode(node.clone());
}

fn parseExport<'a>(&'a mut self, node:&'a mut Box<Node>) -> Box<Node> {
    self.next();
    if (self.tokType.unwrap()==_var || self.tokType.unwrap()==_const || self.tokType.unwrap()==_let || self.tokType.unwrap()==_function || self.tokType.unwrap()==_class) {
{
        node.declaration = Some(self.parseStatement());
        node._default = false;
        node.specifiers = vec![];
        node.source = None;
    }
} else {if (self.eat(_default)) {
{
        node.declaration = Some(self.parseExpression(true, false));
        node._default = true;
        node.specifiers = vec![];
        node.source = None;
        self.semicolon();
    }
} else {{
        let mut isBatch:bool = self.tokType.unwrap()==_star; 
        node.declaration = None;
        node._default = false;
        node.specifiers = self.parseExportSpecifiers();
        let tokval = self.tokVal.clone().unwrap();
        if (self.tokType.unwrap()==_name && tokval.to_string().as_slice()=="from") {
{
            self.next();
            if (self.tokType.unwrap()!=_string) {
{
                self.expected(None);
            }
}
            node.source = Some(self.parseExprAtom());
        }
} else {{
            if (isBatch) {
self.expected(None);
}
            node.source = None;
        }}
    }}}
    self.enterNode(node, "ExportDeclaration");
    return self.finishNode(node.clone());
}
fn parseExportSpecifiers(&mut self) -> Vec<Box<Node>> {
    let mut nodes:Vec<Box<Node>> = vec![];  let mut first:bool = true; 
    if (self.tokType.unwrap()==_star) {
{
        let node:&mut Box<Node> = &mut self.startNode(); 
        self.next();
        self.enterNode(node, "ExportBatchSpecifier");
        nodes.push(self.finishNode(node.clone()));
    }
} else {{
        self.expect(_braceL);
        while !self.eat(_braceR){
{
            if (!first) {
{
                self.expect(_comma);
                if (self.options.allowTrailingCommas && self.eat(_braceR)) {
break;
}
            }
} else {first = false;}
            let node:&mut Box<Node> = &mut self.startNode(); 
            node.id = Some(self.parseIdent(false));
            let tokval = self.tokVal.clone().unwrap();
            if (self.tokType.unwrap()==_name && tokval.to_string().as_slice()=="as") {
{
                self.next();
            }
} else {{
                node.name = null.to_string();
            }}
            self.enterNode(node, "ExportSpecifier");
            nodes.push(self.finishNode(node.clone()));
        }
}
    }}
    return nodes;
}
fn parseImport<'a>(&'a mut self, node:&'a mut Box<Node>) -> Box<Node> {
    self.next();
    if (self.tokType.unwrap()==_string) {
{
        node.specifiers = vec![];
        node.source = Some(self.parseExprAtom());
        node.kind = null.to_string();
    }
} else {{
        node.specifiers = self.parseImportSpecifiers();
        let tokval = self.tokVal.clone().unwrap();
        if (self.tokType.unwrap()!=_name || tokval.to_string().as_slice()!="from") {
self.expected(None);
}
        self.next();
        if (self.tokType.unwrap()!=_string) {
{
            self.expected(None);
        }
}
        node.source = Some(self.parseExprAtom());
        node.kind = (if node.specifiers[0]._default { "default" } else { "named" }).to_string();
    }}
    self.enterNode(node, "ImportDeclaration");
    return self.finishNode(node.clone());
}
fn parseImportSpecifiers(&mut self) -> Vec<Box<Node>> {
    let mut nodes:Vec<Box<Node>> = vec![];  let mut first:bool = true; 
    if (self.tokType.unwrap()==_star) {
{
        let node:&mut Box<Node> = &mut self.startNode(); 
        self.next();
        let tokval = self.tokVal.clone().unwrap();
        if (self.tokType.unwrap()!=_name || tokval.to_string().as_slice()!="as") {
self.expected(None);
}
        self.next();
        {
        }
        self.enterNode(node, "ImportBatchSpecifier");
        nodes.push(self.finishNode(node.clone()));
        return nodes;
    }
}
    if (self.tokType.unwrap()==_name) {
{
        let node:&mut Box<Node> = &mut self.startNode(); 
        node.id = Some(self.parseIdent(false));
        {
        }
        node.name = null.to_string();
        node._default = true;
        self.enterNode(node, "ImportSpecifier");
        nodes.push(self.finishNode(node.clone()));
        if (!self.eat(_comma)) {
return nodes;
}
    }
}
    self.expect(_braceL);
    while !self.eat(_braceR){
{
        if (!first) {
{
            self.expect(_comma);
            if (self.options.allowTrailingCommas && self.eat(_braceR)) {
break;
}
        }
} else {first = false;}
        let node:&mut Box<Node> = &mut self.startNode(); 
        node.id = Some(self.parseIdent(true));
        let tokval = self.tokVal.clone().unwrap();
        if (self.tokType.unwrap()==_name && tokval.to_string().as_slice()=="as") {
{
            self.next();
        }
} else {{
            node.name = null.to_string();
        }}
        {
        }
        node._default = false;
        self.enterNode(node, "ImportSpecifier");
        nodes.push(self.finishNode(node.clone()));
    }
}
    return nodes;
}
fn parseYield(&mut self) -> Box<Node> {
    let node:&mut Box<Node> = &mut self.startNode(); 
    self.next();
    if (self.eat(_semi) || self.canInsertSemicolon()) {
{
        node.delegate = false;
        node.argument = None;
    }
} else {{
        node.delegate = self.eat(_star);
        node.argument = Some(self.parseExpression(true, false));
    }}
    self.enterNode(node, "YieldExpression");
    return self.finishNode(node.clone());
}
fn parseComprehension<'a>(&'a mut self, node:&'a mut Box<Node>, isGenerator:bool) -> Box<Node> {
    node.blocks = vec![];
    while self.tokType.unwrap()==_for{
{
        let mut block:Box<Node> = self.startNode(); 
        self.next();
        self.expect(_parenL);
        block.left = Some(toAssignable(self.parseExprAtom(), false, false));
        {
        }
        let tokval = self.tokVal.clone().unwrap();
        if (self.tokType.unwrap()!=_name || tokval.to_string().as_slice()!="of") {
self.expected(None);
}
        self.next();
        block.of = true;
        block.right = Some(self.parseExpression(false, false));
        self.expect(_parenR);
        self.enterNode(&mut block, "ComprehensionBlock");
        node.blocks.push(self.finishNode(block));
    }
}
    node.filter = if self.eat(_if) { Some(self.parseParenExpression()) } else { None };
    node.body = Some(self.parseExpression(false, false));
    self.expect(if isGenerator { _parenR } else { _bracketR });
    node.generator = isGenerator;
    self.enterNode(node, "ComprehensionExpression");
    return self.finishNode(node.clone());
}
}
