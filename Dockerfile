# ---- Build ----
FROM rust:1.68.2-buster AS builder
COPY . /shieldci
WORKDIR /shieldci
RUN cargo build

ENV PROJECT_ID = ""
ENV SHIELD_URL = ""
ENV SHIELD_USER = ""
ENV SHIELD_PASS = ""

# ---- Run ---- 
FROM redhat/ubi8:latest AS runtime
VOLUME [ "/home/shieldci/scan" ] scan
ENV SHIELD_CI_SCAN_DIR="/home/shieldci/scan"

# Install trivy
RUN rpm -ivh https://github.com/aquasecurity/trivy/releases/download/v0.42.1/trivy_0.42.1_Linux-64bit.rpm

# Copy EXE
COPY --from=builder /shieldci/target/debug/shieldci /home/shieldci/

ENTRYPOINT ["/home/shieldci/shieldci"]