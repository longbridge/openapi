include_directories(../../../cpp/include)

add_executable(history_candlesticks_by_offset main.cpp)
target_link_libraries(history_candlesticks_by_offset longbridge_cpp)
