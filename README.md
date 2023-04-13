# Shield-CI

CI tool for tracking dependencies, vulnerabilities, and project meta data.

## Development

We have provided a development Dockerfile for development hear are instuctions to start it. Or use the docker desktop or the docker plugin on VScode.

   1. Build dev container

      * ```docker build -t shieldci-dev:latest ./src```

   2. Run Container
      (Note: Target folder is a seperate volume to speed up build times due to slow IO performance between VM and Host on MacOs)

      * ```sh
         docker run --name shieldci-dev --mount type=bind,source="$(pwd)"/,target=/home/shieldci/ --mount type=volume,dst=/home/shieldci/target --mount type=volume,dst=/home/shieldci/dependency-check shieldci-dev:latest
         ```

## Usage

```sh
Usage: shieldci [OPTIONS]
Options:
      --path <PATH>                [default: ./]
  -v, --verbose
      --shield-url <SHIELD_URL>    [default: ]
      --shield-user <SHIELD_USER>  [default: ]
      --shield-pass <SHIELD_PASS>  [default: ]
  -h, --help                       Print help
  -V, --version                    Print version
```
