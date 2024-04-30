import { defineConfig } from "cypress";

export default defineConfig({
    e2e: {
        baseUrl: "http://localhost:8080",
        env: {
            USER_EXPIRATION: 1500,
            REQUIRE_CLEAN_PERSISTENCE: false,
        },
    },
});
