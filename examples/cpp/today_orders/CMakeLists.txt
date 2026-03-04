include_directories(../../../cpp/include)

add_executable(today_orders_cpp main.cpp)
target_link_libraries(today_orders_cpp longbridge_cpp)
