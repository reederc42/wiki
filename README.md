# Wiki (2.0)

This is wiki 2.0: wikis serve an interconnected set of pages, maintained and
edited by users. Wiki 2.0 is intended to be an example of scalable TDD, using Go
and Javascript to create a well-tested, secure, client/server application.

This monorepo includes all design notes and a set of build and test tools that
are developed similarly to the main application, using TDD.

# Major Technologies

* Go
    * Go is used to develop the server and tools
* Javascript
    * Javascript is used to develop the user interface, the client
        * Solid.js is used as a framework for developing reactive components
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
