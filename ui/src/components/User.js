import {component} from 'reefjs';
import {user} from '../store/user';

class WikiUser extends HTMLElement {
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
customElements.define("wiki-user", WikiUser);

class SignedInUser extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        let events = {
            signOut() {
                user.signOut();
            }
        };
        component(this, function() {
            return `
                Username: ${user.data.username}
                <button onclick="signOut()">Sign Out</button>
            `
        }, {events});
    }
}
customElements.define("wiki-signed-in-user", SignedInUser);

class SignedOutUser extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        let events = {
            signIn() {
                let username = this
                    .parentNode
                    .parentNode
                    .querySelector("#username");
                user.signIn(username.value);
            }
        };
        component(this, function() {
            return `
                <label for="username">Username</label>
                <input id="username" />
                <label for="password">Password</label>
                <input id="password" />
                <button onclick="signIn()">Sign In</button>
            `
        }, {events});
    }
}
customElements.define("wiki-signed-out-user", SignedOutUser);
