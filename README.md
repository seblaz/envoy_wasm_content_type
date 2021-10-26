# Envoy wasm response properties

This repo demonstrates that asking for the response status code in a wasm filter returns None. According to [the documentation](https://www.envoyproxy.io/docs/envoy/v1.19.0/intro/arch_overview/advanced/attributes), the status code should be returned.

## Run example

Run `docker-compose up`. That will compile the wasm filter and then start envoy with the filter. After that you can go to [localhost:10000/green](http://localhost:10000/green) and you will see the log `code: None` in the console.