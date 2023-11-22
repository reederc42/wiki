/* eslint no-undef: "off" */

import { loremIpsum } from "lorem-ipsum";

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

    ["In", "Up"].forEach((method) => {
        it(`Edits existing subject from homepage [sign${method}]`, () => {
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
            cy.visit("/");

            cy.get("wiki-list-subjects > div")
                .next()
                .find("a")
                .then(($a) => {
                    let subject = randomElement($a);
                    subject.click();
                    cy.get("wiki-view-subject > h1").contains(
                        subject.textContent,
                    );
                    cy.get("button").contains("Edit").should("be.disabled");
                    if (method == "In") {
                        cy.get("#username").type("bob");
                        cy.get("#password").type("bobpass");
                        cy.get("button").contains("Sign In").click();
                    } else if (method == "Up") {
                        cy.get("#username").type("testUser");
                        cy.get("#password").type("goodpass");
                        cy.get("button").contains("Sign Up").click();
                    }
                    cy.get("button").contains("Edit").should("not.be.disabled");
                    cy.get("button").contains("Save").should("be.disabled");
                    cy.get("button").contains("Edit").click();
                    cy.get("wiki-view-subject > h1")
                        .contains(subject.textContent)
                        .should("not.be.visible");
                    let testText = loremIpsum();
                    cy.get("#ace-editor").type(testText);
                    cy.get("button").contains("View").click();
                    cy.get("wiki-view-subject > h1")
                        .contains(subject.textContent)
                        .should("be.visible");
                    cy.get("wiki-view-subject")
                        .contains(testText)
                        .should("have.length", 1);
                });
        });
    });

    ["In", "Up"].forEach((method) => {
        it(`Creates new subject from homepage [sign${method}]`, () => {
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
    });

    ["Out", "In", "Up"].forEach((userState) => {
        it(`Creates existing subject directly [signed ${userState}]`, () => {
            // Steps:
            // 1. Visit create new with existing subject
            // 1. See redirect
        });
    });

    ["In", "Up"].forEach((method) => {
        it(`Creates new subject directly [sign${method}]`, () => {
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
    });

    ["In", "Up"].forEach((method) => {
        it(`Creates existing subject from homepage [sign${method}]`, () => {
            // Steps:
            // 1. Visit create new
            // 2. See cannot edit
            // 3. Sign in/up
            // 4. Add existing title
            // 5. Save
            // 6. See error
        });
    });

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
