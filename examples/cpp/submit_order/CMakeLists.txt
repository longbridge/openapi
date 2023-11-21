include_directories(../../../cpp/include)

add_executable(submit_order_cpp main.cpp)
target_link_libraries(submit_order_cpp longport_cpp)
