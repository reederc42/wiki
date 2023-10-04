import * as fs from 'fs';
import * as esbuild from 'esbuild';

const outdir = 'dist';

export async function build(options={}, env='') {
    fs.rmSync(outdir, {
        recursive: true,
        force: true
    });

    fs.mkdirSync(outdir);
    fs.cpSync('index.html', outdir + '/index.html');

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
