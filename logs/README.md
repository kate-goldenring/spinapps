# logs

A Spin HTTP application for testing log capacity on the platform that runs it. Send a request with configurable parameters to emit a controlled volume of log output.

## Build

```sh
cargo build --target wasm32-wasip2 --release
```

## Run

```sh
spin up
```

The app listens on `http://localhost:3000` by default.

## Usage

Send a `GET` request to `/` with any combination of the following query parameters:

| Parameter | Description | Default |
|---|---|---|
| `line_size` | Bytes per log line | `100` |
| `lines` | Number of log lines to emit per request | `10` |
| `total_bytes` | Cap on total bytes emitted (hard limit) | unlimited |

### Examples

Emit 10 lines of 100 bytes each (defaults):
```sh
curl "http://localhost:3000/"
```

Emit 100 lines of 512 bytes each:
```sh
curl "http://localhost:3000/?line_size=512&lines=100"
```

Emit lines of 1 KB each, capped at 50 KB total:
```sh
curl "http://localhost:3000/?line_size=1024&lines=100&total_bytes=51200"
```

### Response

The response body contains a summary of what was emitted, for example:

```
Emitted 49 log lines, 50175 bytes total
Config: line_size=1024, lines=100, total_bytes=51200
```

### Notes

- `total_bytes` counts the newline character appended by each `println!` call. The last line may be truncated to stay within the cap.
- If `total_bytes` is reached before all `lines` are emitted, emission stops early and the actual count is reflected in the response.
