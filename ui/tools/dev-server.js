const build = require('./build');
const express = require('express');

const app = express();
const port = 8080;

build(['--sourcemap=inline'], 'dev');

let dist = process.cwd() + "/dist";

app.get('/index.js', (req, res) => {
    res.sendFile(`${dist}/index.js`);
});

app.get('/*', (req, res) => {
    res.sendFile(`${dist}/index.html`);
})

let listener = app.listen(port).address();
console.log(`listening at ${listener.address}:${listener.port}`);
