include_directories(../../../c/csrc/include)

add_executable(account_asset_c main.c)
target_link_libraries(account_asset_c longbridge-c)
if(NOT CMAKE_HOST_WIN32)
    target_link_libraries(account_asset_c ncurses)
endif()
