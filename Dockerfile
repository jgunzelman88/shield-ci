# ---- Build ----
FROM rust:1.68.2-buster AS builder
ADD . /shieldci
WORKDIR /shieldci
RUN cargo build

# ---- Run ---- 
FROM redhat/ubi8:latest
VOLUME [ "/home/shieldci/scan" ] scan

ARG JAVA_VERSION=java-11-openjdk
ARG DEP_CHECK_VERSION=8.2.1
ARG DEP_CHECK_URL=https://github.com/jeremylong/DependencyCheck/releases/download/v${DEP_CHECK_VERSION}/dependency-check-${DEP_CHECK_VERSION}-release.zip

# Install OWASP Dependecy check
RUN yum -y install ${JAVA_VERSION}
RUN useradd -ms /bin/bash shieldci
WORKDIR /home/shieldci
RUN curl ${DEP_CHECK_URL}
# Copy EXE
COPY --from=builder /shieldci/target/debug/shieldci /home/shieldci/
ENTRYPOINT [ "shieldci", "" ]