# Useful commands:
```bash
cargo test

cargo build --release && ./target/release/kv-service 127.0.0.1:8080
#or
cargo run -- 127.0.0.1:8080

curl -v "127.0.0.1:8080/remember?key=key1" -d value1
curl -v 127.0.0.1:8080/lookup/key1
```
# Design decisions:
## HTTP Responses:
For `POST /remember?key=$KEY`:
- `201 Created` - for well formatted requests (valid key and value)
- `400 Bad Request` - missing key, empty body, empty key

For `GET /lookup/$KEY`:
- `200 OK` and the stored value in body (text/plain) - if the key exists in the store
- `204 No Content` - if the key doesn't exist in the store
	- I took this decision because with `200 OK` and a message like `KEY NOT FOUND` in body, the message could be confused with a valid value
## Testing:
Because tests are ran concurrently and we run an instance of the server for each one, we need to bind each test server to a different port. This is why we have a helper function [`build_test_server`](https://github.com/razvp/kv-service/blob/c0dd8bc9958401249ad1911697adf9a639ff36a8/tests/helpers.rs#L16-L22) that "asks" the OS for a free port and returns it so the client in each test can use it. 

To simulate integration testing with concurrent clients there is a test that uses separate threads for 2 clients. This test checks that an insert done by `client_one` is immediately seen by `client_two`. To synchronize the 2 clients we use a `oneshot channel`.
There are more explanations in the [comments](https://github.com/razvp/kv-service/blob/c0dd8bc9958401249ad1911697adf9a639ff36a8/tests/concurrent_clients.rs#L6-L9). 
# Requirements:
Write a simple HTTP server that can store key=value pairs and return them on
demand.
1. POST /remember?key=$KEY with the value as a string body.
2. GET /lookup/$KEY: return the entry for $KEY, or return an appropriate error
response.
This service doesn't need to persist key=value pairs across reboots - all data
is ephemeral. If a key already exists, the value is replaced (upsert semantics).
