include_directories(../../../c/csrc/include)

add_executable(http_client_c main.c)
target_link_libraries(http_client_c longport_c)
if(NOT CMAKE_HOST_WIN32)
    target_link_libraries(http_client_c ncurses)
endif()
