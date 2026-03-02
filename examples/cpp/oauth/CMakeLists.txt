include_directories(../../../cpp/include)

add_executable(oauth_cpp main.cpp)
target_link_libraries(oauth_cpp longport_cpp)
