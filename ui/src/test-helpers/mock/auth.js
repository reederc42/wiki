let id = 0;

export function newUser() {
    return {
        name: "test-" + id++,
        pass: "goodpass",
    };
}

export function existingUser() {
    return {
        name: "bob",
        pass: "bobpass",
    };
}
