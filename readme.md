<div align="center">
    <img src="./assets/octopus.svg" width="180" />
    <h1>PatchUp</h1>
    <p>A service for creating and distributing binary patches.</p>
</div>

## About
An open source service to reduce the overall installation time of large software updates. This is achieved through the efficient computation of binary patches between software versions and fast, low-latency delivery of patches using a content delivery network. 

## System Architecture
#### Client
React single page app using Redux for state management. Deployed on AWS S3 for efficient distribution.
#### Server
Express server using Mongoose as the database query engine. Deployed on AWS EC2.  
#### Database
MongoDB replica of 3 nodes. Deployed on AWS EC2.
#### Service
Patching service in Rust using the bidiff crate. Deployed on AWS Fargate with automatic horizontal scaling.

## Development
To start a local development server, client and database with hot-reloading,
```
docker-compose up -d --build
```
To stop the local development server (required if installing modules),
```
docker-compose down -v
```

## Contributing
Pull requests are welcome although it is recommended to first open an issue to discuss potential changes.

