include_directories(../c/csrc/include include)
set(SOURCES
    src/http_client.cpp
    src/config.cpp
    src/content_context.cpp
    src/decimal.cpp
    src/alert_context.cpp
    src/dca_context.cpp
    src/sharelist_context.cpp
    src/calendar_context.cpp
    src/fundamental_context.cpp
    src/market_context.cpp
    src/portfolio_context.cpp
    src/status.cpp
    src/types.cpp
    src/quote_context.cpp
    src/trade_context.cpp
    src/oauth.cpp
)
add_library(
    longbridge_cpp SHARED
    ${SOURCES}
)
target_link_libraries(longbridge_cpp longbridge_c)

add_subdirectory(test)
