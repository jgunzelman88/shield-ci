# ---- Build ----
FROM rust:1.68.2-buster AS builder
COPY . /shieldci
WORKDIR /shieldci
RUN cargo build

# ---- Run ---- 
FROM redhat/ubi8:latest AS runtime
VOLUME [ "/home/shieldci/scan" ] scan
ENV SCAN_DIR="/home/shieldci/scan"
ENV SHIELD_URL=""

# Install trivy
RUN rpm -ivh https://github.com/aquasecurity/trivy/releases/download/v0.39.1/trivy_0.39.1_Linux-64bit.rpm

# Copy EXE
COPY --from=builder /shieldci/target/debug/shieldci /home/shieldci/
ENTRYPOINT [ "shieldci", "${SCAN_DIR}" ]