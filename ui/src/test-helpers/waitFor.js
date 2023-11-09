import assert from "node:assert";
import { waitFor as domWaitFor } from "@testing-library/dom";

export async function waitFor(
    assertion,
    container,
    options = {},
    message = undefined,
) {
    options.container = container;
    await domWaitFor(() => {
        if (!assertion()) {
            throw new Error("waiting");
        }
    }, options);
    assert(assertion(), message);
}
