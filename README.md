# RPi zero wireless test project
The RPi0 is running a [piCore Linux 13.0.3](http://forum.tinycorelinux.net/). The rust code is cross compiled by using a docker image from this [github repo](https://github.com/Ragnaroek/rust-on-raspberry-docker).
The following script mounts the project folder into the image and builds the application. Finally the application will be copied to the device.

```
#!/bin/sh
PROJ="hello_rpi0"
docker run --volume $PROJ:/home/cross/project --entrypoint /home/cross/bin/run.sh rust-raspberry-cross build --release
scp -r $PROJ/target/arm-unknown-linux-gnueabihf/release/$PROJ tc@<IP_ADDRESS>:
```
Login into the RPi0 and run the application which is placed inside home folder.
To activate logging export the two environment variables APP_LOG_STYLE and APP_LOG_LEVEL.
The application can be executed like: 
```
$ APP_LOG_STYLE=always APP_LOG_LEVEL=debug ./hello_rpi0
```
