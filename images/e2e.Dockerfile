# Latest Node.js version: https://nodejs.org/en
ARG NODE_VERSION="21.7.3"

# Latest Chrome version: https://www.ubuntuupdates.org/package/google_chrome/stable/main/base/google-chrome-stable
ARG CHROME_VERSION="123.0.6312.122-1"

# Latest Firefox version: https://www.mozilla.org/en-US/firefox/releases/
ARG FIREFOX_VERSION="124.0.2"

# Disable other browsers
ARG EDGE_VERSION=
ARG YARN_VERSION=
ARG CYPRESS_VERSION=

FROM cypress/factory:3.5.4

ARG NPM_VERSION="10.5.2"

RUN [ "$(npm --version)" = "${NPM_VERSION}" ] || npm install --verbose -g npm@${NPM_VERSION}

WORKDIR /ci
COPY ./ui/package.json .
COPY ./ui/package-lock.json .
RUN npm install --verbose
