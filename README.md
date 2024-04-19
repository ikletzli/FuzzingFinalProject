### Build Docker image

docker build -t "image_name" .

### Get into the container

docker run -it --name minecraft-fuzz image_name bash

### Command for testing rust

afl-fuzz -t 20000 -g 8 -G 8 -Q -i in -o out ../target/debug/theseus_gui @@

### Command for testing frontend

afl-fuzz -t 20000 -Q -i in -o out ./fuzzable @@

### Frontend harness

test.js contains the frontend harness

### Backend harness

theseus_main/theseus_gui/src-tauri/src/main.rs contains the start to the backend harness
