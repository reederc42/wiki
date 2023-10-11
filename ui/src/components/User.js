import {component} from "reefjs";
import {user} from "../store/user";

export class User extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        component(this, function() {
            return `${user.data.username != "" ? `
                <wiki-signed-in-user></wiki-signed-in-user>
            ` : `
                <wiki-signed-out-user></wiki-signed-out-user>
            `}`
        });
    }
}
customElements.define("wiki-user", User);

export class SignedInUser extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        function signOut() {
            user.signOut();
        }
        component(this, function() {
            return `
                Username: ${user.data.username}
                <button onclick="signOut()">Sign Out</button>
            `
        }, {events: {signOut}});
    }
}
customElements.define("wiki-signed-in-user", SignedInUser);

export class SignedOutUser extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        let root = this;
        function signIn() {
            let username = root.querySelector("#username");
            user.signIn(username.value);
        }
        component(this, function() {
            return `
                <label for="username">Username</label>
                <input id="username" />
                <label for="password">Password</label>
                <input id="password" />
                <button onclick="signIn()">Sign In</button>
            `
        }, {events: {signIn}});
    }
}
customElements.define("wiki-signed-out-user", SignedOutUser);
