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
Actix-Web server with Diesel as the ORM for a PostgreSQL database. Deployed on AWS EC2.  
#### Database
PostgreSQL deployed on AWS EC2.
#### Service
Patching service in Rust using the bidiff crate. Deployed on AWS Fargate with automatic horizontal scaling.

## Development
#### 1. Start Development Server
To start up a local development server
```
docker-compose up -d
```
This will create instances of the client, server, database and aws resources, each inside their own docker container. The client and server will reload on any changes. The database data will persist inside the `.database` directory and the AWS resources will persist inside the `.aws` directory.

#### 2. Provision Local AWS Services
To provision the required AWS services (S3)
```
cd terraform/development
terraform init
terraform apply -auto-approve
```
This will create a local S3 bucket that the local development server can access for local testing. The aws services data will persist inside the `.aws` directory and thus only needs to be provisioned once.

#### 3. Stop Development Server
To stop the local development server and cleanly shut down the containers
```
docker-compose down
```
## Contributing
Pull requests are welcome although it is recommended to first open an issue to discuss potential changes.
