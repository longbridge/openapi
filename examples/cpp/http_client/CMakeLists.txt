include_directories(../../../cpp/include)

add_executable(http_client_cpp main.cpp)
target_link_libraries(http_client_cpp longbridge_cpp)
