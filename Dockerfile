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

RUN bash -lc "\
    mkdir -p ~/.local/share/bash-completion/completions && \
    rustup completions bash > ~/.local/share/bash-completion/completions/rustup && \
    mkdir -p ~/.zfunc && \
    rustup completions zsh > ~/.zfunc/_rustup && \
    mkdir -p ~/.config/fish/completions && \
    rustup completions fish > ~/.config/fish/completions/rustup.fish \
    "

ADD ./mirror.config /root/.cargo/config

RUN apt-fast install -y libssh-dev pkg-config cmake
RUN bash -lc "\
    rustup toolchain install nightly && \
    rustup run nightly rustc --version && \
    rustup default nightly \
    "

RUN bash -lc "cargo install --force \
    cargo-add \
    cargo-edit cargo-edit-locally \
    cargo-make \
    cargo-lock \
    cargo-tree \
    "

RUN bash -lc "cargo install cargo-script runner"
RUN bash -lc "cargo install --git https://github.com/faern/rustscript"
