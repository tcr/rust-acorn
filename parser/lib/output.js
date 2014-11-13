
;
const version:&'static str = "0.6.1"; 
const options:options_t = options_t {ecmaVersion: 5, strictSemicolons: false, allowTrailingCommas: true, forbidReserved: "", allowReturnOutsideFunction: false, locations: false, ranges: false, program: null, sourceFile: "", directSourceFile: ""};  const input:&'static str = 0i;  const inputLen:int = 0i;  const sourceFile:&'static str = 0i; 
fn parse(inpt:&str, opts:options_t) -> Box<Node> {
    input = toString(inpt);
    inputLen = input.length();
    setOptions(opts);
    initTokenState();
    return parseTopLevel(options.program);
}
;
const defaultOptions:options_t = options_t {ecmaVersion: 5, strictSemicolons: false, allowTrailingCommas: true, forbidReserved: "", allowReturnOutsideFunction: false, locations: false, ranges: true, program: null, sourceFile: "", directSourceFile: ""}; 


;

;

;
const tokPos:int = 0i; 
const tokStart:int = 0i;  const tokEnd:int = 0i; 
const tokStartLoc:int = 0i;  const tokEndLoc:int = 0i; 
const tokType:keyword_t = {};  const tokVal:js_any_type = js_any_type { value_boolean: false }; 
const tokRegexpAllowed:bool = 0i; 
const tokCurLine:int = 0i;  const tokLineStart:int = 0i; 
const lastStart:int = 0i;  const lastEnd:int = 0i;  const lastEndLoc:int = 0i; 
const inFunction:bool = 0i;  const inGenerator:bool = 0i;  const labels:Vec<label_t> = Vec::new();  const _strict:bool = 0i; 
const metParenL:int = 0i; 
const inTemplate:bool = 0i; 

const empty:Vec<Box<Node>> = vec![]; 
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
const _null:keyword_t = keyword_t {_id: 35, atomValue: ATOM_NULL, beforeExpr: false, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "null", postfix: false, prefix: false, _type : ""};  const _true:keyword_t = keyword_t {_id: 36, atomValue: ATOM_TRUE, beforeExpr: false, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "true", postfix: false, prefix: false, _type : ""}; 
const _false:keyword_t = keyword_t {_id: 37, atomValue: ATOM_FALSE, beforeExpr: false, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "false", postfix: false, prefix: false, _type : ""}; 
const _in:keyword_t = keyword_t {_id: 38, atomValue: ATOM_NULL, beforeExpr: true, binop: 7, isAssign: false, isLoop: false, isUpdate: false, keyword: "in", postfix: false, prefix: false, _type : ""}; 
const _typeof:keyword_t = keyword_t {_id: 39, atomValue: ATOM_NULL, beforeExpr: true, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "typeof", postfix: false, prefix: true, _type : ""}; 
const _instanceof:keyword_t = keyword_t {_id: 40, atomValue: ATOM_NULL, beforeExpr: true, binop: 7, isAssign: false, isLoop: false, isUpdate: false, keyword: "instanceof", postfix: false, prefix: false, _type : ""}; 
const _void:keyword_t = keyword_t {_id: 41, atomValue: ATOM_NULL, beforeExpr: true, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "void", postfix: false, prefix: true, _type : ""}; 
const _delete:keyword_t = keyword_t {_id: 42, atomValue: ATOM_NULL, beforeExpr: true, binop: -1, isAssign: false, isLoop: false, isUpdate: false, keyword: "delete", postfix: false, prefix: true, _type : ""}; 
 fn keywordTypes(arg:&str) -> keyword_t { if (arg == "break") { return _break; } if (arg == "case") { return _case; } if (arg == "catch") { return _catch; } if (arg == "continue") { return _continue; } if (arg == "debugger") { return _debugger; } if (arg == "default") { return _default; } if (arg == "do") { return _do; } if (arg == "else") { return _else; } if (arg == "finally") { return _finally; } if (arg == "for") { return _for; } if (arg == "function") { return _function; } if (arg == "if") { return _if; } if (arg == "return") { return _return; } if (arg == "switch") { return _switch; } if (arg == "throw") { return _throw; } if (arg == "try") { return _try; } if (arg == "var") { return _var; } if (arg == "let") { return _let; } if (arg == "const") { return _const; } if (arg == "while") { return _while; } if (arg == "with") { return _with; } if (arg == "null") { return _null; } if (arg == "true") { return _true; } if (arg == "false") { return _false; } if (arg == "new") { return _new; } if (arg == "in") { return _in; } if (arg == "instanceof") { return _instanceof; } if (arg == "this") { return _this; } if (arg == "typeof") { return _typeof; } if (arg == "void") { return _void; } if (arg == "delete") { return _delete; } if (arg == "class") { return _class; } if (arg == "extends") { return _extends; } if (arg == "export") { return _export; } if (arg == "import") { return _import; } if (arg == "yield") { return _yield; } return {}; } 
;
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
fn isIdentifierStart(code:int) -> bool {
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
    return code >= 0xaa && test(nonASCIIidentifierStart, fromCharCode(code));
}
;
fn isIdentifierChar(code:int) -> bool {
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
    return code >= 0xaa && test(nonASCIIidentifier, fromCharCode(code));
}
;

    fn Position() -> () {
        THIS.line = tokCurLine;
        THIS.column = tokPos - tokLineStart;
    }
    

fn initTokenState() -> int {
    tokCurLine = 1;
    tokPos = tokLineStart = 0;
    tokRegexpAllowed = true;
    metParenL = 0;
    inTemplate = false;
    skipSpace();
}
fn finishToken(_type :keyword_t, val:js_any_type) -> int {
    tokEnd = tokPos;
    
    tokType = _type;
    if (_type !=_bquote || inTemplate) {
skipSpace();
}
    tokVal = val;
    tokRegexpAllowed = _type.beforeExpr;
    
}
fn skipBlockComment() -> int {
    let start:int = tokPos;  let end:int = indexOf(input, "*/", tokPos += 2); 
    if (end==-1) {
raise(tokPos - 2, "Unterminated comment");
}
    tokPos = end + 2;
    
    
}
fn skipLineComment() -> int {
    let start:int = tokPos; 
    let ch:int = charCodeAt(input, tokPos += 2); 
    while tokPos < inputLen && ch!=10 && ch!=13 && ch!=8232 && ch!=8233{
{
        tokPos+= 1;
        ch = charCodeAt(input, tokPos);
    }
}
    
}
fn skipSpace() -> int {
    while tokPos < inputLen{
{
        let ch:int = charCodeAt(input, tokPos); 
        if (ch==32) {
{
            tokPos+= 1;
        }
} else {if (ch==13) {
{
            tokPos+= 1;
            let next:int = charCodeAt(input, tokPos); 
            if (next==10) {
{
                tokPos+= 1;
            }
}
            
        }
} else {if (ch==10 || ch==8232 || ch==8233) {
{
            tokPos+= 1;
            
        }
} else {if (ch > 8 && ch < 14) {
{
            tokPos+= 1;
        }
} else {if (ch==47) {
{
            let next:int = charCodeAt(input, tokPos + 1); 
            if (next==42) {
{
                skipBlockComment();
            }
} else {if (next==47) {
{
                skipLineComment();
            }
} else {break;}}
        }
} else {if (ch==160) {
{
            tokPos+= 1;
        }
} else {if (ch >= 5760 && test(nonASCIIwhitespace, fromCharCode(ch))) {
{
            tokPos+= 1;
        }
} else {{
            break;
        }}}}}}}}
    }
}
}
fn readToken_dot() -> int {
    let next:int = charCodeAt(input, tokPos + 1); 
    if (next >= 48 && next <= 57) {
return readNumber(true);
}
    let next2:int = charCodeAt(input, tokPos + 2); 
    if (options.ecmaVersion >= 6 && next==46 && next2==46) {
{
        tokPos += 3;
        return finishToken(_ellipsis);
    }
} else {{
        tokPos+= 1;
        return finishToken(_dot);
    }}
}
fn readToken_slash() -> int {
    let next:int = charCodeAt(input, tokPos + 1); 
    if (tokRegexpAllowed) {
{
        tokPos+= 1;
        return readRegexp();
    }
}
    if (next==61) {
return finishOp(_assign, 2);
}
    return finishOp(_slash, 1);
}
fn readToken_mult_modulo(code:int) -> int {
    let next:int = charCodeAt(input, tokPos + 1); 
    if (next==61) {
return finishOp(_assign, 2);
}
    return finishOp(if code==42 { _star } else { _modulo }, 1);
}
fn readToken_pipe_amp(code:int) -> int {
    let next:int = charCodeAt(input, tokPos + 1); 
    if (next==code) {
return finishOp(if code==124 { _logicalOR } else { _logicalAND }, 2);
}
    if (next==61) {
return finishOp(_assign, 2);
}
    return finishOp(if code==124 { _bitwiseOR } else { _bitwiseAND }, 1);
}
fn readToken_caret() -> int {
    let next:int = charCodeAt(input, tokPos + 1); 
    if (next==61) {
return finishOp(_assign, 2);
}
    return finishOp(_bitwiseXOR, 1);
}
fn readToken_plus_min(code:int) -> int {
    let next:int = charCodeAt(input, tokPos + 1); 
    if (next==code) {
{
        if (next == 45 && charCodeAt(input, tokPos + 2) == 62 && test(newline, slice(input, lastEnd, tokPos))) {
{
            tokPos += 3;
            skipLineComment();
            skipSpace();
            return readToken();
        }
}
        return finishOp(_incDec, 2);
    }
}
    if (next==61) {
return finishOp(_assign, 2);
}
    return finishOp(_plusMin, 1);
}
fn readToken_lt_gt(code:int) -> int {
    let next:int = charCodeAt(input, tokPos + 1); 
    let size:int = 1; 
    if (next==code) {
{
        size = if code==62 && charCodeAt(input, tokPos + 2)==62 { 3 } else { 2 };
        if (charCodeAt(input, tokPos + size)==61) {
return finishOp(_assign, size + 1);
}
        return finishOp(_bitShift, size);
    }
}
    if (next == 33 && code == 60 && charCodeAt(input, tokPos + 2) == 45 && charCodeAt(input, tokPos + 3) == 45) {
{
        tokPos += 4;
        skipLineComment();
        skipSpace();
        return readToken();
    }
}
    if (next==61) {
size = if charCodeAt(input, tokPos + 2)==61 { 3 } else { 2 };
}
    return finishOp(_relational, size);
}
fn readToken_eq_excl(code:int) -> int {
    let next:int = charCodeAt(input, tokPos + 1); 
    if (next==61) {
return finishOp(_equality, if charCodeAt(input, tokPos + 2)==61 { 3 } else { 2 });
}
    if (code==61 && next==62 && options.ecmaVersion >= 6) {
{
        tokPos += 2;
        return finishToken(_arrow);
    }
}
    return finishOp(if code==61 { _eq } else { _prefix }, 1);
}
fn getTokenFromCode(code:int) -> bool {
    if (inTemplate) {
{
        if (tokType==_string) {
{
            if (code==96) {
{
                tokPos+= 1;
                finishToken(_bquote);
                return true;
            }
}
            if (code==36 && charCodeAt(input, tokPos + 1)==123) {
{
                tokPos += 2;
                finishToken(_dollarBraceL);
                return true;
            }
}
        }
}
        readString();
        return true;
    }
}
    match code{
46 => {readToken_dot();;
return true;},
40 => {tokPos+= 1;;
finishToken(_parenL);;
return true;},
41 => {tokPos+= 1;;
finishToken(_parenR);;
return true;},
59 => {tokPos+= 1;;
finishToken(_semi);;
return true;},
44 => {tokPos+= 1;;
finishToken(_comma);;
return true;},
91 => {tokPos+= 1;;
finishToken(_bracketL);;
return true;},
93 => {tokPos+= 1;;
finishToken(_bracketR);;
return true;},
123 => {tokPos+= 1;;
finishToken(_braceL);;
return true;},
125 => {tokPos+= 1;;
finishToken(_braceR);;
return true;},
58 => {tokPos+= 1;;
finishToken(_colon);;
return true;},
63 => {tokPos+= 1;;
finishToken(_question);;
return true;},
96 => {if (options.ecmaVersion >= 6) {
{
                tokPos+= 1;
                finishToken(_bquote);
                return true;
            }
}},
48 => {let next:int = charCodeAt(input, tokPos + 1); ;
if (next==120 || next==88) {
{
                readRadixNumber(16);
                return true;
            }
};
if (options.ecmaVersion >= 6) {
{
                if (next==111 || next==79) {
{
                    readRadixNumber(8);
                    return true;
                }
}
                if (next==98 || next==66) {
{
                    readRadixNumber(2);
                    return true;
                }
}
            }
}},
49 => {},
50 => {},
51 => {},
52 => {},
53 => {},
54 => {},
55 => {},
56 => {},
57 => {readNumber(false);;
return true;},
34 => {},
39 => {readString(code);;
return true;},
47 => {readToken_slash();;
return true;},
37 => {},
42 => {readToken_mult_modulo(code);;
return true;},
124 => {},
38 => {readToken_pipe_amp(code);;
return true;},
94 => {readToken_caret();;
return true;},
43 => {},
45 => {readToken_plus_min(code);;
return true;},
60 => {},
62 => {readToken_lt_gt(code);;
return true;},
61 => {},
33 => {readToken_eq_excl(code);;
return true;},
126 => {finishOp(_prefix, 1);;
return true;}}
    return false;
}
fn readToken(forceRegexp:bool) -> int {
    if (!forceRegexp) {
tokStart = tokPos;
} else {tokPos = tokStart + 1;}
    
    if (forceRegexp) {
return readRegexp();
}
    if (tokPos >= inputLen) {
return finishToken(_eof);
}
    let code:int = charCodeAt(input, tokPos); 
    if (!inTemplate && (isIdentifierStart(code) || code==92)) {
return readWord();
}
    let tok:bool = getTokenFromCode(code); 
    if (tok==false) {
{
        let ch:&str = fromCharCode(code); 
        if (ch=="\\" || test(nonASCIIidentifierStart, ch)) {
return readWord();
}
        raise(tokPos, "Unexpected character '" + ch + "'");
    }
}
}
fn finishOp(_type :keyword_t, size:int) -> int {
    let str:&str = slice(input, tokPos, tokPos + size); 
    tokPos += size;
    finishToken(_type, str);
}
fn readRegexp() -> int {
    let content:&str = "";  let escaped:bool = 0i;  let inClass:bool = 0i;  let start:int = tokPos; 
    ;
loop{{
        if (tokPos >= inputLen) {
raise(start, "Unterminated regular expression");
}
        let ch:&str = charAt(input, tokPos); 
        if (test(newline, ch)) {
raise(start, "Unterminated regular expression");
}
        if (!escaped) {
{
            if (ch=="[") {
inClass = true;
} else {if (ch=="]" && inClass) {
inClass = false;
} else {if (ch=="/" && !inClass) {
break;
}}}
            escaped = ch=="\\";
        }
} else {escaped = false;}
        tokPos+= 1;
    };
}
    content = slice(input, start, tokPos);
    tokPos+= 1;
    let mods:&str = readWord1(); 
    if (mods && !test(regex_11, mods)) {
raise(start, "Invalid regular expression flag");
}
    let value = RegExp(content, mods); 
     if (value.length() == 0) { raise(start, "Error parsing regular expression."); } 
    return finishToken(_regexp, value);
}
fn readInt(radix:int, len:int) -> f64 {
    let start:int = tokPos;  let total:int = 0; 
    let i:int = 0; ;
while !len || i < len{{
        let code:int = charCodeAt(input, tokPos);  let val:int = 0i; 
        if (code >= 97) {
val = code - 97 + 10;
} else {if (code >= 65) {
val = code - 65 + 10;
} else {if (code >= 48 && code <= 57) {
val = code - 48;
} else {val = Infinity;}}}
        if (val >= radix) {
break;
}
        tokPos+= 1;
        total = total * radix + val;
    };
}
    if (tokPos==start || (len && tokPos - start!=len)) {
return NaN;
}
    return total;
}
fn readRadixNumber(radix:int) -> int {
    tokPos += 2;
    let val:int = readInt(radix); 
    if (isNaN(val)) {
raise(tokStart + 2, "Expected number in radix " + radix);
}
    if (isIdentifierStart(charCodeAt(input, tokPos))) {
raise(tokPos, "Identifier directly after number");
}
    return finishToken(_num, val);
}
fn readNumber(startsWithDot:bool) -> int {
    let start:int = tokPos;  let isFloat:bool = false;  let octal:bool = charCodeAt(input, tokPos)==48; 
    if (!startsWithDot && isNaN(readInt(10))) {
raise(start, "Invalid number");
}
    if (charCodeAt(input, tokPos)==46) {
{
        tokPos+= 1;
        readInt(10);
        isFloat = true;
    }
}
    let next:int = charCodeAt(input, tokPos); 
    if (next==69 || next==101) {
{
        next = charCodeAt(input, tokPos+= 1);
        if (next==43 || next==45) {
tokPos+= 1;
}
        if (isNaN(readInt(10))) {
raise(start, "Invalid number");
}
        isFloat = true;
    }
}
    if (isIdentifierStart(charCodeAt(input, tokPos))) {
raise(tokPos, "Identifier directly after number");
}
    let str:&str = slice(input, start, tokPos);  let val:int = 0i; 
    if (isFloat) {
val = parseFloat(str);
} else {if (!octal || str.length()==1) {
val = parseInt(str, 10);
} else {if (test(regex_12, str) || _strict) {
raise(start, "Invalid number");
} else {val = parseInt(str, 8);}}}
    return finishToken(_num, val);
}
fn readCodePoint() -> &'static str {
    let ch:int = charCodeAt(input, tokPos);  let code:int = 0i; 
    if (ch==123) {
{
        if (options.ecmaVersion < 6) {
unexpected();
}
        tokPos+= 1;
        code = readHexChar(indexOf(input, "}", tokPos) - tokPos);
        tokPos+= 1;
        if (code > 0x10FFFF) {
unexpected();
}
    }
} else {{
        code = readHexChar(4);
    }}
    if (code <= 0xFFFF) {
{
        return fromCharCode(code);
    }
}
    let cu1:int = ((code - 0x10000) >> 10) + 0xD800; 
    let cu2:int = ((code - 0x10000) & 1023) + 0xDC00; 
    return fromCharCode(cu1, cu2);
}
fn readString(quote:int) -> int {
    if (!inTemplate) {
tokPos+= 1;
}
    let out:&str = ""; 
    ;
loop{{
        if (tokPos >= inputLen) {
raise(tokStart, "Unterminated string constant");
}
        let ch:int = charCodeAt(input, tokPos); 
        if (inTemplate) {
{
            if (ch==96 || ch==36 && charCodeAt(input, tokPos + 1)==123) {
return finishToken(_string, out);
}
        }
} else {if (ch==quote) {
{
            tokPos+= 1;
            return finishToken(_string, out);
        }
}}
        if (ch==92) {
{
            ch = charCodeAt(input, tokPos+= 1);
            let octalmatch = exec(regex_13, slice(input, tokPos, tokPos + 3)); 
            let octal:&str = if octalmatch { octalmatch[0] } else { "0" }; 
            while octal && parseInt(octal, 8) > 255{
octal = slice(octal, 0, -1);
}
            if (octal=="0") {
octal = null;
}
            tokPos+= 1;
            if (octal.length() > 0) {
{
                if (_strict) {
raise(tokPos - 2, "Octal literal in _strict mode");
}
                out += fromCharCode(parseInt(octal, 8));
                tokPos += octal.length() - 1;
            }
} else {{
                match ch{
110 => {out += "\n";;
break;},
114 => {out += "\r";;
break;},
120 => {out += fromCharCode(readHexChar(2));;
break;},
117 => {out += readCodePoint();;
break;},
85 => {out += fromCharCode(readHexChar(8));;
break;},
116 => {out += "\t";;
break;},
98 => {out += "\u0008";;
break;},
118 => {out += "\u000b";;
break;},
102 => {out += "\u000c";;
break;},
48 => {out += "\u0000";;
break;},
13 => {if (charCodeAt(input, tokPos)==10) {
tokPos+= 1;
}},
10 => {;
break;},
_ => {out += fromCharCode(ch);;
break;}}
            }}
        }
} else {{
            tokPos+= 1;
            if (test(newline, fromCharCode(ch))) {
{
                if (inTemplate) {
{
                    if (ch==13 && charCodeAt(input, tokPos)==10) {
{
                        tokPos+= 1;
                        ch = 10;
                    }
}
                    
                }
} else {{
                    raise(tokStart, "Unterminated string constant");
                }}
            }
}
            out += fromCharCode(ch);
        }}
    };
}
}
fn readHexChar(len:int) -> int {
    let n:int = readInt(16, len); 
    if (isNaN(n)) {
raise(tokStart, "Bad character escape sequence");
}
    return n;
}
const containsEsc:bool = 0i; 
fn readWord1() -> &'static str {
    containsEsc = false;
    let word:&str = 0i;  let first:bool = true;  let start:int = tokPos; 
    ;
loop{{
        let ch:int = charCodeAt(input, tokPos); 
        if (isIdentifierChar(ch)) {
{
            if (containsEsc) {
word += charAt(input, tokPos);
}
            tokPos+= 1;
        }
} else {if (ch==92) {
{
            if (!containsEsc) {
word = slice(input, start, tokPos);
}
            containsEsc = true;
            if (charCodeAt(input, tokPos+= 1) != 117) {
raise(tokPos, "Expecting Unicode escape sequence \\uXXXX");
}
            tokPos+= 1;
            let esc:int = readHexChar(4); 
            let escStr:&str = fromCharCode(esc); 
            if (!escStr) {
raise(tokPos - 1, "Invalid Unicode escape");
}
            if (!(if first { isIdentifierStart(esc) } else { isIdentifierChar(esc) })) {
raise(tokPos - 4, "Invalid Unicode escape");
}
            word += escStr;
        }
} else {{
            break;
        }}}
        first = false;
    };
}
    return if containsEsc { word } else { slice(input, start, tokPos) };
}
fn readWord() -> int {
    let word:&str = readWord1(); 
    let _type :keyword_t = _name; 
    if (!containsEsc && isKeyword(word)) {
_type = keywordTypes(word);
}
    return finishToken(_type, word);
}
fn next() -> int {
    lastStart = tokStart;
    lastEnd = tokEnd;
    lastEndLoc = tokEndLoc;
    readToken();
}
fn setStrict(strct:bool) -> int {
    _strict = strct;
    tokPos = tokStart;
    
    skipSpace();
    readToken();
}

    
    

;

    
    

;
fn startNode() -> Box<Node> {
    let node:Box<Node> = box Node { start: 0 }; 
    
    if (options.directSourceFile.length() > 0) {
node.sourceFile = options.directSourceFile;
}
    if (options.ranges) {
node.range = vec![tokStart, 0];
}
    return node;
}
fn startNodeFrom(other:Box<Node>) -> Box<Node> {
    let node:Box<Node> = box Node { start: 0 }; 
    node.start = other.start;
    
    if (options.ranges) {
node.range = vec![other.range[0], 0];
}
    return node;
}
fn enterNode(node:Box<Node>, _type :&str) -> Box<Node> {
    node._type = _type;
    return node;
}
fn finishNode(node:Box<Node>) -> Box<Node> {
    node.end = lastEnd;
    
    if (options.ranges) {
node.range[1] = lastEnd;
}
     jsparse_callback_close(convert_to_Node_C(node)); 
    return node;
}

fn eat(_type :keyword_t) -> bool {
    if (tokType== _type) {
{
        next();
        return true;
    }
} else {{
        return false;
    }}
}
fn canInsertSemicolon() -> bool {
    return !options.strictSemicolons && (tokType==_eof || tokType==_braceR || test(newline, slice(input, lastEnd, tokStart)));
}
fn semicolon() -> int {
    if (!eat(_semi) && !canInsertSemicolon()) {
unexpected();
}
}
fn expect(_type :keyword_t) -> int {
    eat(_type) || unexpected();
}
fn unexpected(pos:int) -> int {
    raise(if ISNOTNULL(pos) { pos } else { tokStart }, "Unexpected token");
}


fn checkSpreadAssign(node:Box<Node>) -> int {
    if (node._type!="Identifier" && node._type!="ArrayPattern") {
unexpected(node.start);
}
}



fn parseTopLevel(program:Box<Node>) -> Box<Node> {
    lastStart = lastEnd = tokPos;
    
    inFunction = inGenerator = _strict = false;
    labels = vec![];
    readToken();
    let node:Box<Node> = program || startNode();  let first:bool = true; 
    if (!program) {
node.bodylist = vec![];
}
    while tokType!=_eof{
{
        let stmt:Box<Node> = parseStatement(); 
        push(node.bodylist, stmt);
        if (first && isUseStrict(stmt)) {
setStrict(true);
}
        first = false;
    }
}
    enterNode(node, "Program");
    return finishNode(node);
}
const loopLabel:label_t = label_t {kind: "loop"};  const switchLabel:label_t = label_t {kind: "switch"}; 
fn parseStatement() -> Box<Node> {
     jsparse_callback_open("parseStatement"); 
    if (tokType==_slash || tokType==_assign && tokVal == "/=") {
readToken(true);
}
    let starttype:keyword_t = tokType;  let node:Box<Node> = startNode(); 
    match starttype{
_break => {},
_continue => {return parseBreakContinueStatement(node, starttype.keyword);},
_debugger => {return parseDebuggerStatement(node);},
_do => {return parseDoStatement(node);},
_for => {return parseForStatement(node);},
_function => {return parseFunctionStatement(node);},
_class => {return parseClass(node, true);},
_if => {return parseIfStatement(node);},
_return => {return parseReturnStatement(node);},
_switch => {return parseSwitchStatement(node);},
_throw => {return parseThrowStatement(node);},
_try => {return parseTryStatement(node);},
_var => {},
_let => {},
_const => {return parseVarStatement(node, starttype.keyword);},
_while => {return parseWhileStatement(node);},
_with => {return parseWithStatement(node);},
_braceL => {return parseBlock();},
_semi => {return parseEmptyStatement(node);},
_export => {return parseExport(node);},
_import => {return parseImport(node);},
_ => {let maybeName = tokVal;  let expr:Box<Node> = parseExpression(); ;
if (starttype==_name && expr._type=="Identifier" && eat(_colon)) {
return parseLabeledStatement(node, maybeName, expr);
} else {return parseExpressionStatement(node, expr);}}}
}
fn parseBreakContinueStatement(node:Box<Node>, keyword:&str) -> Box<Node> {
    let isBreak:bool = keyword == "break"; 
    next();
    if (eat(_semi) || canInsertSemicolon()) {
node.label = null;
} else {if (tokType!=_name) {
unexpected();
} else {{
        node.label = parseIdent();
        semicolon();
    }}}
    let i:int = 0; ;
while i < labels.size(){{
        let lab:label_t = labels[i]; 
        if (ISNULL(node.label) || lab.name==node.label.name) {
{
            if (ISNOTNULL(lab.kind) && (isBreak || lab.kind=="loop")) {
break;
}
            if (node.label && isBreak) {
break;
}
        }
}
    };
}
    if (i==labels.size()) {
raise(node.start, "Unsyntactic " + keyword);
}
    enterNode(node, if isBreak { "BreakStatement" } else { "ContinueStatement" });
    return finishNode(node);
}
fn parseDebuggerStatement(node:Box<Node>) -> Box<Node> {
    next();
    semicolon();
    enterNode(node, "DebuggerStatement");
    return finishNode(node);
}
fn parseDoStatement(node:Box<Node>) -> Box<Node> {
    next();
    push(labels, loopLabel);
    node.body = parseStatement();
    pop(labels);
    expect(_while);
    node.test = parseParenExpression();
    semicolon();
    enterNode(node, "DoWhileStatement");
    return finishNode(node);
}
fn parseForStatement(node:Box<Node>) -> Box<Node> {
    next();
    push(labels, loopLabel);
    expect(_parenL);
    if (tokType==_semi) {
return parseFor(node, null);
}
    if (tokType==_var || tokType==_let) {
{
        let init:Box<Node> = startNode();  let varKind:&str = tokType.keyword;  let isLet:bool = tokType==_let; 
        next();
        parseVar(init, true, varKind);
        enterNode(init, "VariableDeclaration");
        finishNode(init);
        if ((tokType==_in || (tokType==_name && tokVal=="of")) && init.declarations.size()==1 && !(isLet && init.declarations[0].init)) {
return parseForIn(node, init);
}
        return parseFor(node, init);
    }
}
    let init:Box<Node> = parseExpression(false, true); 
    if (tokType==_in || (tokType==_name && tokVal=="of")) {
{
        {
        }
        return parseForIn(node, init);
    }
}
    return parseFor(node, init);
}
fn parseFunctionStatement(node:Box<Node>) -> Box<Node> {
     jsparse_callback_open("function-declaration"); 
    next();
    return parseFunction(node, true);
}
fn parseIfStatement(node:Box<Node>) -> Box<Node> {
     jsparse_callback_open("if-start"); 
    enterNode(node, "IfStatement");
    next();
     jsparse_callback_open("if-test"); 
    node.test = parseParenExpression();
     jsparse_callback_open("if-consequent"); 
    node.consequent = parseStatement();
    if (eat(_else)) {
{
         jsparse_callback_open("if-alternate"); 
        node.alternate = parseStatement();
    }
} else {{
         jsparse_callback_open("if-no-alternate"); 
        node.alternate = null;
    }}
     jsparse_callback_open("if-end"); 
    return finishNode(node);
}
fn parseReturnStatement(node:Box<Node>) -> Box<Node> {
     jsparse_callback_open("parseReturnStatement"); 
    if (!inFunction && !options.allowReturnOutsideFunction) {
raise(tokStart, "'return' outside of function");
}
    next();
    if (eat(_semi) || canInsertSemicolon()) {
node.argument = null;
} else {{
        node.argument = parseExpression();
        semicolon();
    }}
    if (ISNULL(node.argument)) {
{
         jsparse_callback_open("return-no-argument"); 
    }
} else {{
         jsparse_callback_open("return-argument"); 
    }}
    enterNode(node, "ReturnStatement");
    return finishNode(node);
}
fn parseSwitchStatement(node:Box<Node>) -> Box<Node> {
    next();
    node.discriminant = parseParenExpression();
    node.cases = vec![];
    expect(_braceL);
    push(labels, switchLabel);
    let cur:Box<Node> = nullptr;  let sawDefault:bool = 0i; ;
while tokType != _braceR{{
        if (tokType==_case || tokType==_default) {
{
            let isCase:bool = tokType==_case; 
            if (cur) {
{
                enterNode(cur, "SwitchCase");
                finishNode(cur);
            }
}
            push(node.cases, cur = startNode());
            cur.consequents = vec![];
            next();
            if (isCase) {
cur.test = parseExpression();
} else {{
                if (sawDefault) {
raise(lastStart, "Multiple default clauses");
}
                sawDefault = true;
                cur.test = null;
            }}
            expect(_colon);
        }
} else {{
            if (!cur) {
unexpected();
}
            push(cur.consequents, parseStatement());
        }}
    };
}
    if (cur) {
{
        enterNode(cur, "SwitchCase");
        finishNode(cur);
    }
}
    next();
    pop(labels);
    enterNode(node, "SwitchStatement");
    return finishNode(node);
}
fn parseThrowStatement(node:Box<Node>) -> Box<Node> {
    next();
    if (test(newline, slice(input, lastEnd, tokStart))) {
raise(lastEnd, "Illegal newline after throw");
}
    node.argument = parseExpression();
    semicolon();
    enterNode(node, "ThrowStatement");
    return finishNode(node);
}
fn parseTryStatement(node:Box<Node>) -> Box<Node> {
    next();
    node.block = parseBlock();
    node.handler = null;
    if (tokType==_catch) {
{
        let clause:Box<Node> = startNode(); 
        next();
        expect(_parenL);
        clause.param = parseIdent();
        if (_strict && isStrictBadIdWord(clause.param.name)) {
raise(clause.param.start, "Binding " + clause.param.name + " in _strict mode");
}
        expect(_parenR);
        clause.guard = null;
        clause.body = parseBlock();
        enterNode(clause, "CatchClause");
        node.handler = finishNode(clause);
    }
}
    node.guardedHandlers = empty;
    node.finalizer = if eat(_finally) { parseBlock() } else { null };
    if (!node.handler && !node.finalizer) {
raise(node.start, "Missing catch or finally clause");
}
    enterNode(node, "TryStatement");
    return finishNode(node);
}
fn parseVarStatement(node:Box<Node>, kind:&str) -> Box<Node> {
    next();
    parseVar(node, false, kind);
    semicolon();
    enterNode(node, "VariableDeclaration");
    return finishNode(node);
}
fn parseWhileStatement(node:Box<Node>) -> Box<Node> {
    next();
     jsparse_callback_open("while-test"); 
    node.test = parseParenExpression();
    push(labels, loopLabel);
     jsparse_callback_open("while-body"); 
    node.body = parseStatement();
    pop(labels);
    enterNode(node, "WhileStatement");
     jsparse_callback_open("while-end"); 
    return finishNode(node);
}
fn parseWithStatement(node:Box<Node>) -> Box<Node> {
    if (_strict) {
raise(tokStart, "'with' in _strict mode");
}
    next();
    node.object = parseParenExpression();
    node.body = parseStatement();
    enterNode(node, "WithStatement");
    return finishNode(node);
}
fn parseEmptyStatement(node:Box<Node>) -> Box<Node> {
    next();
    enterNode(node, "EmptyStatement");
    return finishNode(node);
}
fn parseLabeledStatement(node:Box<Node>, maybeName:&str, expr:Box<Node>) -> Box<Node> {
    let i:int = 0; ;
while i < labels.size(){if (labels[i].name==maybeName) {
raise(expr.start, "Label '" + maybeName + "' is already declared");
};
}
    let kind:&str = if tokType.isLoop { "loop" } else { if tokType==_switch { "switch" } else { null } }; 
    push(labels, label_t {kind: kind, name: maybeName});
    node.body = parseStatement();
    pop(labels);
    node.label = expr;
    enterNode(node, "LabeledStatement");
    return finishNode(node);
}
fn parseExpressionStatement(node:Box<Node>, expr:Box<Node>) -> Box<Node> {
    node.expression = expr;
    semicolon();
    enterNode(node, "ExpressionStatement");
    return finishNode(node);
}
fn parseParenExpression() -> Box<Node> {
    expect(_parenL);
    let val:Box<Node> = parseExpression(); 
    expect(_parenR);
    return val;
}
fn parseBlock(allowStrict:bool) -> Box<Node> {
    let node:Box<Node> = startNode();  let first:bool = true;  let isstrict:bool = false;  let oldStrict:bool = 0i; 
    node.bodylist = vec![];
    expect(_braceL);
    while !eat(_braceR){
{
        let stmt:Box<Node> = parseStatement(); 
        push(node.bodylist, stmt);
        if (first && allowStrict && isUseStrict(stmt)) {
{
            oldStrict = isstrict;
            setStrict(isstrict = true);
        }
}
        first = false;
    }
}
    if (isstrict && !oldStrict) {
setStrict(false);
}
    enterNode(node, "BlockStatement");
    return finishNode(node);
}
fn parseFor(node:Box<Node>, init:Box<Node>) -> Box<Node> {
    node.init = init;
    expect(_semi);
     jsparse_callback_open("for-test"); 
    node.test = if tokType==_semi { null } else { parseExpression() };
    expect(_semi);
     jsparse_callback_open("for-update"); 
    node.update = if tokType==_parenR { null } else { parseExpression() };
    expect(_parenR);
     jsparse_callback_open("for-body"); 
    node.body = parseStatement();
    pop(labels);
     jsparse_callback_open("for-end"); 
    enterNode(node, "ForStatement");
    return finishNode(node);
}
fn parseForIn(node:Box<Node>, init:Box<Node>) -> Box<Node> {
    let _type :&str = if tokType==_in { "ForInStatement" } else { "ForOfStatement" }; 
    next();
    node.left = init;
    node.right = parseExpression();
    expect(_parenR);
    node.body = parseStatement();
    pop(labels);
    enterNode(node, _type);
    return finishNode(node);
}
fn parseVar(node:Box<Node>, noIn:bool, kind:&str) -> Box<Node> {
    node.declarations = vec![];
    node.kind = kind;
    ;
loop{{
         jsparse_callback_open("var-declarator"); 
        let decl:Box<Node> = startNode(); 
        decl.id = if options.ecmaVersion >= 6 { toAssignable(parseExprAtom()) } else { parseIdent() };
        {
        }
        if (eat(_eq)) {
{
             jsparse_callback_open("var-declarator-assign"); 
            decl.init = parseExpression(true, noIn);
        }
} else {if (kind==_const.keyword) {
{
            unexpected();
        }
} else {{
             jsparse_callback_open("var-declarator-no-assign"); 
            decl.init = null;
        }}}
        enterNode(decl, "VariableDeclarator");
        push(node.declarations, finishNode(decl));
        if (!eat(_comma)) {
break;
}
    };
}
    return node;
}
fn parseExpression(noComma:bool, noIn:bool) -> Box<Node> {
     jsparse_callback_open("parseExpression"); 
    let expr:Box<Node> = parseMaybeAssign(noIn); 
    if (!noComma && tokType==_comma) {
{
        let node:Box<Node> = startNodeFrom(expr); 
        node.expressions = vec![expr];
        while eat(_comma){
push(node.expressions, parseMaybeAssign(noIn));
}
        enterNode(node, "SequenceExpression");
        return finishNode(node);
    }
}
    return expr;
}
fn parseMaybeAssign(noIn:bool) -> Box<Node> {
    let left:Box<Node> = parseMaybeConditional(noIn); 
    if (tokType.isAssign) {
{
         jsparse_callback_open(tokVal.value_string.c_str()); 
        let node:Box<Node> = startNodeFrom(left); 
        node._operator = tokVal;
        node.left = if tokType==_eq { toAssignable(left) } else { left };
        {
        }
        next();
        node.right = parseMaybeAssign(noIn);
        enterNode(node, "AssignmentExpression");
        return finishNode(node);
    }
}
    return left;
}
fn parseMaybeConditional(noIn:bool) -> Box<Node> {
    let expr:Box<Node> = parseExprOps(noIn); 
    if (eat(_question)) {
{
        let node:Box<Node> = startNodeFrom(expr); 
        node.test = expr;
         jsparse_callback_open("ternary-consequent"); 
        node.consequent = parseExpression(true);
        expect(_colon);
         jsparse_callback_open("ternary-alternate"); 
        node.alternate = parseExpression(true, noIn);
        enterNode(node, "ConditionalExpression");
        return finishNode(node);
    }
}
    return expr;
}
fn parseExprOps(noIn:bool) -> Box<Node> {
    return parseExprOp(parseMaybeUnary(), -1, noIn);
}
fn parseExprOp(left:Box<Node>, minPrec:int, noIn:bool) -> Box<Node> {
    let prec:int = tokType.binop; 
    if (ISNOTNULL(prec) && (!noIn || tokType!=_in)) {
{
        if (prec > minPrec) {
{
            let node:Box<Node> = startNodeFrom(left); 
            node.left = left;
            node._operator = tokVal;
             jsparse_callback_open(tokVal.value_string.c_str()); 
            let op:keyword_t = tokType; 
            next();
            node.right = parseExprOp(parseMaybeUnary(), prec, noIn);
            enterNode(node, if op==_logicalOR || op==_logicalAND { "LogicalExpression" } else { "BinaryExpression" });
            let exprNode:Box<Node> = finishNode(node); 
            return parseExprOp(exprNode, minPrec, noIn);
        }
}
    }
}
    return left;
}
fn parseMaybeUnary() -> Box<Node> {
     if (tokType.keyword == "function") { jsparse_callback_open(tokVal.value_string.c_str()); } 
    if (tokType.prefix) {
{
         jsparse_callback_open(("unary-" + tokVal.value_string).c_str()); 
        let node:Box<Node> = startNode();  let update:bool = tokType.isUpdate; 
        node._operator = tokVal;
        node.prefix = true;
        tokRegexpAllowed = true;
        next();
        enterNode(node, if update { "UpdateExpression" } else { "UnaryExpression" });
        node.argument = parseMaybeUnary();
        if (update) {
checkLVal(node.argument);
} else {if (_strict && node._operator=="delete" && node.argument._type=="Identifier") {
raise(node.start, "Deleting local variable in _strict mode");
}}
        return finishNode(node);
    }
}
    let expr:Box<Node> = parseExprSubscripts(); 
    while tokType.postfix && !canInsertSemicolon(){
{
         jsparse_callback_open(tokVal.value_string.c_str()); 
        let node:Box<Node> = startNodeFrom(expr); 
        node._operator = tokVal;
        node.prefix = false;
        node.argument = expr;
        {
        }
        next();
        enterNode(node, "UpdateExpression");
        expr = finishNode(node);
    }
}
    return expr;
}
fn parseExprSubscripts() -> Box<Node> {
    return parseSubscripts(parseExprAtom());
}
fn parseSubscripts(base:Box<Node>, noCalls:bool) -> Box<Node> {
     jsparse_callback_open("subscripts"); 
    if (eat(_dot)) {
{
        let node:Box<Node> = startNodeFrom(base); 
        enterNode(node, "MemberExpression");
        node.object = base;
        node.property = parseIdent(true);
        node.computed = false;
        return parseSubscripts(finishNode(node), noCalls);
    }
} else {if (eat(_bracketL)) {
{
         jsparse_callback_open("member-var-open"); 
        let node:Box<Node> = startNodeFrom(base); 
        node.object = base;
        node.property = parseExpression();
        node.computed = true;
        expect(_bracketR);
         jsparse_callback_open("member-var-close"); 
        enterNode(node, "MemberExpression");
        return parseSubscripts(finishNode(node), noCalls);
    }
} else {if (!noCalls && eat(_parenL)) {
{
         jsparse_callback_open("call-open"); 
        let node:Box<Node> = startNodeFrom(base); 
        enterNode(node, "CallExpression");
        node.callee = base;
        node.arguments = parseExprList(_parenR, false);
        return parseSubscripts(finishNode(node), noCalls);
    }
} else {if (tokType==_bquote) {
{
        let node:Box<Node> = startNodeFrom(base); 
        node.tag = base;
        node.quasi = parseTemplate();
        enterNode(node, "TaggedTemplateExpression");
        return parseSubscripts(finishNode(node), noCalls);
    }
}}}}
    return base;
}
fn parseExprAtom() -> Box<Node> {
    match tokType{
_this => {let node:Box<Node> = startNode(); ;
next();;
enterNode(node, "ThisExpression");;
return finishNode(node);},
_yield => {if (inGenerator) {
return parseYield();
}},
_name => {let id:Box<Node> = parseIdent(tokType!=_name); ;
if (eat(_arrow)) {
{
                return parseArrowExpression(startNodeFrom(id), vec![id]);
            }
};
return id;},
_num => {},
_string => {},
_regexp => {let node:Box<Node> = startNode(); ;
node.value = tokVal;;
node.raw = slice(input, tokStart, tokEnd);;
next();;
enterNode(node, "Literal");;
return finishNode(node);},
_null => {},
_true => {},
_false => {let node:Box<Node> = startNode(); ;
node.raw = tokType.keyword;;
next();;
enterNode(node, "Literal");;
return finishNode(node);},
_parenL => {let tokStartLoc1:int = tokStartLoc;  let tokStart1:int = tokStart;  let val:Box<Node> = 0i;  let exprList:Vec<Box<Node>> = Vec::new(); ;
next();;
if (options.ecmaVersion >= 6 && tokType==_for) {
{
                val = parseComprehension(startNode(), true);
            }
} else {{
                let oldParenL:int = metParenL+= 1; 
                if (tokType!=_parenR) {
{
                    val = parseExpression();
                    exprList = if val._type=="SequenceExpression" { val.expressions } else { vec![val] };
                }
} else {{
                    exprList = vec![];
                }}
                expect(_parenR);
                if (metParenL==oldParenL && eat(_arrow)) {
{
                    val = parseArrowExpression(startNode(), exprList);
                }
} else {{
                    if (!val) {
unexpected(lastStart);
}
                    if (options.ecmaVersion >= 6) {
{
                        let i:int = 0; ;
while i < exprList.size(){{
                            if (exprList[i]._type=="SpreadElement") {
unexpected();
}
                        };
}
                    }
}
                }}
            }};
val.start = tokStart1;;
val.end = lastEnd;;
;
if (options.ranges) {
{
                val.range = vec![tokStart1, lastEnd];
            }
};
return val;},
_bracketL => {let node:Box<Node> = startNode(); ;
next();;
if (options.ecmaVersion >= 6 && tokType==_for) {
{
                return parseComprehension(node, false);
            }
};
node.elements = parseExprList(_bracketR, true, true);;
enterNode(node, "ArrayExpression");;
return finishNode(node);},
_braceL => {return parseObj();},
_function => {let node:Box<Node> = startNode(); ;
next();;
return parseFunction(node, false);},
_class => {return parseClass(startNode(), false);},
_new => {return parseNew();},
_ellipsis => {return parseSpread();},
_bquote => {return parseTemplate();},
_ => {unexpected();}}
}
fn parseNew() -> Box<Node> {
     jsparse_callback_open("new-open"); 
    let node:Box<Node> = startNode(); 
    next();
    node.callee = parseSubscripts(parseExprAtom(), true);
     jsparse_callback_open("new-args"); 
    if (eat(_parenL)) {
node.arguments = parseExprList(_parenR, false);
} else {node.arguments = empty;}
     jsparse_callback_open("new-close"); 
    enterNode(node, "NewExpression");
    return finishNode(node);
}
fn parseSpread() -> Box<Node> {
    let node:Box<Node> = startNode(); 
    next();
    node.argument = parseExpression(true);
    enterNode(node, "SpreadElement");
    return finishNode(node);
}

fn parseObj() -> Box<Node> {
     jsparse_callback_open("object-literal"); 
    let node:Box<Node> = startNode();  let first:bool = true;  let propHash:&str = ""; 
    node.properties = vec![];
    next();
    while !eat(_braceR){
{
        if (!first) {
{
            expect(_comma);
            if (options.allowTrailingCommas && eat(_braceR)) {
break;
}
        }
} else {first = false;}
        let prop:Box<Node> = startNode();  let kind:&str = 0i;  let isGenerator:bool = 0i; 
        if (options.ecmaVersion >= 6) {
{
            prop.method = false;
            prop.shorthand = false;
            isGenerator = eat(_star);
        }
}
         jsparse_callback_open("object-literal-key"); 
        parsePropertyName(prop);
        if (eat(_colon)) {
{
             jsparse_callback_open("object-literal-value"); 
            prop.value = parseExpression(true);
            kind = prop.kind = "init";
        }
} else {if (options.ecmaVersion >= 6 && tokType==_parenL) {
{
            kind = prop.kind = "init";
            prop.method = true;
            prop.value = parseMethod(isGenerator);
        }
} else {if (options.ecmaVersion >= 5 && !prop.computed && prop.key._type=="Identifier" && (prop.key.name=="get" || prop.key.name=="set")) {
{
            if (isGenerator) {
unexpected();
}
            kind = prop.kind = prop.key.name;
            parsePropertyName(prop);
            prop.value = parseMethod(false);
        }
} else {if (options.ecmaVersion >= 6 && !prop.computed && prop.key._type=="Identifier") {
{
            kind = prop.kind = "init";
            prop.value = prop.key;
            prop.shorthand = true;
        }
} else {unexpected();}}}}
        {
        }
        enterNode(prop, "Property");
         jsparse_callback_open("object-literal-push"); 
        push(node.properties, finishNode(prop));
    }
}
    enterNode(node, "ObjectExpression");
    return finishNode(node);
}
fn parsePropertyName(prop:Box<Node>) -> int {
    if (options.ecmaVersion >= 6) {
{
        if (eat(_bracketL)) {
{
            prop.computed = true;
            prop.key = parseExpression();
            expect(_bracketR);
            return 0;
        }
} else {{
            prop.computed = false;
        }}
    }
}
    prop.key = if tokType==_num || tokType==_string { parseExprAtom() } else { parseIdent(true) };
}
fn initFunction(node:Box<Node>) -> int {
    node.id = null;
    node.params = vec![];
    if (options.ecmaVersion >= 6) {
{
        node.defaults = vec![];
        node.rest = null;
        node.generator = false;
    }
}
}
fn parseFunction(node:Box<Node>, isStatement:bool, allowExpressionBody:bool) -> Box<Node> {
    initFunction(node);
    if (options.ecmaVersion >= 6) {
{
        node.generator = eat(_star);
    }
}
    if (isStatement || tokType==_name) {
{
        node.id = parseIdent();
    }
}
     jsparse_callback_open("function-params"); 
    parseFunctionParams(node);
     jsparse_callback_open("function-body"); 
    parseFunctionBody(node, allowExpressionBody);
    enterNode(node, if isStatement { "FunctionDeclaration" } else { "FunctionExpression" });
    return finishNode(node);
}
fn parseMethod(isGenerator:bool) -> Box<Node> {
    let node:Box<Node> = startNode(); 
    initFunction(node);
    parseFunctionParams(node);
    let allowExpressionBody:bool = 0i; 
    if (options.ecmaVersion >= 6) {
{
        node.generator = isGenerator;
        allowExpressionBody = true;
    }
} else {{
        allowExpressionBody = false;
    }}
    parseFunctionBody(node, allowExpressionBody);
    enterNode(node, "FunctionExpression");
    return finishNode(node);
}
fn parseArrowExpression(node:Box<Node>, params:Vec<Box<Node>>) -> Box<Node> {
    initFunction(node);
    let defaults:Vec<Box<Node>> = node.defaults;  let hasDefaults:bool = false; 
    let i:int = 0;  let lastI:int = params.size() - 1; ;
while i <= lastI{{
        let param:Box<Node> = params[i]; 
        if (param._type=="AssignmentExpression" && param._operator=="=") {
{
            hasDefaults = true;
            params[i] = param.left;
            push(defaults, param.right);
        }
} else {{
            toAssignable(param, i==lastI, true);
            push(defaults, null);
            if (param._type=="SpreadElement") {
{
                pop(params);
                node.rest = param.argument;
                break;
            }
}
        }}
    };
}
    node.params = params;
    if (!hasDefaults) {
node.defaults = vec![];
}
    parseFunctionBody(node, true);
    enterNode(node, "ArrowFunctionExpression");
    return finishNode(node);
}
fn parseFunctionParams(node:Box<Node>) -> int {
    let defaults:Vec<Box<Node>> = vec![];  let hasDefaults:bool = false; 
    expect(_parenL);
    ;
loop{{
        if (eat(_parenR)) {
{
            break;
        }
} else {if (options.ecmaVersion >= 6 && eat(_ellipsis)) {
{
            node.rest = toAssignable(parseExprAtom(), false, true);
            checkSpreadAssign(node.rest);
            expect(_parenR);
            break;
        }
} else {{
             jsparse_callback_open("function-param"); 
            push(node.params, if options.ecmaVersion >= 6 { toAssignable(parseExprAtom(), false, true) } else { parseIdent() });
            if (options.ecmaVersion >= 6 && tokType==_eq) {
{
                next();
                hasDefaults = true;
                push(defaults, parseExpression(true));
            }
}
            if (!eat(_comma)) {
{
                expect(_parenR);
                break;
            }
}
        }}}
    };
}
    if (hasDefaults) {
node.defaults = defaults;
}
}
fn parseFunctionBody(node:Box<Node>, allowExpression:bool) -> int {
    let isExpression:bool = allowExpression && tokType!=_braceL; 
    if (isExpression) {
{
        node.body = parseExpression(true);
    }
} else {{
        let oldInFunc:bool = inFunction;  let oldInGen:bool = inGenerator;  let oldLabels:Vec<label_t> = labels; 
        inFunction = true;
        inGenerator = node.generator;
        labels = vec![];
        node.body = parseBlock(true);
        inFunction = oldInFunc;
        inGenerator = oldInGen;
        labels = oldLabels;
    }}
    if (_strict || !isExpression && node.body.bodylist.size() && isUseStrict(node.body.bodylist[0])) {
{
        let nameHash:&str = ""; 
        if (node.id) {
{
        }
}
        let i:int = 0; ;
while i < node.params.size(){{
        };
}
        if (node.rest) {
{
        }
}
    }
}
}
fn parseClass(node:Box<Node>, isStatement:bool) -> Box<Node> {
    next();
    if (tokType==_name) {
{
        node.id = parseIdent();
    }
} else {if (isStatement) {
{
        unexpected();
    }
} else {{
        node.id = null;
    }}}
    node.superClass = if eat(_extends) { parseExpression() } else { null };
    let classBody:Box<Node> = startNode();  let methodHash:&str = "";  let staticMethodHash:&str = ""; 
    classBody.bodylist = vec![];
    expect(_braceL);
    while !eat(_braceR){
{
        let method:Box<Node> = startNode(); 
        if (tokType==_name && tokVal=="static") {
{
            next();
            method._static = true;
        }
} else {{
            method._static = false;
        }}
        let isGenerator:bool = eat(_star); 
        parsePropertyName(method);
        if (tokType==_name && !method.computed && method.key._type=="Identifier" && (method.key.name=="get" || method.key.name=="set")) {
{
            if (isGenerator) {
unexpected();
}
            method.kind = method.key.name;
            parsePropertyName(method);
        }
} else {{
            method.kind = "";
        }}
        method.value = parseMethod(isGenerator);
        {
        }
        enterNode(method, "MethodDefinition");
        push(classBody.bodylist, finishNode(method));
        eat(_semi);
    }
}
    enterNode(classBody, "ClassBody");
    node.body = finishNode(classBody);
    enterNode(node, if isStatement { "ClassDeclaration" } else { "ClassExpression" });
    return finishNode(node);
}
fn parseExprList(close:keyword_t, allowTrailingComma:bool, allowEmpty:bool) -> Vec<Box<Node>> {
    let elts:Vec<Box<Node>> = vec![];  let first:bool = true; 
    while !eat(close){
{
        if (!first) {
{
            expect(_comma);
             jsparse_callback_open("parseExprList-next"); 
            if (allowTrailingComma && options.allowTrailingCommas && eat(close)) {
break;
}
        }
} else {first = false;}
        if (allowEmpty && tokType==_comma) {
push(elts, null);
} else {push(elts, parseExpression(true));}
    }
}
    return elts;
}
fn parseIdent(liberal:bool) -> Box<Node> {
    let node:Box<Node> = startNode(); 
    if (liberal && options.forbidReserved == "everywhere") {
liberal = false;
}
    if (tokType==_name) {
{
        if (!liberal && (options.forbidReserved && if options.ecmaVersion==3 { isReservedWord3 } else { isReservedWord5 }(tokVal) || _strict && isStrictReservedWord(tokVal)) && indexOf(slice(input, tokStart, tokEnd), "\\") == -1) {
raise(tokStart, "The keyword '" + tokVal + "' is reserved");
}
        node.name = tokVal;
    }
} else {if (liberal && tokType.keyword) {
{
        node.name = tokType.keyword;
    }
} else {{
        unexpected();
    }}}
    tokRegexpAllowed = false;
    next();
    enterNode(node, "Identifier");
    return finishNode(node);
}
fn parseExport(node:Box<Node>) -> Box<Node> {
    next();
    if (tokType==_var || tokType==_const || tokType==_let || tokType==_function || tokType==_class) {
{
        node.declaration = parseStatement();
        node._default = false;
        node.specifiers = vec![];
        node.source = null;
    }
} else {if (eat(_default)) {
{
        node.declaration = parseExpression(true);
        node._default = true;
        node.specifiers = vec![];
        node.source = null;
        semicolon();
    }
} else {{
        let isBatch:bool = tokType==_star; 
        node.declaration = null;
        node._default = false;
        node.specifiers = parseExportSpecifiers();
        if (tokType==_name && tokVal=="from") {
{
            next();
            if (tokType!=_string) {
{
                unexpected();
            }
}
            node.source = parseExprAtom();
        }
} else {{
            if (isBatch) {
unexpected();
}
            node.source = null;
        }}
    }}}
    enterNode(node, "ExportDeclaration");
    return finishNode(node);
}
fn parseExportSpecifiers() -> Vec<Box<Node>> {
    let nodes:Vec<Box<Node>> = vec![];  let first:bool = true; 
    if (tokType==_star) {
{
        let node:Box<Node> = startNode(); 
        next();
        enterNode(node, "ExportBatchSpecifier");
        push(nodes, finishNode(node));
    }
} else {{
        expect(_braceL);
        while !eat(_braceR){
{
            if (!first) {
{
                expect(_comma);
                if (options.allowTrailingCommas && eat(_braceR)) {
break;
}
            }
} else {first = false;}
            let node:Box<Node> = startNode(); 
            node.id = parseIdent();
            if (tokType==_name && tokVal=="as") {
{
                next();
            }
} else {{
                node.name = null;
            }}
            enterNode(node, "ExportSpecifier");
            push(nodes, finishNode(node));
        }
}
    }}
    return nodes;
}
fn parseImport(node:Box<Node>) -> Box<Node> {
    next();
    if (tokType==_string) {
{
        node.specifiers = vec![];
        node.source = parseExprAtom();
        node.kind = "";
    }
} else {{
        node.specifiers = parseImportSpecifiers();
        if (tokType!=_name || tokVal!="from") {
unexpected();
}
        next();
        if (tokType!=_string) {
{
            unexpected();
        }
}
        node.source = parseExprAtom();
        node.kind = if node.specifiers[0]._default { "default" } else { "named" };
    }}
    enterNode(node, "ImportDeclaration");
    return finishNode(node);
}
fn parseImportSpecifiers() -> Vec<Box<Node>> {
    let nodes:Vec<Box<Node>> = vec![];  let first:bool = true; 
    if (tokType==_star) {
{
        let node:Box<Node> = startNode(); 
        next();
        if (tokType!=_name || tokVal!="as") {
unexpected();
}
        next();
        {
        }
        enterNode(node, "ImportBatchSpecifier");
        push(nodes, finishNode(node));
        return nodes;
    }
}
    if (tokType==_name) {
{
        let node:Box<Node> = startNode(); 
        node.id = parseIdent();
        {
        }
        node.name = null;
        node._default = true;
        enterNode(node, "ImportSpecifier");
        push(nodes, finishNode(node));
        if (!eat(_comma)) {
return nodes;
}
    }
}
    expect(_braceL);
    while !eat(_braceR){
{
        if (!first) {
{
            expect(_comma);
            if (options.allowTrailingCommas && eat(_braceR)) {
break;
}
        }
} else {first = false;}
        let node:Box<Node> = startNode(); 
        node.id = parseIdent(true);
        if (tokType==_name && tokVal=="as") {
{
            next();
        }
} else {{
            node.name = null;
        }}
        {
        }
        node._default = false;
        enterNode(node, "ImportSpecifier");
        push(nodes, finishNode(node));
    }
}
    return nodes;
}
fn parseYield() -> Box<Node> {
    let node:Box<Node> = startNode(); 
    next();
    if (eat(_semi) || canInsertSemicolon()) {
{
        node.delegate = false;
        node.argument = null;
    }
} else {{
        node.delegate = eat(_star);
        node.argument = parseExpression(true);
    }}
    enterNode(node, "YieldExpression");
    return finishNode(node);
}
fn parseComprehension(node:Box<Node>, isGenerator:bool) -> Box<Node> {
    node.blocks = vec![];
    while tokType==_for{
{
        let block:Box<Node> = startNode(); 
        next();
        expect(_parenL);
        block.left = toAssignable(parseExprAtom());
        {
        }
        if (tokType!=_name || tokVal!="of") {
unexpected();
}
        next();
        block.of = true;
        block.right = parseExpression();
        expect(_parenR);
        enterNode(block, "ComprehensionBlock");
        push(node.blocks, finishNode(block));
    }
}
    node.filter = if eat(_if) { parseParenExpression() } else { null };
    node.body = parseExpression();
    expect(if isGenerator { _parenR } else { _bracketR });
    node.generator = isGenerator;
    enterNode(node, "ComprehensionExpression");
    return finishNode(node);
}
