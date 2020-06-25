#!/bin/env bash
set -ex
JVM=/usr/lib/jvm/java-14-openjdk-amd64

cbindgen --clean --cpp-compat --lang C++ --output include/morpheus.hpp

cd java

swig -java -c++ -outcurrentdir -package global.iop.morpheus ../include/morpheus.i
gcc -fPIC -c -I../include morpheus_wrap.cxx -I"$JVM/include/"  -I"$JVM/include/linux/"
