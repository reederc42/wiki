/* eslint no-undef: "off" */

import { loremIpsum } from "lorem-ipsum";
import { newUser, existingUser } from "../../src/test-helpers/mock/auth";
import { newSubject, existingSubject } from "../../src/test-helpers/mock/api";

describe("UI e2e tests", () => {
    it("Views subject list", () => {
        // 1. Visit homepage
        cy.visit("/");

        // 2. See list of subjects
        cy.get("wiki-list-subjects")
            .get("tr")
            .should("have.length.greaterThan", 0);
    });

    [
        { method: "In", user: existingUser() },
        { method: "Up", user: newUser() },
    ].forEach((t) => {
        it(`Edits existing subject from homepage [sign${t.method}]`, () => {
            // 1. Visit homepage
            cy.visit("/");

            // 2. Visit random subject
            cy.get("wiki-list-subjects > div")
                .next()
                .find("a")
                .then(($a) => {
                    let subject = randomElement($a);
                    subject.click();
                    cy.get("wiki-view-subject > h1").contains(
                        subject.textContent,
                    );

                    // 3. See cannot edit
                    cy.get("button").contains("Edit").should("be.disabled");

                    // 4. Sign in/up
                    cy.get("#username").type(t.user.name);
                    cy.get("#password").type(t.user.pass);
                    cy.get("button").contains(`Sign ${t.method}`).click();

                    // 5. Edit subject
                    cy.get("button").contains("Edit").should("not.be.disabled");
                    cy.get("button").contains("Save").should("be.disabled");
                    cy.get("button").contains("Edit").click();
                    cy.get("wiki-view-subject > h1")
                        .contains(subject.textContent)
                        .should("not.be.visible");
                    cy.get("textarea").should("not.have.attr", "readonly");
                    let testText = loremIpsum();
                    cy.get("#ace-editor").type(testText);

                    // 6. Save changes
                    cy.get("button").contains("Save").click();

                    // 7. See changes
                    cy.get("button").contains("View").click();
                    cy.get("wiki-view-subject > h1")
                        .contains(subject.textContent)
                        .should("be.visible");
                    cy.get("wiki-view-subject")
                        .contains(testText)
                        .should("have.length", 1);

                    // 8. Visit home
                    cy.get("a").contains("Wiki").click();

                    // 9. Visit same subject
                    cy.get("a").contains(subject.textContent).click();

                    // 10. See changes
                    cy.get("wiki-view-subject")
                        .contains(testText)
                        .should("have.length", 1);
                });
        });
    });

    [
        { method: "In", user: existingUser() },
        { method: "Up", user: newUser() },
    ].forEach((t) => {
        it(`Creates new subject from homepage [sign${t.method}]`, () => {
            // 1. Visit homepage
            cy.visit("/");

            // 2. Create new subject
            cy.get("a").contains("new").click();

            // 3. See cannot edit
            cy.get("button").contains("Edit").should("be.disabled");
            cy.get("textarea").should("have.attr", "readonly");

            // 4. Sign in/up
            cy.get("#username").type(t.user.name);
            cy.get("#password").type(t.user.pass);
            cy.get("button").contains(`Sign ${t.method}`).click();

            // 5. Edit subject
            cy.get("textarea").should("not.have.attr", "readonly");
            let testText = newSubject();
            cy.get("#ace-editor").type("# " + testText);

            // 6. Save subject
            cy.get("button").contains("Save").click();

            // 7. See changes
            cy.get("button").contains("View").click();
            cy.get("wiki-view-subject")
                .contains(testText)
                .should("have.length", 1);

            // 8. Visit home
            cy.get("a").contains("Wiki").click();

            // 9. See new subject
            cy.get("wiki-list-subjects > div")
                .next()
                .find("a")
                .should("have.length.greaterThan", 1);
            cy.get("a").contains(testText).should("have.length", 1);

            // 10. Visit same subject
            cy.get("a").contains(testText).click();

            // 11. See content
            cy.get("wiki-view-subject")
                .contains(testText)
                .should("have.length", 1);
        });
    });

    // TODO: mock user should use local storage
    ["Out", "In", "Up"].forEach((userState) => {
        it(`Creates existing subject directly [signed ${userState}]`, () => {
            let subject = existingSubject();
            // 1. Visit create new with existing subject
            cy.visit(`/wiki-new/${encodeURIComponent(subject)}`);

            // 2. See redirect
            cy.url().should(
                "equal",
                Cypress.config().baseUrl +
                    `/wiki/${encodeURIComponent(subject)}`,
            );
        });
    });

    [
        { method: "In", user: existingUser() },
        { method: "Up", user: newUser() },
    ].forEach((t) => {
        it(`Creates new subject directly [sign${t.method}]`, () => {
            // 1. Visit create new with new subject
            let subject = newSubject();
            cy.visit(`/wiki-new/${encodeURIComponent(subject)}`);

            // 2. See cannot edit
            cy.get("button").contains("Edit").should("be.disabled");

            // 3. Sign in/up
            cy.get("#username").type(t.user.name);
            cy.get("#password").type(t.user.pass);
            cy.get("button").contains(`Sign ${t.method}`).click();

            // 6. Save has no error
            cy.get("button").contains("Save").click();

            // 7. Visit homepage
            cy.get("a").contains("Wiki").click();

            // 8. See new subject
            cy.get("wiki-list-subjects > div")
                .next()
                .find("a")
                .should("have.length.greaterThan", 1);
            cy.get("a").contains(subject).should("have.length", 1);
        });
    });

    [
        { method: "In", user: existingUser() },
        { method: "Up", user: newUser() },
    ].forEach((t) => {
        it(`Creates existing subject from homepage [sign${t.method}]`, () => {
            // 1. Visit create new
            cy.visit("/");
            cy.get("a").contains("new").click();

            // 2. See cannot edit
            cy.get("button").contains("Edit").should("be.disabled");

            // 3. Sign in/up
            cy.get("#username").type(t.user.name);
            cy.get("#password").type(t.user.pass);
            cy.get("button").contains(`Sign ${t.method}`).click();

            // 4. Add existing title
            cy.get("textarea").should("not.have.attr", "readonly");
            let subject = existingSubject();
            cy.get("#ace-editor").type("# " + subject);

            // 5. Save
            cy.get("button").contains("Save").click();

            // 6. See error
            cy.get("wiki-subject > span").should("be.visible");
        });
    });

    // TODO: mock user uses local storage
    ["In", "Up"].forEach((method) => {
        it(`Reloads preserves sign in state [sign${method}]`, () => {
            // Steps:
            // 1. Visit homepage
            // 2. Sign in/up
            // 3. Reload
            // 4. See signed in
            // 5. Sign out
            // 6. Reload
            // 7. See signed out
        });
    });

    // TODO: mock user uses local storage
    ["In", "Up"].forEach((method) => {
        it(`Is signed out after reload after expiration [sign${method}]`, () => {
            // Steps:
            // 1. Visit homepage
            // 2. Sign in/up
            // 3. Wait until expiration
            // 4. Reload
            // 5. See signed out
        });
    });

    // TODO: mock user uses local storage
    ["In", "Up"].forEach((method) => {
        it(`Is signed out after save after expiration [sign${method}]`, () => {
            // Steps:
            // 1. Visit create new subject
            // 2. Sign in/up
            // 3. Add new content
            // 4. Save
            // 5. Add more new content
            // 6. Wait until expiration
            // 7. Save
            // 8. See signed out
        });
    });
});

function randomElement(list = []) {
    return list[Math.floor(Math.random() * list.length)];
}
