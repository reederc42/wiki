/* eslint no-undef: "off" */

import axios from "axios";
import { loremIpsum } from "lorem-ipsum";

import { newSubject, existingSubject } from "../../src/test-helpers/mock/api";
import { newUser, existingUser } from "../../src/test-helpers/mock/auth";
import mockSubjects from "../../src/api/mock/subjects.json";

const maxTextLength = 256;
const userSignInWait = 1000;
const userExpiration = Number(Cypress.env("USER_EXPIRATION")) + 250;

describe("UI e2e tests", () => {
    before(async () => {
        if (Cypress.env("API_URL")) {
            let token;
            if (!Cypress.env("AUTH_URL")) {
                let { user, pass } = existingUser();
                token = `Basic ${user}:${pass}`;
            } else {
                throw new Error("auth url not supported");
            }

            for (const k in mockSubjects) {
                await axios
                    .post(
                        Cypress.env("API_URL") + "/subject/" + k,
                        mockSubjects[k],
                        {
                            headers: {
                                Authorization: token,
                                "Content-Type": "text/plain",
                            },
                        },
                    )
                    .catch((err) => {
                        if (Cypress.env("REQUIRE_CLEAN_PERSISTENCE")) {
                            throw err;
                        }
                    });
            }
        }
    });

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
                    cy.wait(userSignInWait);

                    // 5. Edit subject
                    cy.get("button").contains("Edit").should("not.be.disabled");
                    cy.get("button").contains("Save").should("be.disabled");
                    cy.get("button").contains("Edit").click();
                    cy.get("wiki-view-subject > h1")
                        .contains(subject.textContent)
                        .should("not.be.visible");
                    cy.get("textarea").should("not.have.attr", "readonly");
                    let testText = randomText();
                    cy.get("#ace-editor")
                        .get(".ace_content")
                        .click()
                        .type(testText);

                    // 6. Save changes
                    cy.get("button").contains("Save").click();
                    cy.wait(500);

                    // 7. See changes
                    cy.get("button").contains("View").click();
                    cy.get("wiki-view-subject > h1")
                        .contains(subject.textContent)
                        .should("be.visible");
                    cy.get("wiki-view-subject")
                        .contains(testText)
                        .should("exist");

                    // 8. Visit home
                    cy.get("a").contains("Wiki").click();

                    // 9. Visit same subject
                    cy.get("a").contains(subject.textContent).click();

                    // 10. See changes
                    cy.get("wiki-view-subject")
                        .contains(testText)
                        .should("exist");
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
            cy.wait(userSignInWait);

            // 5. Edit subject
            cy.get("textarea").should("not.have.attr", "readonly");
            let testText = newSubject();
            cy.get("#ace-editor")
                .click()
                .type("# " + testText, { delay: 2 });

            // 6. Save subject
            cy.get("button").contains("Save").click();
            cy.wait(500);

            // 7. See changes
            cy.get("button").contains("View").click();
            cy.get("wiki-view-subject").contains(testText).should("exist");

            // 8. Visit home
            cy.get("a").contains("Wiki").click();

            // 9. See new subject
            cy.get("wiki-list-subjects > div")
                .next()
                .find("a")
                .should("have.length.greaterThan", 1);
            cy.get("a").contains(testText).should("exist");

            // 10. Visit same subject
            cy.get("a").contains(testText).click();

            // 11. See content
            cy.get("wiki-view-subject").contains(testText).should("exist");
        });
    });

    // TODO: mock user should use local storage
    ["Out", "In", "Up"].forEach((userState) => {
        it(`Creates existing subject directly [signed ${userState}]`, () => {
            if (userState == "Up") {
                cy.log("Mock auth does not support reloading new users");
                return;
            }

            if (userState == "In") {
                let user = existingUser();
                cy.visit("/");
                cy.get("#username").type(user.name);
                cy.get("#password").type(user.pass);
                cy.get("button").contains("Sign In").click();
                cy.wait(userSignInWait);
            }

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
            cy.wait(userSignInWait);

            // 6. Save has no error
            cy.get("button").contains("Save").click();

            // 7. Visit homepage
            cy.get("a").contains("Wiki").click();

            // 8. See new subject
            cy.get("wiki-list-subjects > div")
                .next()
                .find("a")
                .should("have.length.greaterThan", 1);
            cy.get("a").contains(subject).should("exist");
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
            cy.wait(userSignInWait);

            // 4. Add existing title
            cy.get("textarea").should("not.have.attr", "readonly");
            let subject = existingSubject();
            cy.get("#ace-editor")
                .click()
                .type("# " + subject);

            // 5. Save
            cy.get("button").contains("Save").click();

            // 6. See error
            cy.get("#save-error").should("be.visible");
        });
    });

    [
        { method: "In", user: existingUser() },
        { method: "Up", user: newUser() },
    ].forEach((t) => {
        it(`Reload preserves sign in state [sign${t.method}]`, () => {
            // 1. Visit homepage
            cy.visit("/");

            // 2. Sign in/up
            cy.get("#username").type(t.user.name);
            cy.get("#password").type(t.user.pass);
            cy.get("button").contains(`Sign ${t.method}`).click();
            cy.wait(userSignInWait);
            cy.get("wiki-user").contains(t.user.name).should("exist");

            // 3. Reload
            cy.reload();

            // 4. See signed in
            cy.get("wiki-user").contains(t.user.name).should("exist");

            // 5. Sign out
            cy.get("button").contains("Sign Out").click();

            // 6. Reload
            cy.reload();

            // 7. See signed out
            cy.wait(userSignInWait);
            cy.get("wiki-user").contains(t.user.name).should("not.exist");
        });
    });

    [
        { method: "In", user: existingUser() },
        { method: "Up", user: newUser() },
    ].forEach((t) => {
        it(`Is signed out after reload after expiration [sign${t.method}]`, () => {
            // 1. Visit homepage
            cy.visit("/");

            // 2. Sign in/up
            cy.get("#username").type(t.user.name);
            cy.get("#password").type(t.user.pass);
            cy.get("button").contains(`Sign ${t.method}`).click();
            cy.wait(userSignInWait);

            // 3. Wait until expiration
            cy.wait(userExpiration);

            // 4. Reload
            cy.reload();

            // 5. See signed out
            cy.wait(userSignInWait);
            cy.get("wiki-user").contains(t.user.name).should("not.exist");
        });
    });

    [
        { method: "In", user: existingUser() },
        { method: "Up", user: newUser() },
    ].forEach((t) => {
        it(`Is signed out after save after expiration [sign${t.method}]`, () => {
            // 1. Visit create new subject
            cy.visit(`/wiki-new/${newSubject()}`);

            // 2. Sign in/up
            cy.get("#username").type(t.user.name);
            cy.get("#password").type(t.user.pass);
            cy.get("button").contains(`Sign ${t.method}`).click();
            cy.wait(userSignInWait);

            // 3. Add new content
            cy.get("textarea").should("not.have.attr", "readonly");
            cy.get("#ace-editor").click().type(randomText());

            // 4. Save
            cy.get("button").contains("Sign Out").click();
            cy.get("#username").type(t.user.name);
            cy.get("#password").type(t.user.pass);
            cy.get("button").contains("Sign In").click();
            cy.wait(userSignInWait);
            cy.get("button").contains("Save").click();
            cy.wait(500);

            // 5. Add more new content
            cy.log("Trying to add more content");
            cy.get("textarea").should("not.have.attr", "readonly");
            cy.get("#ace-editor")
                .click()
                .type("{moveToEnd}{enter}{enter}" + randomText());

            // 6. Wait until expiration
            cy.wait(userExpiration);

            // 7. Save
            cy.get("button").contains("Save").click();

            // 8. See save error
            cy.get("#save-error").should("be.visible");

            // 9. See signed out
            cy.get("wiki-user").contains(t.user.name).should("not.exist");
        });
    });
});

function randomElement(list = []) {
    return list[Math.floor(Math.random() * list.length)];
}

function randomText(length = maxTextLength) {
    return loremIpsum().slice(0, length);
}
