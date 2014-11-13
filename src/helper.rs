use std;

pub struct options_t {
	ecmaVersion:int,
	strictSemicolons:bool,
	allowTrailingCommas:bool,
	forbidReserved:&'static str,
	allowReturnOutsideFunction:bool,
	locations:bool,
	// void (*onComment)();
	ranges:bool,
	program:Option<Box<Node>>,
	sourceFile:&'static str,
	directSourceFile:&'static str,
}

#[deriving(Clone)]
pub struct Node {
	_type:String,
	start:int,
	end:int,
	// SourceLocation* loc;

	sourceFile:String,
	range:Vec<int>,
	body:Option<Box<Node>>,
	bodylist:Vec<Box<Node>>,
	label:Option<Box<Node>>,
	test:Option<Box<Node>>,
	declaration:Option<Box<Node>>,
	source:Option<Box<Node>>,
	specifiers:Vec<Box<Node>>,
	consequent:Option<Box<Node>>,
	consequents:Vec<Box<Node>>,
	defaults:Vec<Box<Node>>,
	alternate:Option<Box<Node>>,
	argument:Option<Box<Node>>,
	discriminant:Option<Box<Node>>,
	cases:Vec<Box<Node>>,
	block:Option<Box<Node>>,
	handler:Option<Box<Node>>,
	guardedHandlers:Vec<Box<Node>>,
	finalizer:Option<Box<Node>>,
	object:Option<Box<Node>>,
	expression:Option<Box<Node>>,
	init:Option<Box<Node>>,
	update:Option<Box<Node>>,
	left:Option<Box<Node>>,
	right:Option<Box<Node>>,
	declarations:Vec<Box<Node>>,
	kind:String,
	expressions:Vec<Box<Node>>,
	prefix:bool,
	property:Option<Box<Node>>,
	computed:bool,
	callee:Option<Box<Node>>,
	arguments:Vec<Box<Node>>,
	key:Option<Box<Node>>,
	value:js_any_type,
	raw:String,
	elements:Vec<Box<Node>>,
	properties:Vec<Box<Node>>,
	id:Option<Box<Node>>,
	param:Option<Box<Node>>,
	params:Vec<Box<Node>>,
	blocks:Vec<Box<Node>>,
	rest:Option<Box<Node>>,
	guard:Option<Box<Node>>,
	name:String,
	generator:bool,
	of:bool,
	quasi:Option<Box<Node>>,
	quasis:Vec<Box<Node>>,
	tag:Option<Box<Node>>,
	delegate:bool,
	_default:bool,
	_static:bool,
	_operator:String,
	filter:Option<Box<Node>>,
	method:bool,
	tail:bool,
	shorthand:bool,
	superClass:Option<Box<Node>>,
}

impl PartialEq for Node {
	fn eq(&self, other: &Node) -> bool {
    	self == other
    }
}

pub static null:&'static str = "";

pub struct keyword_t {
	_id:int,
	atomValue:ATOM_VALUE,
	beforeExpr:bool,
	binop:int,
	isAssign:bool,
	isLoop:bool,
	isUpdate:bool,
	keyword:&'static str,
	postfix:bool,
	prefix:bool,
	_type:&'static str,
}

impl PartialEq for keyword_t {
    fn eq(&self, other: &keyword_t) -> bool {
    	self._id == other._id
    }
}

pub enum ATOM_VALUE {
	ATOM_NULL,
	ATOM_TRUE,
	ATOM_FALSE,
}

pub struct label_t {
    kind:String,
    name:String,
}

pub fn toString (arg:&str) -> &str {
	return arg;
}

pub fn fromCharCode (i:u32) -> String {
	return String::from_char(1, std::char::from_u32(i).unwrap());
}

pub fn fromCharCode2 (i:u32, j:u32) -> String {
	return String::from_chars([std::char::from_u32(i).unwrap(), std::char::from_u32(j).unwrap()]);
}

pub fn push (nodes:Vec<Box<Node>>, node:Box<Node>) {
	nodes.push(node);
}

pub fn charCodeAt(arg:&str, n:uint) -> int {
	return arg.utf16_units().nth(n).unwrap() as int;
}

pub fn convert_to_Node_C (arg:Box<Node>) -> &'static str {
	return "";
}

pub fn jsparse_callback_close (arg:&str) {
}



pub static nullptr:Box<Node> = box Node {
	_type: "".to_string(),
	start: 1,
	end: -1,

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
};

pub fn ISNULL(arg:&str) -> bool {
	return true;
}

pub fn jsparse_callback_open (arg:&str) {
}

pub fn raise (start:int, message:&str){
	// printf("ERROR: %s %d\n", message.c_str(), start);
	// exit(1);
}

pub fn indexOf(arg:&str, needle:&str, start:int) -> int {
	return -1;
}

#[deriving(Clone)]
pub enum js_any_type {
	JS_NULL,
	JS_DOUBLE(f64),
	JS_STRING(String),
	JS_BOOLEAN(bool),
	JS_OBJECT(Box<Node>),
	JS_REGEXP(String),
}

pub impl js_any_type {
	pub fn to_string(&self) -> String {
		match (*self) {
			JS_STRING(s) => s,
			_ => "".to_string()
		}
	}
}

// what

pub fn setOptions (arg:options_t) {
}

pub fn isUseStrict (arg:Box<Node>) -> bool {
	false
}

pub fn toAssignable(node:Box<Node>, a:bool, b:bool) -> Box<Node>
{
	node
}

pub fn slice (arr:&str, start:int, end:int) -> &str {
	// TODO check negative indices
	// return arr.substr(start,end - start);
	arr
}

pub fn exec (regex: |arg:&str| -> bool, val:&str ) -> Option<Box<Vec<String>>> {
	return Some(box vec!["hello".to_string(), "world".to_string()]);
}

pub fn ISNOTNULL (arg:int) -> bool {
	return false;
}

pub fn isNaN (arg:f64) -> bool {
	return arg.is_nan();
}

pub fn parseInt (arg:&str, base:int) -> int {
	return 0;
}

pub fn parseFloat (arg:&str) -> f64 {
	return 0f64;
}

pub fn test (regex: |arg:&str| -> bool, val:&str ) -> bool {
	return false;
}

pub fn checkLVal(arg:Box<Node>) {

}

pub fn parseTemplate() -> Box<Node> {
	return nullptr.clone()
}

pub struct thisval {
	line:uint,
	column:uint,
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

static inf: f64 = Float::infinity();
static nan: f64 = Float::nan();

pub static NaN:f64 = nan;
pub static Infinity:f64 = inf;

pub struct TokTypes {
	bracketL: keyword_t, bracketR: keyword_t, braceL: keyword_t, braceR: keyword_t, parenL: keyword_t, parenR: keyword_t, comma: keyword_t, semi: keyword_t, colon: keyword_t, dot: keyword_t, ellipsis: keyword_t, question: keyword_t, slash: keyword_t, eq: keyword_t, name: keyword_t, eof: keyword_t, num: keyword_t, regexp: keyword_t, string: keyword_t, arrow: keyword_t, bquote: keyword_t, dollarBraceL: keyword_t
}
