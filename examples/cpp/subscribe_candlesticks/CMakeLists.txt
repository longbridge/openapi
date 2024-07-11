include_directories(../../../cpp/include)

add_executable(subscribe_candlesticks_cpp main.cpp)
target_link_libraries(subscribe_candlesticks_cpp longport_cpp)
