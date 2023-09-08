use hyper::{Body, Request, Response, Method};
use hyper::service::Service;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use tokio::macros::support::Poll;

pub struct ServiceWithRouter {
    pub router: Arc<Router>,
}

impl Service<Request<Body>> for ServiceWithRouter {
    type Response = Response<Body>;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let router = self.router.clone(); 
        Box::pin(async move {
            router.route(req).await
        })
    }
}

pub trait Handler: Send + Sync {
    fn call(&self, req: Request<Body>) -> Pin<Box<dyn Future<Output = Response<Body>> + Send>>;
}

impl<F, Fut> Handler for F
where
    F: Fn(Request<Body>) -> Fut + Send + Sync,
    Fut: Future<Output = Response<Body>> + Send + 'static,
{
    fn call(&self, req: Request<Body>) -> Pin<Box<dyn Future<Output = Response<Body>> + Send>> {
        Box::pin(self(req))
    }
}

pub struct Route {
    method: Method,
    path: String,
    handler: Arc<dyn Handler>,
}

impl Route {
    pub fn get<H: Handler + 'static>(path: &str, handler: H) -> Self {
        let handler = Arc::new(handler);
        Route {
            method: Method::GET,
            path: path.to_string(),
            handler,
        }
    }
}

impl Clone for Route {
    fn clone(&self) -> Self {
        Route {
            method: self.method.clone(),
            path: self.path.clone(),
            handler: self.handler.clone(),
        }
    }
}

impl Clone for Router {
    fn clone(&self) -> Self {
        let routes = self.routes.lock().unwrap();
        Router {
            routes: Mutex::new(routes.clone()),
        }
    }
}

pub struct Router {
    routes: Mutex<HashMap<(Method, String), Arc<dyn Handler>>>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: Mutex::new(HashMap::new()),
        }
    }

    pub fn register(&mut self, route: Route) {
        let mut routes = self.routes.lock().unwrap();
        routes.insert((route.method, route.path), route.handler);
    }

    pub async fn route(&self, req: Request<Body>) -> Result<Response<Body>, Infallible> {
        let handler_option = {
            let routes = self.routes.lock().unwrap();
            routes.get(&(req.method().clone(), req.uri().path().to_string())).cloned()
        };
    
        match handler_option {
            Some(handler) => Ok(handler.call(req).await),
            None => Ok(Response::new(Body::from("Not Found")))
        }
    }
}

// The routes! macro
#[macro_export]
macro_rules! routes {
    ($($route_slice:expr),*) => {{
        let mut router = Router::new();
        $(
            for route in $route_slice {
                router.register(route.clone());
            }
        )*
        router
    }};
}

#[macro_export]
macro_rules! route_collector {
    ($($module_name:ident),*) => {
        routes!(
            $(&$module_name::routes()[..]),*
        )
    };
}