use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use spin_sdk::variables;

/// A simple Spin HTTP component.
#[http_component]
fn handle_hello_spin(req: Request) -> anyhow::Result<impl IntoResponse> {
    let response = ab_test_with_kv()?;
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(response)
        .build())
}

fn ab_test_with_variables() -> anyhow::Result<String> {
    let variant = variables::get("variant")?;
    Ok(get_result(&variant))
}

fn get_result(variant: &str) -> String {
    if variant == "A" {
        "single".to_string()
    } else {
        "split".to_string()
    }
}

fn ab_test_with_kv() -> anyhow::Result<String> {
    let kv = spin_sdk::key_value::Store::open_default()?;
    let val = kv.get("variant")?.unwrap_or_else(|| b"A".to_vec());
    let variant = String::from_utf8(val)?;
    // If A, set B; if B, set A.
    if variant == "A" {
        kv.set("variant", b"B")?;
        Ok("single".to_string())
    } else {
        kv.set("variant", b"A")?;
        Ok("split".to_string())
    }
}
