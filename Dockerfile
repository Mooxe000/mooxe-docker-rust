FROM mooxe/base:dev

MAINTAINER FooTearth "footearth@gmail.com"

WORKDIR /root

# update
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get autoremove -y

RUN apt-get install -y gcc

RUN curl https://sh.rustup.rs -SkLf | bash -s -- -y
RUN \
    echo "source $HOME/.cargo/env" >> ~/.bashrc && \
    echo "source $HOME/.cargo/env" >> ~/.zshrc && \
    echo "source $HOME/.cargo/env" >> ~/.config/fish/config.fish

ADD ./mirror.config /root/.cargo/config

RUN apt-get install -y libssh-dev pkg-config cmake
RUN bash -lc "cargo install --force cargo-edit cargo-edit-locally"
