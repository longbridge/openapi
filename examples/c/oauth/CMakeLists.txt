include_directories(../../../c/csrc/include)

add_executable(oauth_c main.c)
target_link_libraries(oauth_c longport_c)
if(NOT CMAKE_HOST_WIN32)
    target_link_libraries(oauth_c ncurses)
endif()
