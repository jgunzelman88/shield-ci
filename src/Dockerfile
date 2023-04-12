# ---- Dev Image ---- 
FROM redhat/ubi8:latest
VOLUME [ "/home/shieldci/src" ] src
VOLUME [ "/home/shieldci/src" ] src

ARG JAVA_VERSION=java-11-openjdk
ARG DEP_CHECK_VERSION=8.2.1
ARG DEP_CHECK_URL=https://github.com/jeremylong/DependencyCheck/releases/download/v${DEP_CHECK_VERSION}/dependency-check-${DEP_CHECK_VERSION}-release.zip

#Set up user
RUN useradd -ms /bin/bash shieldci
WORKDIR /home/shieldci
    # Install Rust
RUN curl https://sh.rustup.rs --output sh.rustup.rs &&\
    sh ./sh.rustup.rs -y &&\
    rm sh.rustup.rs &&\
    yum -y install gcc &&\
    yum -y install openssl-devel &&\
    # Install Java & OWASP Dependecy check
    yum -y install ${JAVA_VERSION} &&\
    yum -y install zip &&\
    yum -y install unzip &&\
    curl -L ${DEP_CHECK_URL} --output dependency-check.zip &&\
    unzip dependency-check.zip &&\
    rm dependency-check.zip
ENTRYPOINT ["tail", "-f", "/dev/null"]