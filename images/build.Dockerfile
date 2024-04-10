ARG NODE_VERSION="21.7.3"

FROM node:${NODE_VERSION}

ARG NPM_VERSION="10.5.2"
ARG RUST_VERSION="1.77.2"

RUN npm install --verbose -g npm@${NPM_VERSION}

USER 1000:1000

ENV PATH=$PATH:/home/node/.cargo/bin
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain ${RUST_VERSION}

WORKDIR /ci
COPY --chown=1000:1000 ./ui/package.json .
COPY --chown=1000:1000 ./ui/package-lock.json .
RUN npm install --verbose

RUN mkdir wiki
RUN cd wiki &&\
    cargo new ci &&\
    cargo new tools &&\
    cargo new wiki
COPY --chown=1000:1000 ./Cargo.lock ./wiki/
COPY --chown=1000:1000 ./Cargo.toml ./wiki/
COPY --chown=1000:1000 ./ci/Cargo.toml ./wiki/ci/
COPY --chown=1000:1000 ./tools/Cargo.toml ./wiki/tools/
COPY --chown=1000:1000 ./wiki/Cargo.toml ./wiki/wiki/
RUN cd wiki &&\
    cargo fetch -vv
