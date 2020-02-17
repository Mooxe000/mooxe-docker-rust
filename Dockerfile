FROM mooxe/base:dev

MAINTAINER FooTearth "footearth@gmail.com"

WORKDIR /root

# update
RUN apt-fast update && \
    apt-fast upgrade -y && \
    apt-fast autoremove -y

RUN apt-fast install -y gcc

RUN curl https://sh.rustup.rs -SkLf | bash -s -- -y
RUN \
    echo "source $HOME/.cargo/env" >> ~/.bashrc && \
    echo "source $HOME/.cargo/env" >> ~/.zshrc && \
    echo "source $HOME/.cargo/env" >> ~/.config/fish/config.fish

ADD ./mirror.config /root/.cargo/config

RUN apt-fast install -y libssh-dev pkg-config cmake

RUN bash -lc "cargo install cargo-script runner"
RUN bash -lc "cargo install --git https://github.com/faern/rustscript"

RUN bash -lc "cargo install --force \
  cargo-edit cargo-edit-locally \
  cargo-make \
  cargo-lock \
  "
