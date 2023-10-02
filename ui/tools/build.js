const {execSync} = require('node:child_process');
const fs = require('fs');
const outdir = 'dist';
const esbuildBin = './node_modules/.bin/esbuild';

const baseOptions = [
    '--bundle',
    `--outfile=${outdir}/index.js`,
    'src/main.js'
];

function build(options=[], env='') {
    console.log(`building UI for ${env}...`);
    console.log(`resetting build dir: ${outdir}...`);
    fs.rmSync(outdir, {recursive: true, force: true});
    fs.mkdirSync(outdir);
    console.log(`reset build dir: ${outdir}.`);
    console.log(`copying index.html to ${outdir}...`);
    fs.copyFileSync('index.html', `${outdir}/index.html`);
    console.log(`copied index.html to ${outdir}.`);
    console.log('building index.js...');
    let cmd = [esbuildBin].concat(options).concat(baseOptions).join(' ');
    console.log(`executing command: ${cmd}`);
    execSync(cmd);
    console.log('built index.js.');
    console.log(`built UI for ${env}`);
}

module.exports = build;
