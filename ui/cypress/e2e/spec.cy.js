/* eslint no-undef: "off" */

describe("UI e2e tests", () => {
    it("Views subject list", () => {
        // Steps:
        // 1. Visit homepage
        // 2. See list of subjects
        cy.visit("/");

        cy.get("wiki-list-subjects")
            .get("tr")
            .should("have.length.greaterThan", 0);
    });

    it("Edits existing subject from homepage", () => {
        // Steps:
        // 1. Visit homepage
        // 2. Visit random subject
        // 3. See cannot edit
        // 4. Sign in/up
        // 5. Edit subject
        // 6. Save subject
        // 7. See changes
        // 8. Visit home
        // 9. Visit same subject
        // 10. See changes
    });

    it("Creates new subject from homepage", () => {
        // Steps:
        // 1. Visit homepage
        // 2. Create new subject
        // 3. See cannot edit
        // 4. Sign in/up
        // 5. Edit subject
        // 6. Save subject
        // 7. See changes
        // 8. Visit home
        // 9. See new subject
        // 10. Visit same subject
        // 11. See content
    });

    it("Creates existing subject directly", () => {
        // Steps:
        // 1. Visit create new with existing subject
        // 1. See redirect
    });

    it("Creates new subject directly", () => {
        // Steps:
        // 1. Visit create new with new subject
        // 2. See cannot edit
        // 3. Sign in/up
        // 4. Add new content
        // 5. See changes
        // 6. Save has no error
        // 7. Visit homepage
        // 8. See new subject
    });

    it("Creates existing subject from homepage", () => {
        // Steps:
        // 1. Visit create new
        // 2. See cannot edit
        // 3. Sign in/up
        // 4. Add existing title
        // 5. Save
        // 6. See error
    });

    it("Reloads preserves sign in state", () => {
        // Steps:
        // 1. Visit homepage
        // 2. Sign in/up
        // 3. Reload
        // 4. See signed in
        // 5. Sign out
        // 6. Reload
        // 7. See signed out
    });

    it("Is signed out after reload after expiration", () => {
        // Steps:
        // 1. Visit homepage
        // 2. Sign in/up
        // 3. Wait until expiration
        // 4. Reload
        // 5. See signed out
    });

    it("Is signed out after save after expiration", () => {
        // Steps:
        // 1. Visit create new subject
        // 2. Sign in
        // 3. Add new content
        // 4. Save
        // 5. Add more new content
        // 6. Wait until expiration
        // 7. Save
        // 8. See signed out
    });
});
