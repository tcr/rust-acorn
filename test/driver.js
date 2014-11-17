var finished = 0;
var successes = 0;

var tests = [];

exports.test = function (code, expected, opts) {
  // console.log(name);
  var origin = (new Error).stack.match(/tests\.js:\d+/)[0];
  tests.push(function (next) {
    var acorn = require('child_process').spawn(__dirname + '/../target/acorn', ['-']);
    acorn.stdin.write(code);
    acorn.stdin.end();
    var data = [];
    var exitcode = null;
    // acorn.stderr.pipe(process.stderr);
    acorn.stdout.on('data', function (chunk) {
      data.push(chunk);
    })
    acorn.on('error', function () {
      console.log('boo')
    })

    var promise = 0;
    acorn.on('exit', function (_code) {
      exitcode = _code;
      if ((promise += 1) == 2) finish();
    });
    acorn.stdout.on('finish', function () {
      if ((promise += 1) == 2) finish();
    });
    setTimeout(function () {
      if (promise < 2) {
        acorn.kill();
        console.error('not ok - ERROR: killed', origin)
        finished += 1;
        promise = -1;
        next();
      }
    }, 1000);

    function finish () {
      var result = null;
      try {
        result = JSON.parse(Buffer.concat(data).toString().replace(/_type/g, 'type'));
      } catch (e) { }

      var test = misMatch(expected, result);
      if (!test) {
        successes += 1;
        console.log('ok', '-', origin, '(exit code ' + exitcode + ')')
      } else {
        console.log('not ok', '-', origin, '(exit code ' + exitcode + ')')
      }
      finished += 1;
      next();
      // console.error(total);
      // next();
    }
  })
  return true;
};

exports.testAssert = function () {
};

exports.testFail = function () {
  // tests.push(function (next) {
  //   successes += 1;
  //   total += 1;
  //   next();
  // })
};

process.nextTick(function () {
  var total = tests.length;
  console.log('1..' + total);
  function next () {
    var test = tests.shift();
    if (test) {
      test(next)
    } else {
      if (finished == total) {
        console.log('# pass  ', successes);
        console.log('# total ', total);
      }
      return;
    }
  }

  for (var i = 0; i < 4; i++) {
    next();
  }
})

function ppJSON(v) { return JSON.stringify(v, null, 2); }
function addPath(str, pt) {
  if (str.charAt(str.length-1) == ")")
    return str.slice(0, str.length-1) + "/" + pt + ")";
  return str + " (" + pt + ")";
}

var misMatch = exports.misMatch = function(exp, act) {
  if (!exp || !act || (typeof exp != "object") || (typeof act != "object")) {
    if (exp !== act) return ppJSON(exp) + " !== " + ppJSON(act);
  } else if (exp.splice) {
    if (!act.slice) return ppJSON(exp) + " != " + ppJSON(act);
    if (act.length != exp.length) return "array length mismatch " + exp.length + " != " + act.length;
    for (var i = 0; i < act.length; ++i) {
      var mis = misMatch(exp[i], act[i]);
      if (mis) return addPath(mis, i);
    }
  } else {
    for (var prop in exp) {
      var mis = misMatch(exp[prop], act[prop]);
      if (mis) return addPath(mis, prop);
    }
  }
};
