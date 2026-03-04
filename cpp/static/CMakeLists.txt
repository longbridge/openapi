cmake_minimum_required(VERSION 3.11)
cmake_policy(SET CMP0048 NEW)
cmake_policy(SET CMP0079 NEW)
project(longbridge)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_WINDOWS_EXPORT_ALL_SYMBOLS ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

include_directories(../../c/csrc/include ../include)
set(SOURCES
    ../src/http_client.cpp
    ../src/config.cpp
    ../src/decimal.cpp
    ../src/status.cpp
    ../src/types.cpp
    ../src/quote_context.cpp
    ../src/trade_context.cpp
    ../src/oauth.cpp
)
add_library(
    longbridge_cpp STATIC
    ${SOURCES}
)
