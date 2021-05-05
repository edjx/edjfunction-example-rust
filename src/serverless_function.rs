
use edjlib::{EdjLogger, FetchResponse, HttpFetch, 
    HttpRequest, HttpResponse, StatusCode,Uri};
use std::str::FromStr;

//stipped down version until HTTP req is ready.
pub fn serverless(req: HttpRequest) -> HttpResponse {
    EdjLogger::info("New Req Framework");
    let req_uri = req.uri();
    EdjLogger::info(&req_uri.to_string());

    let query_url = req.query_param_by_name("url").unwrap();
    EdjLogger::info(query_url.as_str());
    let fetch_uri = Uri::from_str(query_url.as_str()).unwrap();

    let mut fetch_res: FetchResponse = HttpFetch::get(fetch_uri)
        .set_header("headerv".parse().unwrap(), "headernValue".parse().unwrap())
        .send()
        .unwrap();

    EdjLogger::info("Fetch Successful");

    let mut res = HttpResponse::from(fetch_res.body())
        .set_status(StatusCode::OK)
        .set_header("header_key1".parse().unwrap(), "header_Value1".parse().unwrap());

    let headers = res.headers_mut();
    //headers.insert(edjlib::CONTENT_TYPE, "text/plain".parse().unwrap());
   

    return res;
}