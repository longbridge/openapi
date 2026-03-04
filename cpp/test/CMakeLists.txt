include_directories(../include)

add_executable(test-cpp main.cpp)
target_link_libraries(test-cpp longbridge_cpp)