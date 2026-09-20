use std::sync::Arc;

use longbridge::{
    Config,
    grid::{
        GetGridOrderDetailOptions, GetGridOrdersOptions, GetGridTriggerHistoryOptions, GridContext,
        GridTradeRule, SubmitGridOrderOptions,
    },
    oauth::OAuthBuilder,
    trade::TradeContext,
};
use rust_decimal::Decimal;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let oauth = OAuthBuilder::new("your-client-id")
        .build(|url| println!("Open this URL to authorize: {url}"))
        .await?;
    let config = Arc::new(Config::from_oauth(oauth));
    // Grid REST calls go through the standalone GridContext.
    let ctx = GridContext::new(config.clone());
    // Grid master-order pushes still arrive on the TradeContext private topic.
    let (_trade, mut receiver) = TradeContext::new(config);

    // Security (symbol) info used to build a grid order (lot size, auth flag).
    let info = ctx.symbol_info("700.HK").await?;
    println!("grid symbol info: {info:?}");

    // Submit a grid order.
    let rule = GridTradeRule {
        submitted_base_price: Some(Decimal::new(300, 0)),
        upper_limit_price: Some(Decimal::new(360, 0)),
        lower_limit_price: Some(Decimal::new(240, 0)),
        trigger_price_type: Some(2), // 1 = spread, 2 = percent
        trigger_percent_up: Some(Decimal::new(2, 0)),
        trigger_percent_down: Some(Decimal::new(2, 0)),
        trigger_quantity: Some(Decimal::new(100, 0)),
        upper_limit_quantity: Some(Decimal::new(200, 0)),
        lower_limit_quantity: Some(Decimal::new(100, 0)),
        time_in_force: Some(1), // GTC
        grid_order_type_up: Some("GMO".to_string()),
        grid_order_type_down: Some("GMO".to_string()),
        upper_limit_event: Some(1),
        lower_limit_event: Some(1),
        trigger_sell_depth: Some(0),
        trigger_buy_depth: Some(0),
        support_shortsell: Some(false),
        rth: Some(0),
        multiple_trigger: Some(false),
        expire_time: Some(0),
        ..Default::default()
    };
    let submitted = ctx
        .submit(SubmitGridOrderOptions::new("700.HK", "HKD", rule))
        .await?;
    let order_id = submitted.order_id;
    println!("submitted grid order: {order_id}");

    // List grid orders.
    let list = ctx
        .list(GetGridOrdersOptions::new().symbol("700.HK").limit(20))
        .await?;
    println!("grid orders: {} (has_more={})", list.grid_order.len(), list.has_more);

    // Detail.
    let detail = ctx
        .detail(GetGridOrderDetailOptions::new(&order_id))
        .await?;
    println!("grid order detail: {detail:?}");

    // Query by IDs.
    let by_ids = ctx
        .list_by_ids(longbridge::grid::GetGridOrdersByIdsOptions::new([&order_id]))
        .await?;
    println!("grid orders by ids: {}", by_ids.len());

    // Trigger history.
    let history = ctx
        .trigger_history(GetGridTriggerHistoryOptions::new(&order_id))
        .await?;
    println!(
        "trigger history: {} (has_more={})",
        history.trigger_orders.len(),
        history.has_more
    );

    // Suspend -> restart -> cancel.
    ctx.suspend(&order_id).await?;
    ctx.restart(&order_id).await?;
    ctx.cancel(&order_id).await?;
    println!("suspend / restart / cancel done");

    // Drain a few push events (grid master-order changes arrive here).
    while let Ok(event) =
        tokio::time::timeout(std::time::Duration::from_secs(2), receiver.recv()).await
    {
        match event {
            Some(evt) => println!("push: {evt:?}"),
            None => break,
        }
    }

    Ok(())
}
