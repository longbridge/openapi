include_directories(../../../cpp/include)

add_executable(submit_order_cpp main.cpp)
target_link_libraries(submit_order_cpp longbridge_cpp)
if(NOT CMAKE_HOST_WIN32)
    target_link_libraries(submit_order_cpp ncurses)
endif()
