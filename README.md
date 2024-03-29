# <img style="height:2rem" src="data:image/svg+xml;base64,PHN2ZyB2aWV3Qm94PSIwIDAgMjQgMjQiIHN0eWxlPSJmaWxsOiB3aGl0ZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KCQkJCTxwYXRoCgkJCQkJZmlsbC1ydWxlPSJldmVub2RkIgoJCQkJCWNsaXAtcnVsZT0iZXZlbm9kZCIKCQkJCQlkPSJNMTIsMUwzLDVWMTFDMywxNi41NSA2Ljg0LDIxLjc0IDEyLDIzQzE3LjE2LDIxLjc0IDIxLDE2LjU1IDIxLDExVjVMMTIsMU0xMiw3QzEzLjQsNyAxNC44LDguMSAxNC44LDkuNVYxMUMxNS40LDExIDE2LDExLjYgMTYsMTIuM1YxNS44QzE2LDE2LjQgMTUuNCwxNyAxNC43LDE3SDkuMkM4LjYsMTcgOCwxNi40IDgsMTUuN1YxMi4yQzgsMTEuNiA4LjYsMTEgOS4yLDExVjkuNUM5LjIsOC4xIDEwLjYsNyAxMiw3TTEyLDguMkMxMS4yLDguMiAxMC41LDguNyAxMC41LDkuNVYxMUgxMy41VjkuNUMxMy41LDguNyAxMi44LDguMiAxMiw4LjJaIgoJCQkJLz4KCQkJPC9zdmc+" alt=""> Shield-CI

CI tool for tracking dependencies, vulnerabilities, and project meta data.

---

## Development

---

### Develop in a container

We have provided a development Dockerfile for development hear are instuctions to start it. Or use the docker desktop or the docker plugin on VScode.

   1. Build dev container

      ```docker build -t shieldci-dev:latest ./src```

   2. Run Container
      (Note: Target folder is a seperate volume to speed up build times due to slow IO performance between VM and Host on MacOs)

      ```sh
      docker run \
      --name shieldci-dev \
      --mount type=bind,source="$(pwd)"/,target=/home/shieldci/ \
      --mount type=volume,dst=/home/shieldci/target \
      shieldci-dev:latest
      ```

### Standalone

   1. Install trivy [https://trivy.dev/](https://trivy.dev/).
   2. Install rustup. [https://rustup.rs/](https://rustup.rs/).
   3. ``` cargo build ```
   4. ``` ./target/build/shieldci -h ```

---

## Install/Execution

---
There are 2 options for running shield-ci.  We have a docker container for use in ci tools and conntains all dependecies required for executing.  Or you can install on your machine.

### Run as container

   1. Build container
 
      ```sh
      docker build -t shield-ci:latest .
      ```

   2. Run container

      ```sh
      docker run \
      --env SHIELD_VERBOSE=true \
      --env PROJECT_ID=<PROJECT_ID> \
      --env SHIELD_URL=<SHIELD_URL> \
      --env SHIELD_USER=<SHIELD_USER> \
      --env SHIELD_PASS=<SHIELD_PASS> \
      --env IMAGE_PATH=<IMAGE_PATH> \
      --env IMAGE_TAG=<IMAGE_TAG> \
      --env TRIVY_TIMEOUT_SEC=600s \
      --mount type=bind,source=$(pwd),target=/home/shieldci/scan \
      shield-ci:latest
      ```
### CLI Usage

```sh
Usage: shieldci [OPTIONS]

Options:
      --project-id <PROJECT_ID>    [default: ]
      --path <PATH>                [default: ./]
  -v, --verbose                    
      --shield-url <SHIELD_URL>    [default: ]
      --shield-user <SHIELD_USER>  [default: ]
      --shield-pass <SHIELD_PASS>  [default: ]
  -h, --help                       Print help
  -V, --version                    Print version
```

---
- Run CLI
   ```sh

   ```

#### Install

This component requires Trivy security scanner. [https://trivy.dev/](https://trivy.dev/) Install before using


## Configuration

---

Configuration of shield-ci can be accomplished via CLI arguments or environment variable.  CLI arguments will override env variables if they are provided.

### Parameters

| Parameter | ENV Variable | CLI Argument | Description |
| --- | --- | --- | --- |
| Path | SHIELD_CI_SCAN_DIR | --path \<PATH> | Scan path for application to process.
| Project Id | PROJECT_ID| --project-id \<PROJECT_ID>| Project Id for application
| Shield URL | SHIELD_URL | --shield-url \<SHIELD_URL> | URL to Shield web application.
| Shield User | SHIELD_USER | --shield-user \<SHIELD_USER>  | Shield user name to access Shield web API
| Shield URL | SHIELD_PASS | --shield-pass \<SHIELD_PASS> | Shield password to access Shield web API
