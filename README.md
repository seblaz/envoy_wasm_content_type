# Envoy wasm content type

This repo demonstrates that when sending a response inside a wasm filter the `Content-Type` header is set automatically.

## Run example

Run `docker-compose up`. That will compile the wasm filter and then start envoy with the filter. After that you can go to [localhost:1000](http://localhost:1000) and you will see the response `response from mock server` that doesn't include a `Content-Type` header. In the other hand, if you go to [localhost:2000](http://localhost:2000) you will see the response `response from wasm filter` that includes the `Content-Type` header, although the response sent from wasm didn't include that header.
