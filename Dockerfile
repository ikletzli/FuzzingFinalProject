FROM node:16

WORKDIR /usr/src

RUN git clone https://github.com/modrinth/theseus.git

WORKDIR /usr/src/theseus/theseus_gui

RUN npm install && \
    npm install -g jest && \
    npm install -g @vue/cli && \
    npm audit fix --force && \
    npm install @vue/cli-plugin-unit-jest @vue/test-utils

RUN mkdir __tests__

COPY package.json ./package.json

COPY search.spec.js __tests__/search.spec.js

COPY browse_test.js ./src/browse_test.js


COPY theseus/theseus_gui/src-tauri/Cargo.toml ./src-tauri/Cargo.toml

COPY theseus/theseus_gui/src-tauri/src/api/profile.rs ./src-tauri/src/api/profile.rs

RUN apt update && apt -y install libsoup2.4-dev && apt -y install libpango1.0-dev && \
apt -y install libatk1.0-dev && apt -y install javascriptcoregtk-4.0 && apt -y install gdk-3.0

RUN apt -y install librust-gdk-dev && apt -y install libwebkit2gtk-4.0-dev

RUN npm run build

COPY theseus/theseus/src/util/io.rs ../theseus/src/util/io.rs

RUN curl --proto -y '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    . "$HOME/.cargo/env" && cargo build --bin theseus_gui 

#CMD ["cargo run --bin theseus_gui "]
#npm run build

#RUN npm test