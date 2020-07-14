<div align="center">
    <img src="./assets/octopus.svg" width="180" />
    <h1>PatchUp</h1>
    <p>A service for creating and distributing binary patches.</p>
</div>

## About
An open source service to reduce the overall installation time of large software updates. This is achieved through the efficient computation of binary patches between software versions and fast, low-latency delivery of patches using a content delivery network. 

## System Architecture
- JavaScript + [React](https://reactjs.org) frontend client
- Rust + [Rocket](https://rocket.rs) api server
- Rust + [bidiff](https://github.com/divvun/bidiff) patch microservice
- [PostgreSQL](https://www.postgresql.org) + [Diesel](https://github.com/diesel-rs/diesel) database

## Development
To start a development server with hot-reloading
```
docker-compose up -d
```

## Contributing
Pull requests are welcome although it is recommended to first open an issue to discuss potential changes.

