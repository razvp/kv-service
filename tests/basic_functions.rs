#[path = "helpers.rs"]
mod helpers;

#[actix_web::test]
async fn remember_returns_200_for_good_request() {
    let test_port = helpers::spawn_test_server();
    let client = httpc_test::new_client(format!("http://127.0.0.1:{}", test_port)).unwrap();

    let res = client
        .do_post("/remember?key=key1", "value1")
        .await
        .unwrap();

    assert_eq!(res.status(), 201);
}

#[actix_web::test]
async fn remember_returns_400_for_request_with_empty_value() {
    let test_port = helpers::spawn_test_server();
    let client = httpc_test::new_client(format!("http://127.0.0.1:{}", test_port)).unwrap();

    let res = client.do_post("/remember?key=key1", "").await.unwrap();

    assert_eq!(res.status(), 400);
    assert_eq!(res.text_body().unwrap(), "EMPTY VALUE");
}

#[actix_web::test]
async fn remember_returns_400_for_request_with_empty_key() {
    let test_port = helpers::spawn_test_server();
    let client = httpc_test::new_client(format!("http://127.0.0.1:{}", test_port)).unwrap();

    let res = client.do_post("/remember?key=", "value1").await.unwrap();

    assert_eq!(res.status(), 400);
    assert_eq!(res.text_body().unwrap(), "EMPTY KEY");
}

#[actix_web::test]
async fn lookup_returns_204_for_inexistent_key() {
    let test_port = helpers::spawn_test_server();
    let client = httpc_test::new_client(format!("http://127.0.0.1:{}", test_port)).unwrap();

    let res = client.do_get("/lookup/key1").await.unwrap();

    assert_eq!(res.status(), 204);
}

#[actix_web::test]
async fn lookup_returns_200_and_correct_value() {
    let test_port = helpers::spawn_test_server();
    let client = httpc_test::new_client(format!("http://127.0.0.1:{}", test_port)).unwrap();

    let res = client
        .do_post("/remember?key=key1", "value1")
        .await
        .unwrap();
    assert_eq!(res.status(), 201);

    let res = client.do_get("/lookup/key1").await.unwrap();
    assert_eq!(res.status(), 200);
    assert_eq!(res.text_body().unwrap(), "value1");
}
