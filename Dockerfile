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
FROM alpine:3.18 AS runtime
VOLUME [ "/home/shieldci/scan" ] scan
ENV SHIELD_CI_SCAN_DIR="/home/shieldci/scan"

# Install trivy
RUN apk --no-cache add ca-certificates git docker && \
    curl -sfL https://raw.githubusercontent.com/aquasecurity/trivy/main/contrib/install.sh | sh -s -- -b /usr/local/bin v0.18.3
# Copy EXE
COPY --from=builder /shieldci/target/debug/shieldci /home/shieldci/

ENTRYPOINT ["/home/shieldci/shieldci"]