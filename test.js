var acorn = require('acorn');
var _ = require('underscore');
var fs = require('fs');
var exec = require('child_process').exec;

var input = fs.readFileSync(__dirname + '/input.js', 'utf-8');
var output = acorn.parse(input);

exec('./target/hello_world', function (err, stdout, stderr) {
	var compare = JSON.parse(stdout.replace(/_type/g, 'type'));
	console.log(_.isEqual(output, compare));
	console.error('\nacorn:', JSON.stringify(output));
	console.error('\nrust: ', JSON.stringify(compare));
	process.exit(_.isEqual(output, compare) ? 0 : 1)
})
