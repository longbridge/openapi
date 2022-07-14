include_directories(../../../c/csrc/include)

add_executable(subscribe_quote_c main.c)
target_link_libraries(subscribe_quote_c longbridge-c)
if(NOT CMAKE_HOST_WIN32)
    target_link_libraries(subscribe_quote_c ncurses)
endif()
