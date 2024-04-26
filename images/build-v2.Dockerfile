# Latest Node.js version: https://nodejs.org/en
ARG NODE_VERSION="21.7.3"

FROM node:${NODE_VERSION}-alpine

# Latest NPM version: https://www.npmjs.com/package/npm
ARG NPM_VERSION="10.5.2"

# Latest Rust version: https://www.rust-lang.org/
ARG RUST_VERSION="1.77.2"

# Latest nextest version: https://github.com/nextest-rs/nextest/releases
ARG NEXTEST_VERSION="^0.9"

USER root

RUN [ "$(npm --version)" = "${NPM_VERSION}" ] || npm install --verbose -g npm@${NPM_VERSION}

RUN apk update &&\
    apk upgrade &&\
    apk add \
    curl \
    gcc \
    musl-dev \
    openssl-dev \
    openssl-libs-static

ENV OPENSSL_STATIC=true
ENV OPENSSL_LIB_DIR=/usr/lib/
ENV OPENSSL_INCLUDE_DIR=/usr/include/

WORKDIR /ci

ENV CARGO_HOME=/ci/.cargo
ENV RUSTUP_HOME=/ci/.rustup
ENV PATH=${PATH}:${CARGO_HOME}/bin
RUN mkdir -p ${CARGO_HOME} ${RUSTUP_HOME}
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs |\
    sh -s -- -y --default-toolchain ${RUST_VERSION}

RUN cargo install cargo-nextest --version ${NEXTEST_VERSION} --locked

COPY ./ui/package.json .
COPY ./ui/package-lock.json .
RUN npm install --verbose

RUN mkdir wiki
RUN cd wiki &&\
    cargo new ci &&\
    cargo new tools &&\
    cargo new wiki
COPY ./Cargo.lock ./wiki/
COPY ./Cargo.toml ./wiki/
COPY ./ci/Cargo.toml ./wiki/ci/
COPY ./tools/Cargo.toml ./wiki/tools/
COPY ./wiki/Cargo.toml ./wiki/wiki/
RUN cd wiki &&\
    cargo fetch -vv
