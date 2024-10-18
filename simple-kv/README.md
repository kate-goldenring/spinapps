# Simple KV Test App

## Local with Redis

1. Run redis server
```sh
redis-server --port 6379
```
2. Run spin with runtime config
```sh
spin build --up --runtime-config-file runtime-config-local.toml 
```

## Pushing
```sh
spin registry push ghcr.io/kate-goldenring/spin-apps/simple-redis:$(date +"%m-%d-%Y-%H-%M")
```

## On SpinKube with Runtime Config
```sh
kubectl apply -f redis.yaml
spin kube scaffold -f ghcr.io/kate-goldenring/spin-apps/simple-redis:10-18-2024-10-23 -c runtime-config-k8s.toml | kubectl apply -f -
```