# Latest Node.js version: https://nodejs.org/en
ARG NODE_VERSION="23.0.0"

# Latest Chrome version: https://www.ubuntuupdates.org/package/google_chrome/stable/main/base/google-chrome-stable
ARG CHROME_VERSION="130.0.6723.58-1"

# Latest Firefox version: https://www.mozilla.org/en-US/firefox/releases/
ARG FIREFOX_VERSION="131.0.3"

# Disable other browsers
ARG EDGE_VERSION=
ARG YARN_VERSION=
ARG CYPRESS_VERSION=

# Latest cypress factory version: https://hub.docker.com/r/cypress/factory/tags
FROM cypress/factory:4.2.2

# Latest NPM version: https://www.npmjs.com/package/npm
ARG NPM_VERSION="10.9.0"

USER root

RUN [ "$(npm --version)" = "${NPM_VERSION}" ] || npm install --verbose -g npm@${NPM_VERSION}

WORKDIR /ci
COPY ./ui/package.json ./ui/
COPY ./ui/package-lock.json ./ui/
RUN cd ui; npm install --verbose
