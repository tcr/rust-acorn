var fs = require('fs');
var falafel = require('falafel');
var typedacorn = require('typedast');


var input = fs.readFileSync(process.argv[2], 'utf-8');

// Replace "export" statements.
input = input.replace(/^\s*export /mg, '') + '\nvar _dummy = null; export = _dummy;';

// Remove checkLVal lines.
input = input.replace(/^\s*(checkLVal|checkPropClash|checkFunctionParam).*/mg, '{}');
input = input.replace(/^\s*(onToken|onComment):.*/mg, '');
input = input.replace(/^\s*(var startLoc).*/mg, '');
input = input.replace(/^\s*(for \(var kw in keywordTypes\)).*/mg, '');
input = input.replace(/node\.specifiers = null.*/mg, 'node.specifiers = [];');
input = input.replace(/node\.expression = (true|false).*/mg, '');
input = input.replace(/node\.name.*parseIdent.*/mg, '');
input = input.replace(/node\.value.*tokType.atomValue.*/mg, '');
input = input.replace(/if \(strict \|\| !isExpression && node.body.bodylist.length && isUseStrict\(node.body.bodylist\[0\]\)\)[^\}]+?\n\s*\}/, '');

input = input.replace(/\/\/C (.*)/mg, function (_, code) {
  return '__c__(' + JSON.stringify(code) + ')';
});
input = input.replace(/[^\n]+\/\/JS/gm, '');

fs.writeFileSync(__dirname + '/output.js', input)

// Variable types (by name).
var varTypes = {
  cur: 'Box<Node>',
  token: 'keyword_t',
  tokVal: 'js_any_type',
  labels: 'Vec<label_t>',
  oldLabels: 'Vec<label_t>',

  lastEndLoc: 'int',
  tokTypes: 'TokTypes',
  isReservedWord3: '|arg:&str|:\'static -> bool',
  isReservedWord5: '|arg:&str|:\'static -> bool',
  isStrictReservedWord: '|arg:&str|:\'static -> bool',
  isStrictBadIdWord: '|arg:&str|:\'static -> bool',
  isEcma5AndLessKeyword: '|arg:&str|:\'static -> bool',
  isEcma6Keyword: '|arg:&str|:\'static -> bool',
  isKeyword: '|arg:&str|:\'static -> bool',
  nonASCIIwhitespace: '|arg:&str|:\'static -> bool',
  nonASCIIidentifierStart: '|arg:&str|:\'static -> bool',
  nonASCIIidentifier: '|arg:&str|:\'static -> bool',
  newline: '|arg:&str|:\'static -> bool',
  lineBreak: '|arg:&str|:\'static -> bool',

  // workarounds
  methodHash: '&str',
  staticMethodHash: '&str',
  propHash: '&str',
  nameHash: '&str',
}

// Default variable initialization.
var varDefaults = {
  options: 'options_t {ecmaVersion: 5, strictSemicolons: false, allowTrailingCommas: true, forbidReserved: "", allowReturnOutsideFunction: false, locations: false, ranges: false, program: null, sourceFile: "", directSourceFile: ""}',
  match: 'RegExpVector()',
  tokType: '{}',
  tokVal: 'js_any_type { value_boolean: false }',
  labels: 'Vec::new()',
  cur: 'nullptr',
  exprList: 'Vec::new()',
};

// Function type definitions.
var funcTypes = {
  finishToken: ['int', 'keyword_t', 'js_any_type'],
  readInt: ['f64', 'int', 'int'],
  parse: ['Box<Node>', '&str', 'options_t'],
  Position: ['()'],
};

// Functions that we'll implement ourselves.
var funcIgnore = ['toAssignable', 'isUseStrict', 'has', 'checkPropClash',
  'checkFunctionParam', 'checkLVal', 'Node', 'SourceLocation', 'makePredicate',
  'setOptions', 'getLineInfo', 'tokenize', 'raise', 'getCurrentToken',
  'parseTemplate'];


// Worker.

typedacorn.compile(input, function (err, data) {

  var out = data.javascript;
  var getType = data.getType;

  /**
   * Abstracted functions for transform.
   */

  function replace (regex, str)
  {
    var out2 = out.replace(regex, str);
    if (out == out2) {
      // throw new Error('Redundant replace.');
    }
    out = out2;
  }

  function transform (typefns)
  {
    out = falafel(out, {
      loc: true
    }, function (node) {
      // Run type functions.
      typefns.forEach(function (fn) {
        if (fn.slice(0, -1).some(function (t) {
          return typeof t == 'string' ? t == node.type : t.test(node.type)
        })) {
          fn.slice(-1)[0](node);
        }
      });
    }).toString();
  }


  /**
   * Transpiling customization.
   */

  // Keep track of keyword constants and function prototypes.
  var keywordid = 1, keywordids = {}, prototypes = [];
  var hoistedregex = [];
  var typeslist = [];

  falafel(out, {
    loc: true
  }, function (node) {
    if (node.type == 'VariableDeclarator') {
      var id = node.id.source();
      var type = adjusttype(getType(node.id));
      typeslist.push([id, (varTypes[id] || (type && !isinvalidtype(type) ? type : null))])
    }

    if (node.type == 'FunctionExpression' || node.type == 'FunctionDeclaration') {
      var type = getType(node);
      if (node.id) {
        var id = node.id.source();
        if (type && !funcTypes[id]) {
          var fntype = type.map(adjusttype);
          if (!fntype.some(isinvalidtype)) {
            funcTypes[id] = fntype;
          }
        }
      }
    }
  });

  replace(/var _dummy [\s\S]+/, '');
  replace(/var (Position|Node|SourceLocation) = \(function.*$/gm, '');
  replace(/return (Position|Node|SourceLocation);[\s\n]*\}\)\(\);/g, '\n');

  replace(/__c__(.*)/mg, function (_, code) {
    return '/*C ' + JSON.parse(code.replace(/\(/, '').replace(/\);\s*$/, '')) + ' */';
  });

  fs.writeFileSync(__dirname + '/output.js', out)

  // overrides
  transform([
    ['VariableDeclarator', function (node) {
      // Replaces keywordTypes array with a function.
      if (node.id && node.id.source() == 'keywordTypes') {
        var source = node.init.source();
        var keys = source.match(/\"[^"]+\":\s*_\w+/g).map(function (entry) {
          return entry.match(/\"([^"]+)\":\s*_\w+/)[1];
        });
        var code = '/*C fn keywordTypes(arg:&str) -> keyword_t { ' + keys.map(function (entry) {
          return 'if (arg == ' + JSON.stringify(entry) + ') { return _' + entry + '; }';
        }).join(' ') + ' return {}; } */'

        return node.update('__ignore;' + code + '\n');
      }
    }],

    ['CallExpression', function (node) {
      // Replaces makePredicate with a dedicated function.
      if (node.callee.source() == 'makePredicate') {
        var args = node.source().replace(/^[^\(]*\(|\)$/g, '');
        args = args.replace(/"|^\s*|\s*$/g, '').split(/\s+/);
        var cases = args.map(function (str) {
          return 'arg == ' + JSON.stringify(str);
        });

        var name = 'predicate_' + hoistedregex.length;
        hoistedregex.push('fn ' + name + '(arg:&str) -> bool { return ' + cases.join(' || ') + '; }');
        return node.update(name);
      }
    }],
  ])

  fs.writeFileSync(__dirname + '/output.js', out)

  // AST transform.
  transform([
    ['VariableDeclarator', function (node) {
      var id = node.id.source();
      if (id == '__ignore') {
        return node.update('');
      }
      var item = typeslist.filter(function (t) {
        return t[0] == id;
      })[0];
      if (item) {
        typeslist.splice(typeslist.indexOf(item), 1);
      }
      var type = item ? item[1] : nil;
      var init = node.init ?
        node.init.source() : varDefaults[id] || (
          type == 'std::string' ? '""'
          : '0i'
          );
      if (isTopLevel(node)) {
        if (type == '&str') {
          type = '&\'static str';
        }
        node.update('const ' + id + (type ? ':' + type : '') + ' = ' + init + '; ');
      } else {
        node.update('let ' + id + (type ? ':' + type : '') + ' = ' + init + '; ');
      }
    }],

    ['VariableDeclaration', function (node) {
      node.update(node.declarations.map(function (d) {
        return d.source();
      }).join(' '));
    }],

    ['ConditionalExpression', function (node) {
      node.update('if ' + node.test.source() + ' { ' + node.consequent.source() + ' } else { ' + node.alternate.source() + ' }');
    }],

    ['BinaryExpression', function (node) {
      if (node.left.source() == 'null' || node.right.source() == 'null') {
        if (node.operator == '==' || node.operator == '===') {
          node.update('ISNULL(' + node.left.source() + ')');
        }
        if (node.operator == '!=' || node.operator == '!==') {
          node.update('ISNOTNULL(' + node.left.source() + ')');
        }

      // Replace strict-equality operators.
      } else if (node.operator == '===' || node.operator == '!==') {
        node.update(node.left.source() + node.operator.slice(0, -1) + node.right.source());
      }
    }],

    ['LogicalExpression', function (node) {
      // if (node.operator == '||') {
      //   node.update('LOGICALOR(' + node.left.source() + ','+ node.right.source() + ')');
      // }
    }],

    ['CallExpression', 'NewExpression', function (node) {
      if (node.callee.source() == 'String') {
        node.callee.source = function () {
          return 'toString';
        };
      }

      if (node.callee.type == 'MemberExpression') {
        var args = [node.callee.object.source()].concat(node.arguments.map(function (arg) {
          return arg.source();
        }));
        if (args[0] == 'String') {
          args.shift();
        }
        node.update([node.callee.property.source(), '(', args.join(', '), ')'].join(''));
      } else {
        node.update([node.callee.source(), '(', node.arguments.map(function (arg) {
          return arg.source();
        }).join(', '), ')'].join(''));
      }
    }],

    ['SwitchStatement', function (node) {
      // make sure if statement bodies are enclosed
      node.update('match ' + node.discriminant.source() + '{\n' +
        node.cases.map(function (c) {
          return c.source();
        }).join(',\n')
      + '\}');
    }],

    ['SwitchCase', function (node) {
      // make sure if statement bodies are enclosed
      node.update((node.test ? node.test.source() : '_') + ' => {' + node.consequent.map(function (c) {
        return c.source();
      }).join(';\n') + '}');
    }],

    ['IfStatement', function (node) {
      if (node.test.source() == 'options.locations' || node.test.source() == 'options.onComment' || node.test.source() == 'options.onToken') {
        node.update('');
      }

      // make sure if statement bodies are enclosed
      else {
        node.update('if (' + node.test.source() + ') {\n' + node.consequent.source() + '\n}' + (node.alternate ? ' else {' + node.alternate.source() + '}' : ''));
      }
    }],

    ['ThisExpression', function (node) {
      node.update('THIS');
    }],

    ['ReturnStatement', function (node) {
      // JavaScript void is cast to useless integers.
      if (!node.argument) {
        node.update('return 0;');
      }
    }],

    ['UpdateExpression', function (node) {
      node.update(node.argument.source() + node.operator.substr(1) + '= 1');
    }],

    ['FunctionExpression', 'FunctionDeclaration', function (node) {
      // Ignore functions.
      if (node.id && funcIgnore.indexOf(node.id.source()) > -1) {
        node.update('');
        return;
      }

      node.update(node.source().replace(/^function\s*(\w+\s*)?\(([^)]*)\)/, function (_, w, a) {
        var def = funcTypes[w] || ['auto', 'auto', 'auto', 'auto', 'auto', 'auto', 'auto', 'auto', 'auto']; //etc
        var args = a.split(/\s*,\s*/).filter(function (a) {
          return a;
        });

        function makeproto (def, args) {
          return 'fn ' + (w||'') + '(' + args.map(function (arg, i) {
            return arg + ':' + def[i+1];
          }).join(', ') + ') -> ' + (def[0].replace(/^&/, '&\'static '));
        }

        // Optional boolean, integer arguments.
        var proto = makeproto(def, args);
        if (def[0] != 'auto') {
          prototypes.push(proto + ';');
          (function (args) {
            while (args.length && (def[args.length] == 'int' || def[args.length] == 'bool')) {
              prototypes.push(makeproto(def, args.slice(0, -1)) + '{ return ' + w + '(' + args.slice(0, -1).concat([
                def[args.length] == 'bool' ? 'false' : '0'
              ]).join(', ') + '); }');
              args.pop();
            }
          })(args.slice());
        }

        return proto;
      }));
    }],

    ['ForStatement', function (node) {
      node.update((node.init ? node.init.source() : '') + ';\n'
        + (node.test ? 'while ' + node.test.source() : 'loop')
        + '{' + node.body.source()
        + (typeof node.update == 'function' ? '' : node.update.source()) + ';\n'
        + '}');
    }],

    ['WhileStatement', function (node) {
      node.update('while ' + node.test.source() + '{\n'
        + node.body.source()
        + '\n}');
    }],

    ['ArrayExpression', function (node) {
      node.update('vec![' + node.elements.map(function (a) {
        return a.source()
      }).join(', ') + ']');
    }],

    ['ObjectExpression', function (node) {
      var map = {};
      node.properties.forEach(function (a) {
        map[a.key.source()] = a.value.source();
      })
      if ('keyword' in map || 'binop' in map || 'beforeExpr' in map || 'isUpdate' in map || 'type' in map) {
        map['_id'] = keywordid++;
        map['atomValue'] = map['atomValue'] ? 'ATOM_' + map['atomValue'].toUpperCase() : 'ATOM_NULL';
        map['keyword'] = map['keyword'] || '""'
        map['beforeExpr'] = map['beforeExpr'] || 'false'
        map['binop'] = map['binop'] || '-1';
        map['isAssign'] = map['isAssign'] || 'false'
        map['isLoop'] = map['isLoop'] || 'false'
        map['isUpdate'] = map['isUpdate'] || 'false'
        map['prefix'] = map['prefix'] || 'false'
        map['postfix'] = map['postfix'] || 'false'
        map['type'] = map['type'] || '""'
      }

      if ('sourceFile' in map || 'directSourceFile' in map) {
        map['sourceFile'] = '""';
        map['directSourceFile'] = '""';
      }

      var keys = Object.keys(map);
      if ('keyword' in map) {
        keys.sort();
      }

      node.update(adjusttype(getType(node.parent)) + ' {' + keys.map(function (key) {
        return key + ': ' + map[key]
      }).join(', ') + '}')
    }],

    ['ExpressionStatement', function (node) {
      // Eliminate "use strict"
      if (node.expression.type == 'Literal') {
        node.update('');
      }
    }],

    ['Literal', function (node) {
      if (node.value instanceof RegExp) {
        if (node.value.source == "^[gmsiy]*$") {
          var idx = hoistedregex.push(makeregex('regex_' + hoistedregex.length, 'gmsiy', 'break;', 'return false;', 'return true;'))
          node.update('regex_' + (idx - 1))
        }

        else if (node.value.source == "^[0-7]+") {
          var idx = hoistedregex.push(makeregex('regex_' + hoistedregex.length, '0-7', 'return true;', 'return false;', 'return false;'))
          node.update('regex_' + (idx - 1))
        }

        else if (node.value.source == "[89]") {
          var idx = hoistedregex.push(makeregex('regex_' + hoistedregex.length, '89', 'return true;', 'break;', 'return false;'))
          node.update('regex_' + (idx - 1))
        }

        else {
          var chars = JSON.parse('"' + node.value.source.replace(/\\x(..)/g, '\\u00$1').slice(1, -1) + '"');
          var idx = hoistedregex.push(makeregex('regex_' + hoistedregex.length, chars, 'return true;', 'break;', 'return false;'))
          node.update('regex_' + (idx - 1))
        }
      }

      else if (typeof node.value == 'string' && node.parent.type == 'BinaryExpression' && node.parent.operator == '+') {
        node.update(JSON.stringify(node.value));
      }

      // Eliminate 'string' for "string"
      else if (typeof node.value == 'string') {
        node.update(JSON.stringify(node.value).replace(/\\b/g, '\\u0008').replace(/\\f/g, '\\u000c'));
      }
    }],
  ]);

  // Populate _keyword constants with numeric ids instead of objects.
  out.match(/_(\w+) = \w+ \{_id: (\d+)/g).forEach(function (a) {
    (function (_, t, a, b) {
      keywordids[a] = Number(b);
    }).apply(null, a.match(/_(\w+) = \w+ \{_id: (\d+)/));
  });
  replace(/case\s*_(\w+):/g, function (a, name) {
    return 'case ' + keywordids[name] + ':';
  });

  // Manual source code hacks.
  replace(/.length(?!\()\b/g, '.length()');
  replace(/\.(default|static|operator|type)\b/g, '._$1'); // default is an inappropriate key name
  replace(/\b(default|static|operator|type)\s*([=:!])/g, '_$1 $2'); // default is an inappropriate key name
  replace(/([=:])\s*(default|static|operator|type)\b/g, '$1 _$2'); // default is an inappropriate key name
  replace(/\b(default|static|operator|type)\.\b/g, '_$1.'); // default is an inappropriate key name
  replace(/\(type\b/g, '(_type'); // default is an inappropriate key name
  replace(/\btype\)/g, '_type)'); // default is an inappropriate key name
  replace(/(labels|declarations|properties|params|bodylist|exprList)\.length/g, '$1.size')
  // replace(/\b(node|loc|label|method|elem|init|prop|cur|clause|param|expr|decl|id|argument|val|key|other|body|stmt|classBody|expression|block|(?:specifiers|exprList|declarations)\[(\d+|i)\])\.(?=\w+)/g, '$1.');
  replace("if (options.directSourceFile)", "if (options.directSourceFile.length() > 0)");
  replace(/if \(octal\) \{/g, 'if (octal.length() > 0) {')
  replace("keywordTypes[word]", "keywordTypes(word)");
  replace(/labels:(\S+) = Vec<int>/g, 'mut labels:$1 = Vec<label_t>');
  replace(/(parseArrowExpression|->(?:quasis|specifiers) =|exprList =)(.*)Vec<int>/g, '$1$2Vec<Box<Node>>');
  replace(/(cases|consequents?|exprList|nodes|blocks|empty|bodylist|declarations|expressions|properties|params|elts|defaults) = Vec<int>/g, '$1 = Vec<Box<Node>>')
  replace('push(labels, Box<Node> {name: maybeName, kind: kind})', 'push(labels, label_t {kind: kind, name: maybeName})');
  replace("match (starttype) {", "match (starttype._id) {")
  replace("match (tokType", "match \(tokType._id");
  replace(/const mut/g, 'static');
  replace(/(__object|null) \{\}/g, '""');
  replace(/keyword_t = (null|__object) \{/g, 'keyword_t = keyword_t {')
  replace(/&str = Box<Node> \{\}/g, '&str = ""');
  replace(/Box<Node> = Node\(\)/g, 'Box<Node> = box Node { start: 0 }')
  replace(/label_t = null/g, 'label_t = label_t');
  replace(/\bstrict\b/g, '_strict');
  replace(/TokTypes = null/g, 'TokTypes = TokTypes');

  // TODO: properly have tokVals.
  replace(/int val/gm, "f64 val");
  replace(/int total/gm, "f64 total");
  replace(/auto value = new RegExp.*/gm, "auto value = content;");

  // Hardcoded C code in comments.
  replace(/\/\*C(.*)\*\//g, '$1');

  // Output file.
  console.log('use helper::*;\n\n' + hoistedregex.join('\n') + out.replace(/^;/mg, ''));
  fs.writeFileSync(__dirname + '/output.js', out)
});

function isTopLevel (node) {
  while (node.parent) {
    if (node.type == 'FunctionExpression' || node.type == 'FunctionDeclaration') {
      return false;
    }
    node = node.parent;
  }
  return true;
}

function onlyUnique(value, index, self) { 
    return self.indexOf(value) === index;
}

function makeregexbody (chars, truecase, defaultcase, lastcase) {
  var fn = ['for c in arg.utf16_units() {', 'match c { '];
  fn.push((chars.match(/[\s\S]\-[\s\S]|[\s\S]/g) || []).map(function (onecase) {
    if (onecase.length == 3) {
      return '0x' + onecase.charCodeAt(0).toString(16) + ' ... 0x' + onecase.charCodeAt(2).toString(16);
    } else {
      return '0x' + onecase.charCodeAt(0).toString(16);
    }
  }).join(' | '));
  // remove duplicate cases
  fn = fn.filter(onlyUnique);
  fn.push('=> {', truecase, '}, _ => {', defaultcase, ' } } } ', lastcase);
  return fn.join(' ');
}

function makeregex (name, chars, truecase, defaultcase, lastcase) {
  return ['fn', name, '(arg:&str) -> bool {', makeregexbody(chars, truecase, defaultcase, lastcase), '}'].join(' ');
}

function makeregexlambda (chars, truecase, defaultcase, lastcase) {
  return ['([](std::string arg)->bool { ', makeregexbody(chars, truecase, defaultcase, lastcase), '})'].join(' ');
}

function adjusttype (t) {
  if (t == 'Node') return 'Box<Node>';
  if (t == 'Token') return 'keyword_t';
  if (t == 'Array' || t == '[]') return 'Vec<Box<Node>>';
  if (t == 'boolean') return 'bool';
  if (t == 'string') return '&str';
  if (t == 'number') return 'int';
  if (t == 'void') return 'int';
  if (t == 'Options') return 'options_t';
  if (t == 'Label') return 'label_t';
  if (t == 'Position') return 'int';
  return t;
}

function isinvalidtype (t) {
  return ['any', '__object', 'Function', 'RegExp', 'RegExpExecArray', 'raise'].indexOf(t) > -1 || t.indexOf('[]') > -1;
}
