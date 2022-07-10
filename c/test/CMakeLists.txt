include_directories(../csrc/include)

add_executable(test-c main.c)
target_link_libraries(test-c longbridge-c)
if(NOT CMAKE_HOST_WIN32)
    target_link_libraries(test-c ncurses)
endif()
