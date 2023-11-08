import { store } from "reefjs";

export const signal = "router";

export const router = store(
    { path: location.href.substring(location.origin.length) },
    {
        navigate(router, path) {
            if (!validatePath(path)) {
                console.log("invalid path, navigating to /");
                path = "/";
            }

            if (router.path == path) {
                console.log("navigating to self");
                return;
            }

            router.path = path;
            history.pushState(null, null, path);
            console.log(`navigating to ${router.path}`);
        },

        set(router, path) {
            if (!validatePath(path)) {
                console.log("invalid path, not setting path");
                return;
            }

            router.path = path;
            console.log(`setting path to ${path}`);
        },
    },
    signal,
);

export function navigate(event) {
    event.preventDefault();

    let target = event.target;
    if (target.hasAttribute("href")) {
        let href = target.getAttribute("href");
        router.navigate(href);
    } else {
        console.error("target is missing href");
    }
}

export function getSubject() {
    let path = router.value.path;
    if (path.startsWith("/wiki-new")) {
        return path.substring("/wiki-new/".length);
    }
    return path.substring("/wiki/".length);
}

export function isNew() {
    return router.value.path.startsWith("/wiki-new");
}

addEventListener("popstate", () => {
    console.log(
        `popping state to ${location.href.substring(location.origin.length)}`,
    );
    router.set(location.href.substring(location.origin.length));
});

const validSubjectRE = /\/wiki\/.+/;
const validNewSubjectRE = /\/wiki-new\/.+/;

// validatePath returns if path is valid
// a valid path satisfies one of these conditions:
//   1. root ("/")
//   2. a new subject ("/wiki-new")
//   3. a subject ("/wiki/.+")
//   4. a new named subject ("/wiki-new/.+")
function validatePath(path) {
    if (path == "/") {
        return true;
    }

    if (path == "/wiki-new") {
        return true;
    }

    if (validSubjectRE.test(path)) {
        return true;
    }

    return validNewSubjectRE.test(path);
}
