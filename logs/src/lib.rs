use std::collections::HashMap;

use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_service;

fn parse_query_params(url: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();
    if let Some(query) = url.split('?').nth(1) {
        for pair in query.split('&') {
            let mut parts = pair.splitn(2, '=');
            if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                params.insert(key.to_string(), value.to_string());
            }
        }
    }
    params
}

fn get_param(params: &HashMap<String, String>, key: &str, default: usize) -> usize {
    params.get(key).and_then(|v| v.parse().ok()).unwrap_or(default)
}

/// Spin HTTP component that emits configurable log output for testing platform log capacity.
///
/// Query parameters:
///   - `line_size`   : bytes per log line (default: 100)
///   - `lines`       : number of log lines to emit (default: 10)
///   - `total_bytes` : cap on total bytes emitted (default: unlimited)
#[http_service]
async fn handle_logs(req: Request) -> anyhow::Result<impl IntoResponse> {
    let url = req
        .headers()
        .get("spin-full-url")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let params = parse_query_params(url);

    let line_size = get_param(&params, "line_size", 100);
    let lines = get_param(&params, "lines", 10);
    let total_bytes = get_param(&params, "total_bytes", usize::MAX);

    let mut bytes_emitted: usize = 0;
    let mut lines_emitted: usize = 0;

    for i in 0..lines {
        if bytes_emitted >= total_bytes {
            break;
        }

        // Build a log line targeting `line_size` bytes
        let prefix = format!("[{}/{}] ", i + 1, lines);
        let fill_len = line_size.saturating_sub(prefix.len());
        let mut line = format!("{}{}", prefix, "A".repeat(fill_len-1));
        line.push('D');
        line.truncate(line_size);

        // Enforce total_bytes cap (include the newline println! adds)
        let line_total = line.len() + 1;
        let remaining = total_bytes.saturating_sub(bytes_emitted);
        if line_total > remaining {
            line.truncate(remaining.saturating_sub(1));
        }

        bytes_emitted += line.len() + 1;
        lines_emitted += 1;
        println!("{}", line);
    }

    let total_bytes_display = if total_bytes == usize::MAX {
        "unlimited".to_string()
    } else {
        total_bytes.to_string()
    };

    let summary = format!(
        "Emitted {} log lines, {} bytes total\n\
         Config: line_size={}, lines={}, total_bytes={}",
        lines_emitted, bytes_emitted, line_size, lines, total_bytes_display
    );

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(summary))
}
