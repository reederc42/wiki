import { setter } from "reefjs";

export const router = setter(
    { path: location.href.substring(location.origin.length) },
    {
        navigate(router, path) {
            if (router.path == path) {
                console.log("navigating to self");
                return;
            }

            if (!validatePath(path)) {
                console.log("invalid path, navigating to /");
                path = "/";
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

addEventListener("popstate", () => {
    console.log(
        `popping state to ${location.href.substring(location.origin.length)}`,
    );
    router.set(location.href.substring(location.origin.length));
});

const validSubjectRE = /\/wiki\/.+/;

// validatePath returns if path is valid
// a valid path satisfies one of these conditions:
//   1. root ("/")
//   2. a subject ("/wiki/.*")
function validatePath(path) {
    if (path == "/") {
        return true;
    }

    return validSubjectRE.test(path);
}
