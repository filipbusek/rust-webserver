# What is this?
Webserver from 20th chapter of the book `The Rust Programming Language` modified to return random gif from directory </br>
https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html

# Available arguments
``` bash
--root, -r: Specifies webroot in which will webserver be looking for gifs
--port, -p: Specifies port for incoming connections
--ip: Specifies on which ip should webserver listen for connections
--worker, -w: Specifies number of available workers. Default is 4. Max value is 256
--help, -h: Shows this message
```

# Run from docker.io
``` bash
docker run --detach --name rust-webserver -p 8080:8080 --volume /path-to-gif-folder docker.io/rust5528/rust-webserver:0.1.3
```

# Build from source
``` bash
cargo build --release
./target/release/rust-webserver
```

# Build OCI image
``` bash
cp -a ./target/release/rust-webserver
docker build . -t $my_name/$container_name:$tag
```
