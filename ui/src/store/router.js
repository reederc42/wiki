import { store } from "reefjs";

export let route = store(location.href.substring(location.origin.length));

export function navigate(event) {
    event.preventDefault();
    let target = event.target;
    if (target.hasAttribute("href")) {
        let href = target.getAttribute("href");
        console.log(`navigating to ${href}`);
        route.value = href;
        history.pushState({}, null, route.value);
    } else {
        console.error("target is missing href");
    }
}

addEventListener("popstate", () => {
    route.value = location.href.substring(location.origin.length);
    console.log(`navigating to ${route.value}`);
});
