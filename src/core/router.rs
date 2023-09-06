use hyper::{Body, Request, Response, Method};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use lazy_static::lazy_static;

// The Handler should return a Future
type AsyncHandler = Arc<dyn Fn(Request<Body>) -> Pin<Box<dyn Future<Output = Response<Body>> + Send + 'static>> + Send + Sync>;

pub struct Route {
    method: Method,
    path: String,
    handler: AsyncHandler,
}

impl Route {
    pub fn get(path: &str, handler_fn: impl Fn(Request<Body>) -> Pin<Box<dyn Future<Output = Response<Body>> + Send + 'static>> + Send + Sync + 'static) {
        let handler = Arc::new(move |req| handler_fn(req));
        let route = Route {
            method: Method::GET,
            path: path.to_string(),
            handler: handler,
        };
        ROUTER.register(route);
    }
    // You can add similar methods for POST, PUT, DELETE, etc.
}

pub struct Router {
    routes: Mutex<HashMap<(Method, String), AsyncHandler>>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: Mutex::new(HashMap::new()),
        }
    }

    pub fn register(&self, route: Route) {
        let mut routes = self.routes.lock().unwrap();
        routes.insert((route.method, route.path), route.handler);
    }

    pub async fn route(self: Arc<Self>, req: Request<Body>) -> Result<Response<Body>, Infallible> {
        let handler_option = {
            let routes = self.routes.lock().unwrap();
            routes.get(&(req.method().clone(), req.uri().path().to_string())).cloned()
        };
    
        match handler_option {
            Some(handler) => Ok(handler(req).await),
            None => Ok(Response::new(Body::from("Not Found")))
        }
    }
}

lazy_static! {
    pub static ref ROUTER: Arc<Router> = Arc::new(Router::new());
}
