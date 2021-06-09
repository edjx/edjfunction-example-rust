mod serverless_function;
use edjx::HttpRequest;

#[no_mangle]
pub fn init() {
    let req = HttpRequest::from_client(true);
    let res =crate::serverless_function::serverless(req.unwrap());
    let _ = res.send();
}
