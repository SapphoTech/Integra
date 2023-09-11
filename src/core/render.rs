#[macro_export]
macro_rules! render {
    ($template:expr, $data:expr) => {{
        let tera = TERA.lock().unwrap();
        let context = ::tera::Context::from_serialize($data).unwrap();
        match tera.render($template, &context) {
            Ok(rendered) => ::hyper::Response::new(::hyper::Body::from(rendered)),
            Err(e) => {
                eprintln!("Template rendering error: {:?}", e);
                ::hyper::Response::new(::hyper::Body::from("Internal server error"))
            }
        }
    }};
}