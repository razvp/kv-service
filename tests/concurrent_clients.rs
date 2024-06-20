#[path = "helpers.rs"]
mod helpers;

use std::{thread, time::Duration};

// Using `tokio::test` or `actix_web::test` would spawn a single-threaded
// tokio runtime and will run both clients and the server on it.
// To better simulate real life concurrent clients we force the test
// to use 3 separate runtimes on different threads.
#[test]
fn remember_request_is_immediately_seen_by_other_clients() {
    // Use a channel to get the `test_port` from the server thread
    let (tx, rx) = std::sync::mpsc::channel();
    let _server_thread = thread::spawn(move || {
        let rt = helpers::build_tokio_rt();
        rt.block_on(async move {
            let (server, test_server_port) = helpers::build_test_server();
            tx.send(test_server_port).unwrap();
            let _ = server.await;
        });
    });

    let test_port = rx.recv().unwrap();
    dbg!(&test_port);

    // Use a channel to sync the clients
    let (tx, rx) = tokio::sync::oneshot::channel::<()>();
    let client_one_thread = thread::spawn(move || {
        let rt = helpers::build_tokio_rt();
        rt.block_on(async move {
            let client = httpc_test::new_client(format!("http://127.0.0.1:{}", test_port)).unwrap();
            // Give some time to `client_two` to check that `key` doesn't exist
            // then add `key1=value1` and notify `client_two` through the `channel`.
            tokio::time::sleep(Duration::from_millis(100)).await;
            let res = client
                .do_post("/remember?key=key1", "value1")
                .await
                .unwrap();
            assert_eq!(res.status(), 201);
            tx.send(()).unwrap();
        });
    });

    let client_two_thread = thread::spawn(move || {
        let rt = helpers::build_tokio_rt();
        rt.block_on(async move {
            let client = httpc_test::new_client(format!("http://127.0.0.1:{}", test_port)).unwrap();

            // Checking that the key doesn't exist
            let res = client.do_get("/lookup/key1").await.unwrap();
            assert_eq!(res.status(), 204);

            // Wait to get notified by `client_one`
            let _ = rx.await;
            let res = client.do_get("/lookup/key1").await.unwrap();
            assert_eq!(res.status(), 200);
            assert_eq!(res.text_body().unwrap(), "value1");
        });
    });

    client_one_thread.join().unwrap();
    client_two_thread.join().unwrap();
    // intentionally not joining _server_thread
}
