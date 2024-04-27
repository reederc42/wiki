/* eslint no-unused-vars: "off" */

import config from "../../config.json";
import axios from "axios";

export const subject = {
    list() {
        return new Promise((resolve, reject) => {
            reject(new Error("unimplemented"));
        });
    },

    get(subject) {
        return new Promise((resolve, reject) => {
            reject(new Error("unimplemented"));
        });
    },

    put(subject, content) {
        return new Promise((resolve, reject) => {
            reject(new Error("unimplemented"));
        });
    },

    post(subject, content) {
        return new Promise((resolve, reject) => {
            reject(new Error("unimplemented"));
        });
    },
};
