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

To-do

## 📘 Usage

Unique concept right now is working.
Will be published soon.

```rust
use crate::app::{index, greet};
use integra::core::router::{Route, ROUTER};

pub fn initialize_routes() {
    Route::get("/", index);
    Route::get("/hello", greet);
}
```

## 🤝 Contributing

To-do

