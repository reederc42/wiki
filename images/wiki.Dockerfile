FROM alpine:3 as certs

RUN apk update &&\
    apk upgrade &&\
    apk add --no-cache \
        ca-certificates

RUN update-ca-certificates

FROM scratch

COPY --from=certs /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs

COPY ./target/release/tools .

ENTRYPOINT [ "/tools" ]
CMD [ "update-mime-types" ]
