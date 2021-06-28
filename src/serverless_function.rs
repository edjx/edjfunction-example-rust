use edjx::{info, HttpRequest, HttpResponse, StatusCode};

pub fn serverless(_req: HttpRequest) -> HttpResponse {

    info!("Inside example function");

    HttpResponse::from("Welcome to EDJX".to_string())
        .set_status(StatusCode::OK)
        .set_header("Server".parse().unwrap(), "EDJX".parse().unwrap())
}
