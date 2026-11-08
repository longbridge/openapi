include_directories(../../../c/csrc/include ../../../cpp/include)

add_executable(submit_order_with_attached_cpp main.cpp)
target_link_libraries(submit_order_with_attached_cpp longbridge_cpp longbridge_c)
