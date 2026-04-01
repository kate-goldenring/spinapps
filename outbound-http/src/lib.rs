use anyhow::Context;
use spin_sdk::http::{IntoResponse, Method, Request, Response};
use spin_sdk::http_component;
use spin_sdk::variables;

#[http_component]
async fn handle_hello_rust(_req: Request) -> anyhow::Result<impl IntoResponse> {
    // Create the outbound request object
    let host = variables::get("outbound_host").context("missing outbound_host variable")?;
    let number_of_requests: usize = variables::get("number_of_requests")
        .context("missing number_of_requests variable")?
        .parse()
        .context("number_of_requests variable is not a valid integer")?;
    for _ in 0..number_of_requests {
        let request = Request::builder()
        .method(Method::Get)
        .uri(&host)
        .build();
        let _response: Response = spin_sdk::http::send(request).await.context("failed to get response")?;
    }
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(format!("Made {} outbound requests to {}", number_of_requests, host))
        .build())
}
