var acorn = require('acorn');
var _ = require('underscore');
var fs = require('fs');
var exec = require('child_process').exec;

var input = fs.readFileSync(__dirname + '/input.js', 'utf-8');
var output = JSON.parse(JSON.stringify(acorn.parse(input)))

// var tab = '  '
var tab = null

exec('./target/acorn', function (err, stdout, stderr) {
	var compare = JSON.parse(stdout.replace(/_type/g, 'type'))
	console.log(_.isEqual(output, compare));
	console.error('\nacorn:', JSON.stringify(output, null, tab));
	console.error('\nrust: ', JSON.stringify(compare, null, tab));
	process.exit(_.isEqual(output, compare) ? 0 : 1)
})
