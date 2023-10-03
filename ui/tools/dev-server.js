const build = require('./build');
const fs = require('fs');
const express = require('express');

const app = express();
const port = 8080;

build(['--sourcemap=inline'], 'dev');


app.get('/index.js', (req, res) => {
    res.sendFile('/home/docker/src/github.com/reederc42/wiki/ui/dist/index.js');
});

app.get('/*', (req, res) => {
    res.sendFile('/home/docker/src/github.com/reederc42/wiki/ui/dist/index.html');
})

let listener = app.listen(port).address();
console.log(`listening at ${listener.address}:${listener.port}`);
