use std;
use std::f64;

pub struct options_t {
	pub ecmaVersion:int,
	pub strictSemicolons:bool,
	pub allowTrailingCommas:bool,
	pub forbidReserved:&'static str,
	pub allowReturnOutsideFunction:bool,
	pub locations:bool,
	// void (*onComment)();
	pub ranges:bool,
	pub program:Option<Box<Node>>,
	pub sourceFile:&'static str,
	pub directSourceFile:&'static str,
}

#[deriving(Clone)]
pub struct Node {
	pub _type:String,
	pub start:int,
	pub end:int,
	// SourceLocation* loc;

	pub sourceFile:String,
	pub range:Vec<int>,
	pub body:Option<Box<Node>>,
	pub bodylist:Vec<Box<Node>>,
	pub label:Option<Box<Node>>,
	pub test:Option<Box<Node>>,
	pub declaration:Option<Box<Node>>,
	pub source:Option<Box<Node>>,
	pub specifiers:Vec<Box<Node>>,
	pub consequent:Option<Box<Node>>,
	pub consequents:Vec<Box<Node>>,
	pub defaults:Vec<Box<Node>>,
	pub alternate:Option<Box<Node>>,
	pub argument:Option<Box<Node>>,
	pub discriminant:Option<Box<Node>>,
	pub cases:Vec<Box<Node>>,
	pub block:Option<Box<Node>>,
	pub handler:Option<Box<Node>>,
	pub guardedHandlers:Vec<Box<Node>>,
	pub finalizer:Option<Box<Node>>,
	pub object:Option<Box<Node>>,
	pub expression:Option<Box<Node>>,
	pub init:Option<Box<Node>>,
	pub update:Option<Box<Node>>,
	pub left:Option<Box<Node>>,
	pub right:Option<Box<Node>>,
	pub declarations:Vec<Box<Node>>,
	pub kind:String,
	pub expressions:Vec<Box<Node>>,
	pub prefix:bool,
	pub property:Option<Box<Node>>,
	pub computed:bool,
	pub callee:Option<Box<Node>>,
	pub arguments:Vec<Box<Node>>,
	pub key:Option<Box<Node>>,
	pub value:js_any_type,
	pub raw:String,
	pub elements:Vec<Box<Node>>,
	pub properties:Vec<Box<Node>>,
	pub id:Option<Box<Node>>,
	pub param:Option<Box<Node>>,
	pub params:Vec<Box<Node>>,
	pub blocks:Vec<Box<Node>>,
	pub rest:Option<Box<Node>>,
	pub guard:Option<Box<Node>>,
	pub name:String,
	pub generator:bool,
	pub of:bool,
	pub quasi:Option<Box<Node>>,
	pub quasis:Vec<Box<Node>>,
	pub tag:Option<Box<Node>>,
	pub delegate:bool,
	pub _default:bool,
	pub _static:bool,
	pub _operator:String,
	pub filter:Option<Box<Node>>,
	pub method:bool,
	pub tail:bool,
	pub shorthand:bool,
	pub superClass:Option<Box<Node>>,
}

impl PartialEq for Node {
	fn eq(&self, other: &Node) -> bool {
    	self == other
    }
}

pub static null:&'static str = "";

#[deriving(Clone)]
pub struct keyword_t {
	pub _id:int,
	pub atomValue:ATOM_VALUE,
	pub beforeExpr:bool,
	pub binop:int,
	pub isAssign:bool,
	pub isLoop:bool,
	pub isUpdate:bool,
	pub keyword:&'static str,
	pub postfix:bool,
	pub prefix:bool,
	pub _type:&'static str,
}

impl PartialEq for keyword_t {
    fn eq(&self, other: &keyword_t) -> bool {
    	self._id == other._id
    }
}

#[deriving(Clone)]
pub enum ATOM_VALUE {
	ATOM_NULL,
	ATOM_TRUE,
	ATOM_FALSE,
}

#[deriving(Clone)]
pub struct label_t {
    pub kind:String,
    pub name:String,
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
}



lazy_static! {
	pub static ref nullptr:Box<Node> = box Node {
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
}

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

impl js_any_type {
	pub fn to_string(&self) -> String {
		match (*self) {
			JS_STRING(ref s) => s.clone(),
			_ => "".to_string()
		}
	}
}

// what

pub fn setOptions (arg:options_t) {
}

pub fn isUseStrict (arg:&mut Box<Node>) -> bool {
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

pub struct TokTypes {
	pub bracketL: keyword_t,
	pub bracketR: keyword_t,
	pub braceL: keyword_t,
	pub braceR: keyword_t,
	pub parenL: keyword_t,
	pub parenR: keyword_t,
	pub comma: keyword_t,
	pub semi: keyword_t,
	pub colon: keyword_t,
	pub dot: keyword_t,
	pub ellipsis: keyword_t,
	pub question: keyword_t,
	pub slash: keyword_t,
	pub eq: keyword_t,
	pub name: keyword_t,
	pub eof: keyword_t,
	pub num: keyword_t,
	pub regexp: keyword_t,
	pub string: keyword_t,
	pub arrow: keyword_t,
	pub bquote: keyword_t,
	pub dollarBraceL: keyword_t
}
