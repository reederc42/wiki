const build = require('./build');

const express = require('express');
const app = express();
const port = 8080;

build(['--sourcemap=inline'], 'dev');

app.use(express.static('dist'));
let listener = app.listen(port).address();
console.log(`listening at ${listener.address}:${listener.port}`);
