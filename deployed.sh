cd build
cmake ..
make
if [ ! -d ./dist ];then
    mkdir dist
fi
cp client/libclient.so ./dist
cp gui/libreqwest_cpp.so ./dist
cp ../gui/wrappers.hpp ./dist
cp ../gui/ffi.hpp ./dist