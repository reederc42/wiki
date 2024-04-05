ARG NODE_VERSION="21.7.1"

FROM node:${NODE_VERSION}

ARG NPM_VERSION="10.5.1"
ARG RUST_VERSION="1.77.1"

RUN npm install --verbose -g npm@${NPM_VERSION}

USER 1000:1000

ENV PATH=$PATH:/home/node/.cargo/bin
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain ${RUST_VERSION}

WORKDIR /ci
COPY --chown=1000:1000 ./ui/package.json .
COPY --chown=1000:1000 ./ui/package-lock.json .
RUN npm install --verbose
