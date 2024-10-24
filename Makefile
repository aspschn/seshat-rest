CXX = clang++
CXXFLAGS = -std=c++11 -Ilibs -I../../Seshat/include -I/home/ubuntu/repo/Seshat/include

default:
	cd libs; ./download.sh
	$(MAKE) libs/libyuarel
	# $(CXX) properties.cpp $(CXXFLAGS) -lseshat libs/libyuarel/libyuarel.a -o cgi-bin/properties
	$(CXX) src/unicode.cpp src/main.cpp $(CXXFLAGS) -lseshat libs/libyuarel/libyuarel.a -o cgi-bin/seshat-rest -lfcgi -lfcgi++ -luriparser

run:
	nohup spawn-fcgi -n -f /home/ubuntu/app/seshat-demo/api/cgi-bin/seshat-rest -a 0.0.0.0 -p 8000 &

run-rocket:
	cargo build --release
	ROCKET_PORT=8001 ROCKET_LOG_LEVEL=critical nohup target/release/seshat-rest &

build-musl:
	cargo build --release --target x86_64-unknown-linux-musl

