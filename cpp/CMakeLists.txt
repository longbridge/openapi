include_directories(../c/csrc/include include)
set(SOURCES
    src/http_client.cpp
    src/config.cpp
    src/decimal.cpp
    src/status.cpp
    src/types.cpp
    src/quote_context.cpp
    src/trade_context.cpp
)
add_library(
    longport_cpp SHARED
    ${SOURCES}
)
if(NOT CMAKE_HOST_LINUX)
target_link_libraries(longport_cpp longport_c)
endif()

add_subdirectory(test)
