docker build -t "image_name" .
docker run -d --name mygtkapp -e DISPLAY=172.30.96.1:0 image_name (PC)
docker run -d --name mygtkapp -e DISPLAY=192.168.56.1:0 image_name (Laptop)

Xvfb :1 -screen 0 1024x768x16 (to start virtual monitor)
DISPLAY=:1.0 ../target/debug/theseus_gui (to run gui on virtual monitor)

afl-fuzz -t 20000 -g 8 -G 8 -Q -i in -o out ../target/debug/theseus_gui @@

afl-fuzz -Q -i in -o out ../target/debug/theseus_gui @@

docker run -it --name mygtkapp image_name bash 

echo "zwCoPr4q" >> new.jar && truncate -s 8 new.jar