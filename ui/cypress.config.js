/* eslint-env node */

import { defineConfig } from "cypress";

export default defineConfig({
    e2e: {
        baseUrl: "http://localhost:8080",
        env: {
            USER_EXPIRATION: 3000,
            REQUIRE_CLEAN_PERSISTENCE: false,
        },
    },
    reporter: "cypress-multi-reporters",
    reporterOptions: {
        reporterEnabled: "spec, mocha-junit-reporter",
        mochaJunitReporterReporterOptions: {
            mochaFile: process.env.CYPRESS_MOCHA_FILE || "./e2e-test-local.xml",
        },
    },
});
