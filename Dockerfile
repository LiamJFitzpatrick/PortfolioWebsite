FROM ubuntu AS buildstage
WORKDIR /app
COPY . .
RUN apt-get update && apt-get install -y \
    build-essential \
    curl
# Get Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Get cargo-leptos
RUN cargo install cargo-leptos

# Get Node and npm
# nvm env vars
RUN mkdir -p /usr/local/nvm
ENV NVM_DIR=/usr/local/nvm
# IMPORTANT: set the exact version
ENV NODE_VERSION=v22.9.0
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.1/install.sh | bash
RUN /bin/bash -c "source $NVM_DIR/nvm.sh && nvm install $NODE_VERSION && nvm use --delete-prefix $NODE_VERSION"
# add node and npm to the PATH
ENV NODE_PATH=$NVM_DIR/versions/node/$NODE_VERSION/bin
ENV PATH=$NODE_PATH:$PATH

# Add wasm32 target to rust
RUN rustup target add wasm32-unknown-unknown

# Build website
WORKDIR /app/website
RUN npm install .
RUN cargo leptos build --release

FROM ubuntu
WORKDIR /app
COPY --from=buildstage /app/website/target/release/website ./
COPY --from=buildstage /app/website/target/site ./site/

# Set any required env variables and
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"

CMD ./website