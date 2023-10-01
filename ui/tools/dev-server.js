const express = require('express');
const app = express();
const port = 8080;
const {execSync} = require('node:child_process');

console.log('building UI for dev...');
const fs = require('fs');
if (!fs.existsSync('dist')) {
    console.log('creating dist dir...');
    fs.mkdirSync('dist');
}
console.log('dist dir exists.');
fs.copyFileSync('index.html', 'dist/index.html');
console.log('copied index.html to dist/');
console.log('building index.js...');
execSync([
    './node_modules/.bin/esbuild',
    '--bundle',
    '--outfile=dist/index.js',
    '--sourcemap=inline',
    'src/main.js',
].join(' '));
console.log('built index.js.');
console.log('built UI for dev.');

app.use(express.static('dist'));
let listener = app.listen(port).address();
console.log(`listening at ${listener.address}:${listener.port}`);
