FROM node:16

WORKDIR /usr/src

RUN git clone https://github.com/modrinth/theseus.git && \
    mv theseus/theseus_gui . && rm -rf theseus && cd theseus_gui

WORKDIR /usr/src/theseus_gui

RUN npm install && \
    npm install -g jest && \
    npm install -g @vue/cli && \
    npm audit fix --force && \
    npm install @vue/cli-plugin-unit-jest @vue/test-utils

RUN mkdir __tests__ && \
    sed -i '10i\"test\": \"node --experimental-vm-modules node_modules/jest/bin/jest.js\",' package.json

RUN npm install @vue/vue3-jest@27

RUN npm install --save-dev @babel/preset-env

RUN npm install --save-dev @vue/cli-plugin-babel

RUN npm install --save-dev babel-core@bridge

COPY package.json ./package.json

COPY app.spec.js __tests__/app.spec.js

COPY babel.config.json ./babel.config.json

COPY theseus_gui/src/helpers/profile.js ./src/helpers/profile.mjs

COPY theseus_gui/src/pages/instance/Mods.vue ./src/pages/instance/Mods.vue

COPY theseus_gui/src/components/ui/ExportModal.vue ./src/components/ui/ExportModal.vue