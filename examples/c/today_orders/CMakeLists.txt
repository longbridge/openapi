include_directories(../../../c/csrc/include)

add_executable(today_orders_c main.c)
target_link_libraries(today_orders_c longport_c)
if(NOT CMAKE_HOST_WIN32)
    target_link_libraries(today_orders_c ncurses)
endif()
