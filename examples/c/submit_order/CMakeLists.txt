include_directories(../../../c/csrc/include)

add_executable(submit_order_c main.c)
target_link_libraries(submit_order_c longport_c)
if(NOT CMAKE_HOST_WIN32)
    target_link_libraries(submit_order_c ncurses)
endif()
