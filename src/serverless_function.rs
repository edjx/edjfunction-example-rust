use edjlib::{HttpRequest, HttpResponse, Version, StatusCode, 
    HeaderName,HeaderValue, HeaderMap};

//stipped down version until HTTP req is ready.
pub fn serverless(req: HttpRequest) -> HttpResponse {
//EdjLogger::info("Test Log Framework ");

let url = req.url();

let mut res = HttpResponse::from(url.query().unwrap())
              .set_version(Version::HTTP_11)
              .set_status(StatusCode::OK)
              .set_header("headerv".parse().unwrap(), "headernValue".parse().unwrap());

let headers = res.headers_mut();
// headers.insert(edjlib::CONTENT_TYPE, "text/plain".parse().unwrap());

return res;
}