docker build -t "image_name" .
docker run -d --name mygtkapp -e DISPLAY=172.30.96.1:0 image_name 