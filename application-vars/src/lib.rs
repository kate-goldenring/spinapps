use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use spin_sdk::variables;

#[http_component]
fn handle_application_vars(_req: Request) -> anyhow::Result<impl IntoResponse> {
    let foo = variables::get("foo").expect("could not get variable");
    let bar = variables::get("bar").expect("could not get variable");
    println!("foo: {}, bar: {}", foo, bar);
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(format!("foo: {}, bar: {}", foo, bar))
        .build())
}
