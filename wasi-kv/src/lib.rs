use helper::{bail, ensure_matches, ensure_ok};

use helper::http_trigger_bindings::wasi::keyvalue::atomics as wasi_atomics;
use helper::http_trigger_bindings::wasi::keyvalue::batch as wasi_batch;
use helper::http_trigger_bindings::wasi::keyvalue::store::{Error, KeyResponse, open};

use spin_sdk::http::{Params, Request, Response, Router};
use spin_sdk::http_component;
use anyhow::{Context, Result};

#[http_component]
fn handle_wasi_kv(req: Request) -> Response {
    let mut router = Router::default();
    // router.get("/test", test);
    router.get("/", help);
    router.get("/exists/:key", exists);
    router.get("/keys", get_keys);
    router.post("/batch/set", batch_set);
    router.post("/batch/get", batch_get);
    router.post("/batch/delete", batch_delete);
    router.post("/atomic/increment/:key", increment);
    router.post("/atomic/cas/:key", compare_and_swap);
    router.get("/:key", get_value);
    router.post("/:key", set_value);
    router.delete("/:key", delete_value);
    router.handle(req)
}

fn get_value(_req: Request, params: Params) -> Result<Response> {
 let store = open("default")?;
     let Some(key) = params.get("key") else {
        return bad_request();
    };
    match store.get(key) {
        Ok(value) => Ok(Response::new(200, value)),
        Err(e) => err_str(e.to_string()),
    }

}

fn set_value(req: Request, params: Params) -> Result<Response> {
 let store = open("default")?;
     let Some(key) = params.get("key") else {
        return bad_request();
    };
    match store.set(key,req.body()) {
        Ok(_) => Ok(Response::new(200, "".as_bytes().to_vec())),
        Err(e) => err_str(e.to_string()),
    }

}

fn delete_value(_req: Request, params: Params) -> Result<Response> {
    let store = open("default")?;
    let Some(key) = params.get("key") else {
        return bad_request();
    };
    match store.delete(key) {
        Ok(_) => Ok(Response::new(200, "".as_bytes().to_vec())),
        Err(e) => err_str(e.to_string()),
    }
}

fn exists(_req: Request, params: Params) -> Result<Response> {
    let store = open("default")?;
    let Some(key) = params.get("key") else {
        return bad_request();
    };
    match store.exists(key) {
        Ok(true) => Ok(Response::new(200, "true".as_bytes().to_vec())),
        Ok(false) => Ok(Response::new(404, "false".as_bytes().to_vec())),
        Err(e) => err_str(e.to_string()),
    }
}

fn get_keys(_req: Request, _params: Params) -> Result<Response> {
    let store = open("default")?;
    match store.list_keys(None) {
        Ok(key_response) => {
            let json = serde_json::to_vec(&key_response.keys)
                .unwrap_or_else(|e| e.to_string().into_bytes());
            Ok(Response::new(200, json))
        }
        Err(e) => err_str(e.to_string()),
    }
}

#[derive(serde::Deserialize)]
struct KvPair {
    key: String,
    value: String,
}

#[derive(serde::Deserialize)]
struct KeyList {
    keys: Vec<String>,
}

fn batch_set(req: Request, _params: Params) -> Result<Response> {
    let store = open("default")?;
    let pairs: Vec<KvPair> = serde_json::from_slice(req.body())
        .map_err(|e| anyhow::anyhow!(e))?;
    let entries: Vec<(String, Vec<u8>)> = pairs
        .into_iter()
        .map(|p| (p.key, p.value.into_bytes()))
        .collect();
    match wasi_batch::set_many(&store, &entries) {
        Ok(_) => Ok(Response::new(200, "".as_bytes().to_vec())),
        Err(e) => err_str(e.to_string()),
    }
}

fn batch_get(req: Request, _params: Params) -> Result<Response> {
    let store = open("default")?;
    let key_list: KeyList = serde_json::from_slice(req.body())
        .map_err(|e| anyhow::anyhow!(e))?;
    match wasi_batch::get_many(&store, &key_list.keys) {
        Ok(results) => {
            let pairs: Vec<(String, Option<String>)> = results
                .into_iter()
                .map(|(k, v)| (k, v.map(|b| String::from_utf8_lossy(&b).into_owned())))
                .collect();
            let json = serde_json::to_vec(&pairs)
                .unwrap_or_else(|e| e.to_string().into_bytes());
            Ok(Response::new(200, json))
        }
        Err(e) => err_str(e.to_string()),
    }
}

fn batch_delete(req: Request, _params: Params) -> Result<Response> {
    let store = open("default")?;
    let key_list: KeyList = serde_json::from_slice(req.body())
        .map_err(|e| anyhow::anyhow!(e))?;
    match wasi_batch::delete_many(&store, &key_list.keys) {
        Ok(_) => Ok(Response::new(200, "".as_bytes().to_vec())),
        Err(e) => err_str(e.to_string()),
    }
}

fn increment(req: Request, params: Params) -> Result<Response> {
    let store = open("default")?;
    let Some(key) = params.get("key") else {
        return bad_request();
    };
    let delta: i64 = serde_json::from_slice(req.body())
        .map_err(|e| anyhow::anyhow!(e))?;
    match wasi_atomics::increment(&store, key, delta) {
        Ok(new_val) => {
            let json = serde_json::to_vec(&new_val)
                .unwrap_or_else(|e| e.to_string().into_bytes());
            Ok(Response::new(200, json))
        }
        Err(e) => err_str(e.to_string()),
    }
}

#[derive(serde::Deserialize)]
struct CasRequest {
    current_value: String,
    new_value: String,
}

fn compare_and_swap(req: Request, params: Params) -> Result<Response> {
    let store = open("default")?;
    let Some(key) = params.get("key") else {
        return bad_request();
    };
    let body: CasRequest = serde_json::from_slice(req.body())
        .map_err(|e| anyhow::anyhow!(e))?;
    let cas = wasi_atomics::Cas::new(&store, key)
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;
    if let Some(current) = cas.current().map_err(|e| anyhow::anyhow!(e.to_string()))? {
        if current != body.current_value.as_bytes() {
            return Ok(Response::new(409, "current value does not match".as_bytes().to_vec()));
        }
    } else {
        return Ok(Response::new(404, "key not found".as_bytes().to_vec()));
    }
    match wasi_atomics::swap(cas, body.new_value.as_bytes()) {
        Ok(_) => Ok(Response::new(200, "".as_bytes().to_vec())),
        Err(e) => err_str(e.to_string()),
    }
}

fn help(_req: Request, _params: Params) -> Response {
    println!("entered help");
    let response = "Available endpoints:\n\
                   GET / - displays this help message\n\
                   GET /:key - retrieves a value for the key in the URL path\n\
                   POST /:key - sets the value for the key to be the http body: curl localhost:3000/foo -d \"bar\"\n\
                   DELETE /:key - deletes a key-value pair\n\
                   GET /exists/:key - checks if a key exists in the store. Found: 200; Not Found: 404\n\
                   GET /keys - returns all keys as JSON array\n\
                   POST /batch/set - sets multiple key-value pairs: [{\"key\": \"k\", \"value\": \"v\"}, ...]\n\
                   POST /batch/get - gets multiple values: {\"keys\": [\"k1\", \"k2\"]}\n\
                   POST /batch/delete - deletes multiple keys: {\"keys\": [\"k1\", \"k2\"]}\n\
                   POST /atomic/increment/:key - atomically increments a key by a delta: curl localhost:3000/atomic/increment/counter -d \"2\"\n\
                   POST /atomic/cas/:key - compare-and-swap: {\"current_value\": \"val1\", \"new_value\": \"val2\"}";
    Response::new(200, response.as_bytes().to_vec())
}

fn keys<E>(res: &Result<KeyResponse, E>) -> Result<&[String], &E> {
    res.as_ref().map(|kr| kr.keys.as_slice())
}


/// helper function to quickly return a bad request HTTP Response
fn bad_request() -> Result<Response> {
    Ok(Response::new(400, "".as_bytes().to_vec()))
}

/// helper function to quickly return an internal server error HTTP Response
fn err() -> Result<Response> {
    Ok(Response::new(500, "".as_bytes().to_vec()))
}

/// helper function to quickly return an internal server error HTTP Response
fn err_str(message: String) -> Result<Response> {
    Ok(Response::new(500, message.as_bytes().to_vec()))
}

fn log_err(err: impl std::fmt::Debug) -> Result<Response> {
    println!("key-value error: {err:?}");
    Ok(Response::new(500, "".as_bytes().to_vec()))
}
