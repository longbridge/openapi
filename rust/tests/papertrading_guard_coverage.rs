//! Every write endpoint must decide, explicitly, whether paper trading mode
//! may reach it.
//!
//! `RequestBuilder::papertrading_restrict()` is opt-in: a call site that does
//! not add it is simply unguarded, and nothing fails. That is how the first
//! version of the guard covered the three `/v1/trade/order*` endpoints while
//! leaving grid trading and fund subscription — which also commit real money —
//! wide open. A silent omission in a safety guard is the failure mode the
//! guard exists to prevent, so this test makes the omission loud: it scans the
//! SDK's own source for write requests and compares what it finds against the
//! two lists below.
//!
//! Adding a write endpoint therefore forces a choice: guard it, or record here
//! why it does not need guarding.

use std::{collections::BTreeSet, fs, path::Path};

/// Write requests that refuse to run in paper trading mode on a credential
/// whose data center has no paper trading accounts.
const GUARDED: &[&str] = &[
    "fund/context.rs POST /v1/fund/orders",
    // `grid_action` decides per caller: restart is guarded, cancel and suspend
    // are not (they only ever reduce exposure).
    "grid/context.rs POST path",
    "grid/context.rs POST /v1/gridtrading/replace",
    "grid/context.rs POST /v1/gridtrading/submit",
    "trade/context.rs POST /v1/trade/order",
    "trade/context.rs POST /v1/trade/order/multileg",
    "trade/context.rs PUT /v1/trade/order",
];

/// Write requests that are deliberately left open, with the reason.
///
/// Three kinds appear here:
/// - **Unwinding actions** (cancel, suspend): they only ever reduce exposure.
///   Refusing them would strand a caller holding a live order they are trying
///   to pull, which is the one failure mode that can cost money.
/// - **Reads that happen to be POSTs** (queries, validations, searches): they
///   return real-money data, which is misleading but not destructive — the same
///   call the guard already allows in its `GET` form.
/// - **Non-trading writes**: watchlists, alerts, share lists, content.
const UNGUARDED: &[&str] = &[
    // Unwinding actions.
    "fund/context.rs POST /v1/fund/orders/{order_id}/cancel",
    "trade/context.rs DELETE /v1/trade/order",
    // Queries and validations served over POST.
    "fund/context.rs POST /v1/fund/funds",
    "fund/context.rs POST /v1/fund/orders/validate",
    "grid/context.rs POST /v1/gridtrading/list",
    "market/context.rs POST path",
    "quote/context.rs POST /v1/quote/options/strikes",
    "quote/context.rs POST path",
    "screener/context.rs POST path",
    "trade/context.rs POST /v1/us/assets/transaction-history",
    "trade/context.rs POST /v1/us/orders/query",
    // Recurring investment: AP-only (`dc_restrict(Ap)`), so a US credential —
    // the only case this guard can detect — is already refused there.
    "dca/context.rs POST path",
    // Non-trading writes.
    "agent/context.rs POST /v1/ai/agents/{agent_id}/conversations",
    "agent/context.rs POST /v1/ai/agents/{agent_id}/conversations/{chat_uid}/messages/{message_id}/continue",
    "alert/context.rs DELETE path",
    "alert/context.rs POST path",
    "content/context.rs POST /v1/content/topics",
    "content/context.rs POST /v1/content/topics/{topic_id}/comments",
    "quote/context.rs DELETE /v1/watchlist/groups",
    "quote/context.rs POST /v1/watchlist/groups",
    "quote/context.rs POST /v1/watchlist/pinned",
    "quote/context.rs PUT /v1/watchlist/groups",
    "sharelist/context.rs DELETE path.leak()",
    "sharelist/context.rs POST path",
];

/// One write request found in the source: `"<file> <METHOD> <path>"`, where
/// the path is the literal (or `format!` template) the call passes, or the
/// variable's name when the path is computed.
fn scan(dir: &Path, out: &mut (BTreeSet<String>, BTreeSet<String>)) {
    for entry in fs::read_dir(dir).expect("src directory must be readable") {
        let path = entry.expect("readable entry").path();
        if path.is_dir() {
            scan(&path, out);
            continue;
        }
        if path.extension().is_none_or(|ext| ext != "rs") {
            continue;
        }
        let source = fs::read_to_string(&path).expect("source must be readable");
        let name = path
            .strip_prefix(Path::new(env!("CARGO_MANIFEST_DIR")).join("src"))
            .expect("inside src")
            .to_string_lossy()
            .replace('\\', "/");
        for (offset, _) in source.match_indices(".request(") {
            let tail = &source[offset..];
            // The call may wrap over several lines. It ends at the statement's
            // `;`, or where the next builder method starts on a line of its
            // own — whichever comes first.
            let next_builder_line = tail
                .match_indices('\n')
                .map(|(i, _)| i)
                .find(|&i| tail[i + 1..].trim_start().starts_with('.'))
                .unwrap_or(tail.len());
            let statement_end = tail.find(';').unwrap_or(tail.len());
            let call_end = next_builder_line.min(statement_end);
            let call = normalize(&tail[..call_end]);
            let Some(args) = call
                .strip_prefix(".request(")
                .and_then(|rest| rest.split_once(','))
            else {
                continue;
            };
            let method = args.0.trim().trim_start_matches("Method::");
            if !matches!(method, "POST" | "PUT" | "DELETE") {
                continue;
            }
            let argument = path_key(args.1);
            let entry = format!("{name} {method} {argument}");
            // The marker sits on one of this chain's builder lines, so stop at
            // the `.send()` that ends the chain — otherwise a guarded call
            // further down the file would be credited to this one.
            let chain_end = tail
                .find(".send()")
                .map_or(tail.len().min(600), |end| end.min(600));
            let window = &tail[..chain_end];
            if window.contains(".papertrading_restrict()") {
                out.0.insert(entry);
            } else {
                out.1.insert(entry);
            }
        }
    }
}

/// The path a `.request(…)` call targets: its string literal when it has one
/// (including a `format!` template), otherwise the expression's text.
fn path_key(argument: &str) -> String {
    if let Some(open) = argument.find('"')
        && let Some(close) = argument[open + 1..].find('"')
    {
        return argument[open + 1..open + 1 + close].to_string();
    }
    // Drop a trailing comment, then the `)` that closes `.request(` itself —
    // but only that one, so `path.leak()` keeps its own parentheses.
    let mut text = argument
        .split("//")
        .next()
        .unwrap_or(argument)
        .trim()
        .trim_end_matches(',')
        .trim()
        .to_string();
    while text.ends_with(')') && text.matches(')').count() > text.matches('(').count() {
        text.pop();
        text = text.trim_end().trim_end_matches(',').trim_end().to_string();
    }
    text
}

/// Collapse the whitespace of a possibly multi-line call argument list.
fn normalize(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[test]
fn every_write_endpoint_decides_about_paper_trading() {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut found = (BTreeSet::new(), BTreeSet::new());
    scan(&src, &mut found);
    let (guarded, unguarded) = found;

    let expected_guarded: BTreeSet<String> = GUARDED.iter().map(|s| s.to_string()).collect();
    let expected_unguarded: BTreeSet<String> = UNGUARDED.iter().map(|s| s.to_string()).collect();

    let lost: Vec<_> = expected_guarded.difference(&guarded).collect();
    assert!(
        lost.is_empty(),
        "these endpoints lost their `.papertrading_restrict()`; a paper-trading order would reach \
         the real-money account again: {lost:#?}"
    );

    let new_unguarded: Vec<_> = unguarded.difference(&expected_unguarded).collect();
    assert!(
        new_unguarded.is_empty(),
        "new write endpoint(s) with no paper-trading decision: {new_unguarded:#?}\n\
         Either add `.papertrading_restrict()` (anything that can place or resume an order), or \
         add it to UNGUARDED in this test with the reason."
    );

    let newly_guarded: Vec<_> = guarded.difference(&expected_guarded).collect();
    assert!(
        newly_guarded.is_empty(),
        "endpoint(s) guarded but not listed in GUARDED — add them: {newly_guarded:#?}"
    );

    let gone: Vec<_> = expected_unguarded.difference(&unguarded).collect();
    assert!(
        gone.is_empty(),
        "UNGUARDED lists endpoint(s) that no longer exist unguarded; remove them: {gone:#?}"
    );
}
