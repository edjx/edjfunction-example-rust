mod serverless_function;
use edjlib::HttpRequest;

#[no_mangle]
pub fn init() {
    let req = HttpRequest::from_client(true);
    let res =crate::serverless_function::serverless(req.unwrap());
    res.send();
}
