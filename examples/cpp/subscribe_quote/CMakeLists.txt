include_directories(../../../cpp/include)

add_executable(subscribe_quote_cpp main.cpp)
target_link_libraries(subscribe_quote_cpp longbridge_cpp)
