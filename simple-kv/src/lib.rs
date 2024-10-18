use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use spin_sdk::key_value::Store;


#[http_component]
fn handle_kv(_req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("handle_kv - entered and should return 'myvalue'");
    let store = Store::open_default()?;
    store.set("mykey", b"myvalue")?;
    let value = store.get("mykey")?;
    let response = value.unwrap_or_else(|| "not found".into());
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(response)
        .build())
}
