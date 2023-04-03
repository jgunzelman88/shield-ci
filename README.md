# Shield-CI

CI tool for tracking dependencies, vulnerabilities, and project meta data.

## Development

We have provided a development Dockerfile for development hear are instuctions to start it. Or use the docker desktop or the docker plugin on VScode.

   1. Build dev container
      * ```docker build -t shieldci-dev:latest ./src```
   2. Run Container
      * ```docker run --mount type=bind,source="$(pwd)"/,target=/home/shieldci/source shieldci-dev:latest```
