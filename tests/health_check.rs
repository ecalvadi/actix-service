#[tokio::test]
async fn health_check_works() {
    // Organizar
    spawn_app();

    //Creamos un nuevo cliente para realizar el request a nuestra
    //api
    let client = reqwest::Client::new();

    // Realizar la petición
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert para comparar el status del header y el body
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Lanzar nuestra aplicación en barckground -de alguna manera-
fn spawn_app() {
    let server = actix_service::run().expect("Failed to bind address");

    let _ = tokio::spawn(server);
}
