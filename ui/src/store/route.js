import { setter } from "reefjs";

export const route = setter(location.href.substring(location.origin.length), {
    navigate(route, path) {
        if (route == path) {
            console.log("navigating to self");
            return;
        }

        if (!validatePath(path)) {
            console.log("invalid path, navigating to /");
            path = "/";
        }

        route = path;
        history.pushState(null, null, path);
        console.log(`navigating to ${path}`);
    },

    set(route, path) {
        if (!validatePath(path)) {
            console.log("invalid path, not setting path");
            return;
        }

        route = path;
        console.log(`setting path to ${path}`);
    },
});

export function navigate(event) {
    event.preventDefault();

    let target = event.target;
    if (target.hasAttribute("href")) {
        let href = target.getAttribute("href");
        route.navigate(href);
    } else {
        console.error("target is missing href");
    }
}

addEventListener("popstate", () => {
    route.set(location.href.substring(location.origin.length));
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
