# ---- Dev Image ---- 
FROM redhat/ubi9:latest
VOLUME [ "/home/shieldci/" ] home
VOLUME [ "/home/shieldci/target" ] target

WORKDIR /home/shieldci
    #Set up user
RUN useradd -ms /bin/bash shieldci &&\
    # Install Rust
    curl https://sh.rustup.rs --output sh.rustup.rs &&\
    sh ./sh.rustup.rs -y &&\
    rm sh.rustup.rs &&\
    yum -y install gcc &&\
    yum -y install openssl-devel &&\
    yum clean all &&\
    # Install trivy
    rpm -ivh https://github.com/aquasecurity/trivy/releases/download/v0.39.1/trivy_0.39.1_Linux-64bit.rpm
ENTRYPOINT ["tail", "-f", "/dev/null"]