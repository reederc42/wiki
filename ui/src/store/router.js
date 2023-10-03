import {store} from 'reefjs';

export let route = store(location.href.substring(location.origin.length));

export function navigate(evt) {
    let target = evt.target;
    if (target.hasAttribute("to-link")) {
        route.value = target.getAttribute("to-link");
        history.pushState({}, null, route.value);
    } else {
        console.error("target is missing to-link");
    }
}
