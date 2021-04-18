<div align="center">
    <img src="./assets/octopus.svg" width="180" />
    <h1>PatchUp</h1>
    <p>A service for creating and distributing binary patches.</p>
    <a href="https://github.com/Dezyh/patchup/actions/workflows/cli.yml">
        <img alt="Patchup CLI" src="https://github.com/Dezyh/patchup/actions/workflows/cli.yml/badge.svg" />
    </a>
</div>

## About
An open source service to reduce the overall installation time of large software updates. This is achieved through the efficient computation of binary patches between software versions and fast, low-latency delivery of patches using a content delivery network. 

### [Client - CLI](https://github.com/dezyh/patchup/tree/master/cli)
- A cross-platform command line tool to generate and apply binary patches across files and directories.
- Written in Rust

### [Client - Web](https://github.com/dezyh/patchup/tree/master/client)
- A web client to interact with the Patchup Cloud web service.
- Written in React/TypeScript

### [Server](https://github.com/dezyh/patchup/tree/master/server)
- The Patchup Cloud web server which handles authentication.
- Written in Rust with a PostgreSQL database

## Development
#### 1. Development Server
To start up a local development server
```
docker-compose up -d
```
This will create instances of the client, server, database and aws resources, each inside their own docker container. The client and server will reload on any changes. The database data will persist inside the `.database` directory and the AWS resources will persist inside the `.aws` directory.

#### 3. Stop Development Server
To stop the local development server and cleanly shut down the containers
```
docker-compose down
```
## Contributing
Pull requests are welcome although it is recommended to first open an issue to discuss potential changes.
