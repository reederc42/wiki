# Wiki (2.0)

This is wiki 2.0: wikis serve an interconnected set of pages, maintained and
edited by users. Wiki 2.0 is intended to be an example of scalable TDD, using Go
and Javascript to create a well-tested, secure, client/server application.

This monorepo includes all design notes and a set of build and test tools that
are developed similarly to the main application, using TDD.

## Major Technologies

* Rust
    * Rust is used to develop the server and tools
* Javascript
    * Javascript is used to develop the user interface, the client
        * Reef.js is used to assist with reactive components
        * Cypress is used for automated testing
* Postgres
    * Postgres is used to persist ordered data
    * Should be absolutely persistent: loss of pages or user data should be
    avoided
* Redis
    * Redis is used to cache authentication data
    * Is not required to be absolutely persistent
* Kubernetes
    * Wiki is intended to support a distributed, microservices-based deployment
    using K8s
* Docker
    * Wiki will provide several container images built by Docker
* Make
    * Make is used to automate build tasks

## Update dependencies

* Regex search the repository for:

    ```
    Latest .* version
    ```

* Update rust dependencies:

    ```sh
    cargo install-update -a
    cargo upgrade
    ```

* Update node.js dependencies:

    ```sh
    npx npm-check-updates -u
    ```

## Remove untracked files

Linux is able to handle globbing with large numbers of files, but MacOS might fail. This will work on both platforms.

```sh
(find . -type f; find . -type d) | xargs git check-ignore | xargs rm -rf
```

Current mindmap idea:
```
@startmindmap
* root
** api/v1
*** disabled handler
*** subject
**** title
***** get
****** read handler
***** put
****** authorization
******* unauthorized handler
******* update handler
***** post
****** authorization
******* unauthorized handler
******* create handler
***** bad method handler
*** subjects
**** get
***** content-type
****** list handler
*** not found handler
** auth/v1
*** not found handler
** ui
*** get
**** resource handler
** not found handler
@endmindmap
```