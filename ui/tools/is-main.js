/* eslint-env node */

export function isMain(url) {
    return (
        process.argv.length > 1 &&
        url.endsWith(process.argv[1].replaceAll("\\", "/"))
    );
}
