let dependencies = new Map();

export function inject(id, obj) {
    dependencies.set(id, obj);
}

export function extract(id) {
    let o = dependencies.get(id);
    dependencies.delete(id);
    return o;
}
