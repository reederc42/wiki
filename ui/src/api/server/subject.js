import axios from "axios";

import config from "../../config.json";
import { user } from "../../store/user";

export const subject = {
    list() {
        return axios.get(config.apiBaseUrl + "/subjects/").then((response) => {
            return response.data.split("\n").map((v) => decodeURIComponent(v));
        });
    },

    get(subject) {
        return axios
            .get(config.apiBaseUrl + "/subject/" + subject)
            .then((response) => response.data)
            .catch((err) => {
                if (err.response.status == 404) {
                    throw new Error("not found");
                } else {
                    throw err;
                }
            });
    },

    put(subject, content) {
        let url = config.apiBaseUrl + "/subject/" + subject;
        return doAuthRequest(axios.patch, url, content).catch((err) => {
            if (!err.response || err.response.status != 401) {
                throw err;
            }
            return user.signIn(user.username(), "", user.refresh()).then(() => {
                console.log("signed back in " + user.username());
                return doAuthRequest(axios.patch, url, content);
            });
        });
    },

    post(subject, content) {
        let url = config.apiBaseUrl + "/subject/" + subject;
        return doAuthRequest(axios.post, url, content).catch((err) => {
            if (!err.response || err.response.status != 401) {
                throw err;
            }
            return user.signIn(user.username(), "", user.refresh()).then(() => {
                console.log("signed back in " + user.username());
                return doAuthRequest(axios.post, url, content);
            });
        });
    },
};

function doAuthRequest(axiosHelper, url, content) {
    return axiosHelper(url, content, {
        headers: {
            Authorization: "Bearer " + user.token(),
            "Content-Type": "text/plain",
        },
    });
}
