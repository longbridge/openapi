include_directories(../../csrc/include)

add_executable(account_asset main.c)
target_link_libraries(account_asset longbridge-c)
if(NOT CMAKE_HOST_WIN32)
    target_link_libraries(account_asset ncurses)
endif()
