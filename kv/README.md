# Key Value Spin Application

```sh
spin build --up
```

Make a request to the application:

```sh
$ curl localhost:3000/help
Available endpoints:
GET /hello - returns a hello world message
GET /all - returns all keys
POST /get - retrieves a value by key in JSON format '{"key": "mykey"}'
POST /set - sets a key-value pair in JSON format '{"key": "mykey", "value": "myvalue"}'
GET /exists/:key - checks if a key exists in the store. Found: 200; Not Found: 404 
POST /:key - sets the value for the key to be the http body: curl localhost:3000/foo -d "bar"
GET /:key - retrieves a value for the key in the URL path
GET /5/:key - retrieves a value by key (5 times)
POST /kbs/:size - sets a key-value pair with a specific size in KBs in the key 'kbs'
DELETE /:key - deletes a key-value pair
GET /help - displays this help message%  
```

TODO: Add WASI KV operations