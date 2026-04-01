use anyhow::{Context, Result};
use serde::Deserialize;
use spin_sdk::{
    http::{Params, Request, Response, Router},
    http_component,
    key_value::Store,
};

#[derive(Deserialize)]
struct Pair {
    key: String,
    value: String,
}

#[derive(Deserialize)]
struct Key {
    key: String,
}

#[http_component]
fn handle_cosmos_showcase(req: Request) -> Result<Response> {
    let mut router = Router::default();
    router.get("/hello", hello_world);
    router.get("/exists/:key", exists);
    router.get("/repeat/:count", get_value_times);
    router.delete("/:key", delete_value);
    router.get("/all", get_all);
    router.post("/get", get_value_json);
    router.post("/set", set_pair);
    router.post("/:key", set_value);
    router.get("/help", help);
    router.post("/kbs/:size", set_value_with_size_kbs);
    router.get("/:key", get_value);
    router.get("/", help);
    router.handle(req)
}

fn help(_req: Request, _params: Params) -> Result<Response> {
    println!("entered help");
    let response = "Available endpoints:\n\
                   GET /hello - returns a hello world message\n\
                   GET /all - returns all keys\n\
                   GET /repeat/:count - retrieves the value at key 'key' :count times\n\
                   POST /get - retrieves a value by key in JSON format '{\"key\": \"mykey\"}'\n\
                   POST /set - sets a key-value pair in JSON format '{\"key\": \"mykey\", \"value\": \"myvalue\"}'\n\
                   GET /exists/:key - checks if a key exists in the store. Found: 200; Not Found: 404 \n\
                   POST /:key - sets the value for the key to be the http body: curl localhost:3000/foo -d \"bar\"\n\
                   GET /:key - retrieves a value for the key in the URL path\n\
                   GET /5/:key - retrieves a value by key (5 times)\n\
                   POST /kbs/:size - sets a key-value pair with a specific size in KBs in the key 'kbs'\n\
                   DELETE /:key - deletes a key-value pair\n\
                   GET /help - displays this help message";
    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        .body(Some(response.into()))?)
}

fn set_value_with_size_kbs(req: Request, params: Params) -> Result<Response> {
    println!("entered set_value_with_size_kbs");
    let store = Store::open_default()?;
    let Some(size) = params.get("size") else {
        return bad_request();
    };
    let size_kbs: usize = size.parse().unwrap_or(0);
    if size_kbs == 0 {
        return bad_request();
    }
    let value = kbs_string(size_kbs);
    let key = "kbs";
    match store.set(key, &value) {
        Ok(_) => Ok(http::Response::builder()
            .status(http::StatusCode::OK)
            .body(Some(
                format!("set key {} with size {} KBs", key, size_kbs).into(),
            ))?),
        Err(e) => err_str(e.to_string()),
    }
}

fn kbs_string(size_kbs: usize) -> String {
    // 1 MB in bytes
    let size = 1024 * size_kbs;
    // Repeat the character 'A' until it's size_kbs KBs long
    let kbs_string: String = "A".repeat(size);
    kbs_string
}

fn set_pair(req: Request, _params: Params) -> Result<Response> {
    let store = Store::open_default()?;
    let pair: Pair =
        serde_json::from_slice(req.body().as_deref().unwrap_or(&[])).context("cant parse json")?;
    println!("setting {} : {}", pair.key, &pair.value);
    match store.set(&pair.key, &pair.value) {
        Ok(_) => Ok(http::Response::builder()
            .status(http::StatusCode::OK)
            .body(None)?),
        Err(e) => err_str(e.to_string()),
    }
}

/// handler to get the value at key from the store
/// if the key does not exist, it returns a 404
fn get_value(_req: Request, params: Params) -> Result<Response> {
    let store = Store::open_default()?;
    let Some(key) = params.get("key") else {
        return bad_request();
    };
    println!("get_value - key {} exists", key);
    match store.get(key) {
        Ok(value) => Ok(http::Response::builder()
            .status(http::StatusCode::OK)
            .body(Some(value.into()))?),
        Err(e) => err_str(e.to_string()),
    }
}

fn get_value_json(req: Request, _params: Params) -> Result<Response> {
    println!("entered get_value_json");
    let store = Store::open_default()?;
    let key: Key =
        serde_json::from_slice(req.body().as_deref().unwrap_or(&[])).context("cant parse json")?;
    println!("getting {}", key.key);
    match store.get(&key.key) {
        Ok(value) => Ok(http::Response::builder()
            .status(http::StatusCode::OK)
            .body(Some(value.into()))?),
        Err(e) => err_str(e.to_string()),
    }
}

fn exists(_req: Request, params: Params) -> Result<Response> {
    println!("entered exists");
    let store = Store::open_default()?;
    let Some(key) = params.get("key") else {
        return bad_request();
    };
    println!("checking if key {} exists", key);
    match store.exists(key) {
        Ok(true) => Ok(http::Response::builder()
            .status(http::StatusCode::OK)
            .body(None)?),
        Ok(false) => Ok(http::Response::builder()
            .status(http::StatusCode::NOT_FOUND)
            .body(None)?),
        Err(e) => err_str(e.to_string()),
    }
}

fn delete_value(_req: Request, params: Params) -> Result<Response> {
    let store = Store::open_default()?;
    let Some(key) = params.get("key") else {
        return bad_request();
    };
    match store.delete(key) {
        Ok(_) => Ok(http::Response::builder()
            .status(http::StatusCode::OK)
            .body(None)?),
        Err(e) => err_str(e.to_string()),
    }
}

/// handler to get the value at key from the store
/// if the key does not exist, it returns a 404
fn get_all(_req: Request, _params: Params) -> Result<Response> {
    let store = Store::open_default()?;
    match store.get_keys() {
        Ok(values) => Ok(http::Response::builder()
            .status(http::StatusCode::OK)
            .body(Some(values.join(",").into()))?),
        Err(e) => err_str(e.to_string()),
    }
}

fn get_value_times(_req: Request, params: Params) -> Result<Response> {
    let store = Store::open_default()?;
    let Some(times) = params.get("count") else {
        return bad_request();
    };
    store.set("key", "value")?;
    let mut val: Vec<u8> = vec![];
    for _ in 0..times.parse::<usize>().unwrap_or(1) {
        match store.get("key") {
            Ok(mut value) => val.append(&mut value),
            Err(_) => return err(),
        }
    }
    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        .body(Some(val.into()))?)
}

/// Handler to store a value in key-value store
fn set_value(req: Request, params: Params) -> Result<Response> {
    println!("entered set_value");
    let store = Store::open_default()?;
    let Some(key) = params.get("key") else {
        return bad_request();
    };
    match store.set(key, req.body().as_deref().unwrap_or(&[])) {
        Ok(_) => Ok(http::Response::builder()
            .status(http::StatusCode::OK)
            .body(None)?),
        Err(e) => err_str(e.to_string()),
    }
}

fn hello_world(_req: Request, _params: Params) -> Result<Response> {
    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        .body(Some("Hello, World!".into()))?)
}

/// helper function to quickly return a bad request HTTP Response
fn bad_request() -> Result<Response> {
    Ok(http::Response::builder()
        .status(http::StatusCode::BAD_REQUEST)
        .body(None)?)
}

/// helper function to quickly return an internal server error HTTP Response
fn err() -> Result<Response> {
    Ok(http::Response::builder()
        .status(http::StatusCode::INTERNAL_SERVER_ERROR)
        .body(None)?)
}

/// helper function to quickly return an internal server error HTTP Response
fn err_str(message: String) -> Result<Response> {
    let message = message.into_bytes();
    Ok(http::Response::builder()
        .status(http::StatusCode::INTERNAL_SERVER_ERROR)
        .body(Some(message.into()))?)
}

fn log_err(err: impl std::fmt::Debug) -> Result<Response> {
    println!("key-value error: {err:?}");
    Ok(http::Response::builder()
        .status(http::StatusCode::INTERNAL_SERVER_ERROR)
        .body(None)?)
}
