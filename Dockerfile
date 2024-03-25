FROM node:16 AS builder

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

RUN apt update && apt -y install libsoup2.4-dev && apt -y install libpango1.0-dev && \
apt -y install libatk1.0-dev && apt -y install javascriptcoregtk-4.0 && apt -y install gdk-3.0 && \
apt -y install librust-gdk-dev && apt -y install libwebkit2gtk-4.0-dev

COPY App.vue ./src/App.vue

RUN npm run build

COPY io.rs ../theseus/src/util/io.rs

COPY mod.rs ../theseus/src/launcher/mod.rs

RUN curl --proto -y '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    . "$HOME/.cargo/env" && cargo build --bin theseus_gui 

#COPY test.jar /root/.config/com.modrinth.theseus/profiles/123/mods

COPY profile.json /root/.config/com.modrinth.theseus/profiles/123/profile.json

# RUN curl -O https://cdn.azul.com/zulu/bin/zulu17.48.15-ca-jre17.0.10-linux_x64.zip && unzip zulu17.48.15-ca-jre17.0.10-linux_x64.zip && \
#     curl -O https://cdn.azul.com/zulu/bin/zulu8.76.0.17-ca-jre8.0.402-linux_x64.zip && unzip zulu8.76.0.17-ca-jre8.0.402-linux_x64.zip

CMD ["../target/debug/theseus_gui"]

# #RUN npm test

#/root/.config/com.modrinth.theseus/profiles/123