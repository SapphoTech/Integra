
# Integra Web Framework

Integra is a sleek, performant web framework for Rust, harnessing the power of the `hyper` library.

![Crates.io](https://img.shields.io/crates/v/integra)
[GitHub repository](https://github.com/SapphoTech/Integra/)

## 🌟 Features

- **Fast**: Built on top of `hyper`, one of the Rust's fastest web libraries.
- **Explicit Routing**: Define routes explicitly with a clear and intuitive router reminding you Laravel.
- **Safety First**: Benefit from Rust's strong safety guarantees.
- **Minimalistic Design**: No bloat, just the essentials.

## 🚀 Quickstart

1. **Create a New Project**: Start by creating a new Rust project.
   ```toml
   cargo new integra_project
   cd integra_project
   ```

2. **Add Dependencies**: Open `Cargo.toml` and add the following lines under `[dependencies]`:
   ```toml
    [dependencies]
    integra = { version = "0.0.4" }
    tokio = { version = "1", features = ["full"] }
    hyper = "0.14"
   ```

3. **Setup Your Server**: In `src/main.rs`, you can use the following template to set up a basic server:
   ```rust
   use integra::core::router::ROUTER;
   use hyper::{Server, Body, Request, Response};
   use std::convert::Infallible;
   
   mod web;
   
   #[tokio::main]
   async fn main() {
       web::initialize_routes();
       
       let make_svc = make_service_fn(move |_conn| {
           async move { Ok::<_, Infallible>(service_fn(ROUTER.route)) }
       });
       
       let addr = ([127, 0, 0, 1], 3000).into();
       let server = Server::bind(&addr).serve(make_svc);

       println!("Server running on http://{}", addr);
       
       if let Err(e) = server.await {
           eprintln!("Server error: {}", e);
       }
   }
   ```

4. **Define Your Routes**: In the `web` module (usually a file named `web.rs` in the root), use Integra's routing system:
   ```rust
   use crate::app::{index, greet};
   use integra::core::router::Route;

   pub fn initialize_routes() {
       Route::get("/", index);
       Route::get("/hello", greet);
   }
   ```

4. **Define Your App**: In the `app` module (usually a file named `app.rs` in the root):
   ```rust
    use hyper::{Body, Request, Response};
    use std::pin::Pin;
    use std::future::Future;

    pub fn index(_req: Request<Body>) -> Pin<Box<dyn Future<Output = Response<Body>> + Send>> {
        Box::pin(async { Response::new(Body::from("Root path")) })
    }

    pub fn greet(_req: Request<Body>) -> Pin<Box<dyn Future<Output = Response<Body>> + Send>> {
        Box::pin(async { Response::new(Body::from("Hello!")) })
    }
   ```

## 📘 Usage

To-do

## 🤝 Contributing

To-do
