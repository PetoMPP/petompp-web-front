FROM rust:latest
ENV NODE_VERSION=18.17.1
RUN apt install -y curl
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
ENV NVM_DIR=/root/.nvm
RUN . "$NVM_DIR/nvm.sh" && nvm install ${NODE_VERSION}
RUN . "$NVM_DIR/nvm.sh" && nvm use v${NODE_VERSION}
RUN . "$NVM_DIR/nvm.sh" && nvm alias default v${NODE_VERSION}
ENV PATH="/root/.nvm/versions/node/v${NODE_VERSION}/bin/:${PATH}"
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk --locked
VOLUME /front
WORKDIR /front
COPY . .
RUN npm install -D
EXPOSE 8080
CMD trunk serve --no-autoreload --release --address 0.0.0.0