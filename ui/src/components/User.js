import { component, render } from "reefjs";
import { user, signal as userSignal } from "../store/user";

export class User extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        this.component = component(
            this,
            function () {
                return `${
                    user.username() != ""
                        ? `
                <wiki-signed-in-user></wiki-signed-in-user>
            `
                        : `
                <wiki-signed-out-user></wiki-signed-out-user>
            `
                }`;
            },
            { signals: [userSignal] },
        );
    }

    disconnectedCallback() {
        this.component.stop();
    }
}
customElements.define("wiki-user", User);

export class SignedInUser extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        render(
            this,
            `
            Username: ${user.username()}
            <button onclick="signOut()">Sign Out</button>
        `,
            {
                signOut: () => {
                    user.signOut();
                },
            },
        );
    }
}
customElements.define("wiki-signed-in-user", SignedInUser);

export class SignedOutUser extends HTMLElement {
    constructor() {
        super();
    }

    connectedCallback() {
        let username, password, signInButton;

        render(
            this,
            `
            <label for="username">Username</label>
            <input id="username" />
            <label for="password">Password</label>
            <input id="password" type="password" />
            <button onclick="signIn()">Sign In</button>
        `,
            {
                signIn: () => {
                    signInButton.setAttribute("disabled", null);
                    user.signIn(username.value, password.value).catch((err) => {
                        console.error(err);
                        signInButton.removeAttribute("disabled");
                    });
                },
            },
        );

        username = this.querySelector("#username");
        password = this.querySelector("#password");
        signInButton = this.querySelector("button");
    }
}
customElements.define("wiki-signed-out-user", SignedOutUser);
