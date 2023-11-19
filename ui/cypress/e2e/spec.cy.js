describe("UI e2e tests", () => {
    it("Views subject list", () => {
        cy.visit("/");

        cy.get("wiki-list-subjects").get("tr").should(($tr) => {
            expect($tr).to.have.length.greaterThan(0);
        });
    });
});
