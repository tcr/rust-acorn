use std;
use std::f64;
use std::fmt;
use std::io;
use serialize::{Encodable, Encoder};
use serialize::json::ToJson;
use serialize::json;

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

// trait RootNode { }

// trait EncodableNode<S, E> : RootNode + Encodable<S, E> { }


// impl<>

#[deriving(Clone, Show)]
pub struct Node {
    pub _type:String,
    pub start:uint,
    pub end:uint,
    pub loc:Option<SourceLocation>,

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
    pub isExpression:bool,
    pub superClass:Option<Box<Node>>,
}

#[deriving(Encodable)]
pub struct DummyNode;

#[deriving(Encodable)]
pub struct ProgramNode {
    pub _type:&'static str,
    pub loc:Option<SourceLocation>,

    pub body:Vec<Box<Node>>,
}

#[deriving(Encodable)]
pub struct FunctionDeclarationNode {
    pub _type:&'static str,
    pub loc:Option<SourceLocation>,

    pub id:Option<Box<Node>>,
    pub params:Vec<Box<Node>>,
    pub body:Option<Box<Node>>,
    pub expression:bool,
}

#[deriving(Encodable)]
pub struct FunctionExpressionNode {
    pub _type:&'static str,
    pub loc:Option<SourceLocation>,

    pub id:Option<Box<Node>>,
    pub params:Vec<Box<Node>>,
    pub body:Option<Box<Node>>,
    pub expression:bool,
}

#[deriving(Encodable)]
pub struct ExpressionStatementNode {
    pub _type:&'static str,
    pub loc:Option<SourceLocation>,

    pub expression:Option<Box<Node>>,
}

#[deriving(Encodable)]
pub struct WhileStatementNode {
    pub _type:&'static str,
    pub loc:Option<SourceLocation>,

    pub test:Option<Box<Node>>,
    pub body:Option<Box<Node>>,
}

#[deriving(Encodable)]
pub struct BlockStatementNode {
    pub _type:&'static str,
    pub loc:Option<SourceLocation>,

    pub body:Vec<Box<Node>>,
}

#[deriving(Encodable)]
pub struct MemberExpressionNode {
    pub _type:&'static str,
    pub loc:Option<SourceLocation>,

    pub object:Option<Box<Node>>,
    pub property:Option<Box<Node>>,
    pub computed:bool,
}

#[deriving(Encodable)]
pub struct ConditionalExpressionNode {
    pub _type:&'static str,
    pub loc:Option<SourceLocation>,

    pub test:Option<Box<Node>>,
    pub consequent:Option<Box<Node>>,
    pub alternate:Option<Box<Node>>,
}

#[deriving(Encodable)]
pub struct BinaryExpressionNode {
    pub _type:&'static str,
    pub loc:Option<SourceLocation>,
    
    pub left:Option<Box<Node>>,
    pub operator:String,
    pub right:Option<Box<Node>>,
}

#[deriving(Encodable)]
pub struct CallExpressionNode {
    pub _type:&'static str,
    pub loc:Option<SourceLocation>,
    
    pub callee:Option<Box<Node>>,
    pub arguments:Vec<Box<Node>>,
}

#[deriving(Encodable)]
pub struct IdentifierNode {
    pub _type:&'static str,
    pub loc:Option<SourceLocation>,
    
    pub name:String,
}

#[deriving(Encodable)]
pub struct LiteralBooleanNode {
    pub _type:&'static str,
    pub loc:Option<SourceLocation>,
    
    pub value:bool,
    pub raw:String,
}

#[deriving(Encodable)]
pub struct LiteralStringNode {
    pub _type:&'static str,
    pub loc:Option<SourceLocation>,
    
    pub value:String,
    pub raw:String,
}

#[deriving(Encodable)]
pub struct ThisExpressionNode {
    pub _type:&'static str,
    pub loc:Option<SourceLocation>,
}

#[deriving(Encodable)]
pub struct LiteralNumberNode {
    pub _type:&'static str,
    pub loc:Option<SourceLocation>,
    
    pub value:f64,
    pub raw:String,
}

#[deriving(Encodable)]
pub struct LiteralRegexpNode {
    pub _type:&'static str,
    pub loc:Option<SourceLocation>,
    
    pub value:DummyNode,
    pub raw:String,
}

#[deriving(Encodable)]
pub struct LiteralNullNode {
    pub _type:&'static str,
    pub loc:Option<SourceLocation>,
    
    pub value:Option<bool>,
    pub raw:String,
}

#[deriving(Encodable)]
pub struct VariableDeclarationNode {
    pub _type:&'static str,
    pub loc:Option<SourceLocation>,
    
    pub declarations:Vec<Box<Node>>,
    pub kind:String,
}

#[deriving(Encodable)]
pub struct VariableDeclaratorNode {
    pub _type:&'static str,
    pub loc:Option<SourceLocation>,
    
    pub id:Option<Box<Node>>,
    pub init:Option<Box<Node>>,
}

impl <S: Encoder<E>, E> Encodable<S, E> for Node {
    fn encode(&self, encoder: &mut S) -> Result<(), E> {
        match self._type.as_slice() {
            "Program" => {
                ProgramNode {
                    _type: "Program",
                    loc: self.loc.clone(),

                    body: self.bodylist.clone()
                }.encode(encoder)
            },
            "FunctionDeclaration" => {
                FunctionDeclarationNode {
                    _type: "FunctionDeclaration",
                    loc: self.loc.clone(),

                    id: self.id.clone(),
                    params: self.params.clone(),
                    body: self.body.clone(),
                    expression: self.isExpression,
                }.encode(encoder)
            },
            "FunctionExpression" => {
                FunctionExpressionNode {
                    _type: "FunctionExpression",
                    loc: self.loc.clone(),

                    id: self.id.clone(),
                    params: self.params.clone(),
                    body: self.body.clone(),
                    expression: self.isExpression,
                }.encode(encoder)
            },
            "ThisExpression" => {
                ThisExpressionNode {
                    _type: "ThisExpression",
                    loc: self.loc.clone(),
                }.encode(encoder)
            },
            "ConditionalExpression" => {
                ConditionalExpressionNode {
                    _type: "ConditionalExpression",
                    loc: self.loc.clone(),

                    test: self.test.clone(),
                    consequent: self.consequent.clone(),
                    alternate: self.alternate.clone(),
                }.encode(encoder)
            },
            "VariableDeclaration" => {
                VariableDeclarationNode {
                    _type: "VariableDeclaration",
                    loc: self.loc.clone(),

                    declarations: self.declarations.clone(),
                    kind: self.kind.clone(),
                }.encode(encoder)
            },
            "VariableDeclarator" => {
                VariableDeclaratorNode {
                    _type: "VariableDeclarator",
                    loc: self.loc.clone(),

                    id: self.id.clone(),
                    init: self.init.clone(),
                }.encode(encoder)
            },
            "BlockStatement" => {
                BlockStatementNode {
                    _type: "BlockStatement",
                    loc: self.loc.clone(),

                    body: self.bodylist.clone(),
                }.encode(encoder)
            },
            "ExpressionStatement" => {
                ExpressionStatementNode {
                    _type: "ExpressionStatement",
                    loc: self.loc.clone(),

                    expression: self.expression.clone(),
                }.encode(encoder)
            },
            "WhileStatement" => {
                WhileStatementNode {
                    _type: "WhileStatement",
                    loc: self.loc.clone(),

                    test: self.test.clone(),
                    body: self.body.clone(),
                }.encode(encoder)
            },
            "MemberExpression" => {
                MemberExpressionNode {
                    _type: "MemberExpression",
                    loc: self.loc.clone(),

                    object: self.object.clone(),
                    property: self.property.clone(),
                    computed: self.computed,
                }.encode(encoder)
            },
            "BinaryExpression" => {
                BinaryExpressionNode {
                    _type: "BinaryExpression",
                    loc: self.loc.clone(),

                    left: self.left.clone(),
                    operator: self._operator.clone(),
                    right: self.right.clone(),
                }.encode(encoder)
            },
            "CallExpression" => {
                CallExpressionNode {
                    _type: "CallExpression",
                    loc: self.loc.clone(),

                    callee: self.callee.clone(),
                    arguments: self.arguments.clone(),
                }.encode(encoder)
            },
            "Identifier" => {
                IdentifierNode {
                    _type: "Identifier",
                    loc: self.loc.clone(),

                    name: self.name.clone(),
                }.encode(encoder)
            },
            "Literal" => {
                match self.value.clone() {
                    JS_STRING(s) => LiteralStringNode {
                        _type: "Literal",
                        loc: self.loc.clone(),

                        value: s.clone(),
                        raw: self.raw.clone(),
                    }.encode(encoder),
                    JS_REGEXP(s) => LiteralRegexpNode {
                        _type: "Literal",
                        loc: self.loc.clone(),

                        value: DummyNode,
                        raw: self.raw.clone(),
                    }.encode(encoder),
                    JS_DOUBLE(s) => LiteralNumberNode {
                        _type: "Literal",
                        loc: self.loc.clone(),

                        value: s,
                        raw: self.raw.clone(),
                    }.encode(encoder),
                    JS_BOOLEAN(s) => LiteralBooleanNode {
                        _type: "Literal",
                        loc: self.loc.clone(),

                        value: s,
                        raw: self.raw.clone(),
                    }.encode(encoder),
                    JS_NULL => LiteralNullNode {
                        _type: "Literal",
                        loc: self.loc.clone(),

                        value: None,
                        raw: self.raw.clone(),
                    }.encode(encoder),
                    _ => {
                        writeln!(io::stderr(), "UNSUPPORTED {}", self.value);
                        encoder.emit_nil()
                    },
                }
            },
            _ => {
                writeln!(io::stderr(), " TODO: {}", self._type);
                encoder.emit_struct("empty", 0, |encoder| {
                  Ok(())
                })
            }
        }
    }
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

impl fmt::Show for keyword_t {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "[keyword {}, '{}']", self._id, self._type)
    }
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

#[deriving(Clone, Show, Encodable)]
pub struct label_t {
    pub kind:String,
    pub name:String,
}

#[deriving(Clone, Show, Encodable)]
pub struct Position {
    pub line:uint,
    pub column:uint,
}

#[deriving(Clone, Show, Encodable)]
pub struct SourceLocation {
    pub start:Position,
    pub end:Position,
}


lazy_static! {
    pub static ref nullptr:Box<Node> = box Node {
        _type: "".to_string(),
        start: 1,
        end: -1,
        loc: None,

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
        isExpression: false,
        superClass: None,
    };
}

#[deriving(Clone, Show, Encodable)]
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
