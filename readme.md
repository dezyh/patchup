<div align="center">
    <img src="./assets/octopus.svg" width="250" />
    <h1>PatchUp</h1>
    <p>A service for creating and distributing binary patches.</p>
</div>

## About
An open source service for creating and distributing binary patches between software versions. Minimal binary patches are efficiently computed in using [bidiff](https://github.com/divvun/bidiff) and delivered using [Amazon CloudFront](https://aws.amazon.com/cloudfront) to provide low latency and high transfer speeds.

## Development
To start a development server with hot-reloading
```
docker-compose up -d
```

## Contributing
Pull requests are welcome although it is recommended to first open an issue to discuss potential changes.

