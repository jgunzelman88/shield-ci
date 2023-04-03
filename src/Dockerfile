# ---- Dev Image ---- 
FROM redhat/ubi8:latest
VOLUME [ "/home/shieldci" ] build

ARG JAVA_VERSION=java-11-openjdk
ARG DEP_CHECK_VERSION=8.2.1
ARG DEP_CHECK_URL=https://github.com/jeremylong/DependencyCheck/releases/download/v${DEP_CHECK_VERSION}/dependency-check-${DEP_CHECK_VERSION}-release.zip
#Install Java

#Set up user
RUN useradd -ms /bin/bash shieldci
WORKDIR /home/shieldci
# Install Rust
RUN curl https://sh.rustup.rs --output sh.rustup.rs &&\
    sh ./sh.rustup.rs -y &&\
    # Install Java & OWASP Dependecy check
    yum -y install ${JAVA_VERSION} &&\
    yum -y install zip &&\
    yum -y install unzip
RUN curl ${DEP_CHECK_URL} --output dependency-check.zip &&\
    zip -FF dependency-check.zip --out dependency-check-fixed.zip | upzip
ENTRYPOINT ["tail", "-f", "/dev/null"]