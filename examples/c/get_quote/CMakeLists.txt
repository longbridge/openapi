include_directories(../../../c/csrc/include)

add_executable(get_quote_c main.c)
target_link_libraries(get_quote_c longbridge_c)
if(NOT CMAKE_HOST_WIN32)
    target_link_libraries(get_quote_c ncurses)
endif()
