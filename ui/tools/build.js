import * as fs from 'fs';
import * as esbuild from 'esbuild';

const outdir = 'dist';

export async function build(options={}, env='') {
    fs.rmSync(outdir, {
        recursive: true,
        force: true
    });

    fs.mkdirSync(outdir);
    fs.cpSync('public', outdir, {
        recursive: true
    });

    let baseOptions = {
        entryPoints: ['src/main.js'],
        bundle: true,
        outfile: outdir + '/index.js',
        logLevel: 'info'
    }
    await esbuild.build({
        ...baseOptions,
        ...options
    });

    console.log(`finishing building for ${env}`);
}

export function buildSync(options={}, env='') {
    console.log(options);
    let baseOptions = {
        bundle: true,
        logLevel: 'info'
    }
    let realOptions = {
        ...baseOptions,
        ...options
    }
    console.log(realOptions);
    esbuild.buildSync({
        ...baseOptions,
        ...options
    });

    console.log(`finished building for ${env}`);
}