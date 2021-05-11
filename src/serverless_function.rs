
use edjlib::{EdjLogger, HttpRequest, HttpResponse, StatusCode};
use std::str::FromStr;

pub fn serverless(req: HttpRequest) -> HttpResponse {

    EdjLogger::info("Inside example function");

    let mut res = HttpResponse::from("Welcome to EDJX".to_string())
        .set_status(StatusCode::OK)
        .set_header("Server".parse().unwrap(), "EDJX".parse().unwrap());   

    return res;
}
