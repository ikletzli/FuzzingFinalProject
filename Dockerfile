FROM node:16 AS builder

WORKDIR /usr/src

RUN apt update && apt -y install libsoup2.4-dev && apt -y install libpango1.0-dev && \
apt -y install libatk1.0-dev && apt -y install javascriptcoregtk-4.0 && apt -y install gdk-3.0

RUN apt update && apt -y install librust-gdk-dev && apt -y install libwebkit2gtk-4.0-dev && apt -y install xvfb 

# get afl++
RUN apt-get update && \
    apt-get install -y build-essential python3-dev automake cmake git flex bison libglib2.0-dev libpixman-1-dev python3-setuptools cargo libgtk-3-dev && \
    apt-get install -y lld-14 llvm-14 llvm-14-dev clang-14 || apt-get install -y lld llvm llvm-dev clang && \
    apt-get install -y gcc-$(gcc --version|head -n1|sed 's/\..*//'|sed 's/.* //')-plugin-dev libstdc++-$(gcc --version|head -n1|sed 's/\..*//'|sed 's/.* //')-dev && \
    apt-get install -y ninja-build

RUN git clone https://github.com/AFLplusplus/AFLplusplus && \
    cd AFLplusplus && make distrib && make install
    
RUN mkdir theseus

COPY theseus_main ./theseus

WORKDIR /usr/src/theseus/theseus_gui

RUN curl --proto -y '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    . "$HOME/.cargo/env" && cargo build --bin theseus_gui

COPY test.jar test.jar

COPY worldmap.jar worldmap.jar

COPY crosshair.jar crosshair.jar

COPY big_profile.json /root/.config/com.modrinth.theseus/profiles/test/profile.json

RUN mkdir /root/.config/com.modrinth.theseus/profiles/test/mods

RUN mkdir in && mkdir out && cd in && touch test.jar && truncate -s +8 test.jar && cd ..

CMD ["../target/debug/theseus_gui", "test.jar"]