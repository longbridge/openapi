include_directories(../c/csrc/include include)
set(SOURCES
    src/config.cpp
    src/decimal.cpp
    src/status.cpp
    src/types.cpp
    src/quote_context.cpp
    src/trade_context.cpp
)
add_library(
    longbridge_cpp SHARED
    ${SOURCES}
)
target_link_libraries(longbridge_cpp longbridge-c)

add_subdirectory(test)
