ARG NODE_VERSION="21.7.1"
ARG CHROME_VERSION="123.0.6312.58-1"
ARG FIREFOX_VERSION="124.0"
ARG EDGE_VERSION=
ARG YARN_VERSION=
ARG CYPRESS_VERSION=

FROM cypress/factory:3.5.1

USER 1000:1000

WORKDIR /ci
COPY --chown=1000:1000 ./ui/package.json .
COPY --chown=1000:1000 ./ui/package-lock.json .
RUN npm install --verbose