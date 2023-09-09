
# Integra Web Framework

Integra is a sleek, performant web framework for Rust, harnessing the power of the `hyper` library.

![Crates.io](https://img.shields.io/crates/v/integra)
[GitHub repository](https://github.com/SapphoTech/Integra/)

## 🌟 Features

- **Fast**: Built on top of `hyper`, one of the Rust's fastest web libraries.
- **Explicit Routing**: Define routes explicitly with a clear and intuitive router reminding you Laravel.
- **Safety First**: Benefit from Rust's strong safety guarantees.
- **Minimalistic Design**: No bloat, just the essentials.

## 🚀 Quickstart with Integra

### 1. Create a New Project
Start by creating a new Rust project.
```bash
cargo new integra_project
cd integra_project
```

### 2. Add Dependencies
Open `Cargo.toml` and add the following lines under `[dependencies]`:
```toml
[dependencies]
integra = { version = "0.0.7" }
tokio = { version = "1", features = ["full"] }
hyper = "0.14"
```

### 3. Setup Your Server
Setup your server in `src/main.rs`
```rust
use hyper::{Server};
use std::net::SocketAddr;
use std::convert::Infallible;
use crate::web::get_all_routes;
use integra::core::router::{ServiceWithRouter};
use std::sync::Arc;
use hyper::service::make_service_fn;

mod routes;
mod web;
mod app;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let router = get_all_routes();
    let shared_router = Arc::new(router);

    let make_svc = make_service_fn(move |_conn| {
        let service = ServiceWithRouter { router: shared_router.clone() };
        async move { Ok::<_, Infallible>(service) }
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], 3011));
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
```
2. Include app_routes.rs in `src/routes/mod.rs`
```rust
pub mod app_routes;
```
3. Collect all routes in `src/web.rs`
```rust
use crate::routes::app_routes;
use integra::core::router::Router;
use integra::routes;
use integra::route_collector;

pub fn get_all_routes() -> Router {
    route_collector!(
        app_routes
    )
}

```

### 5. Define Your App
Define your app for app_routes in `src/app.rs`
```rust
use hyper::{Body, Request, Response}; 

pub async fn index(_req: Request<Body>) -> Response<Body> {
    Response::new(Body::from("Root path"))
}

pub async fn greet(_req: Request<Body>) -> Response<Body> {
    Response::new(Body::from("Hello!"))
}
```


## 📘 Usage

To-do

## 🎯 ROADMAP 
 
- **Routing system**
  - [ ] **Modules**
    - [x] Define application's structure using modules
    - [x] Module encapsulation
    - [ ] Shared modules
  - [x] **Controllers**
    - [ ] Route parameters
    - [x] Request, Response objects handling
  - [ ] **Supported requests**
    - [x] GET
    - [ ] POST
    - [ ] PUT
    - [ ] DELETE
    - [ ] HEAD
    - [ ] CONNECT
    - [ ] OPTIONS
    - [ ] TRACE
    - [ ] PATCH
  - [ ] **Middleware**
    - [ ] Middleware for request/response manipulation
    - [ ] Execution order
  - [ ] **Guards**
    - [ ] Route guards for authentication and authorization
  - [ ] **Interceptors**
    - [ ] Transforming responses
    - [ ] Handling request/response lifecycle
  - [ ] **Macro Attributes**
    - [ ] Custom macro attributes for extracting custom data from requests

- **Framework Architecture**
  - [ ] **Models**
    - [ ] Integration with Diesel ORM
    - [ ] Support for migrations
    - [ ] CRUD operations
    - [ ] Advanced querying and filtering
  - [ ] **Repositories**
    - [ ] Generic Repository pattern
    - [ ] User Repository
  - [ ] **Providers and Services**
    - [ ] Custom providers
    - [ ] Singleton services
  - [ ] **Views**
    - [ ] Template engine integration
    - [ ] Support for dynamic content rendering
    - [ ] Layouts and partials

## 🤝 Contributing

To-do
