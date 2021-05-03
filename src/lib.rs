mod serverless_function;
use edjlib::HttpRequest;

//stipped down version until HTTP req res is ready.
#[no_mangle]
pub fn init() {
    let req = HttpRequest::from_client(true);
    let res =crate::serverless_function::serverless(req);
    res.send();
}