# Jet Web Framework

Jet is a sleek, performant web framework for Rust, harnessing the power of the `hyper` library. It's designed for developers who seek speed without sacrificing simplicity, offering a streamlined approach to web development in Rust.

## 🌟 Features

- **Fast**: Built on top of `hyper`, one of the Rust's fastest web libraries.
- **Explicit Routing**: Define routes explicitly with a clear and intuitive API reminding you Laravel.
- **Safety First**: Benefit from Rust's strong safety guarantees.
- **Minimalistic Design**: No bloat, just the essentials.

## 🚀 Quickstart

To-do

## 📘 Usage

```rust
pub fn initialize_routes() {
    ROUTER.get("/", Box::new(index));
    ROUTER.get("/hello", Box::new(greet));
}
```

## 🤝 Contributing

To-do

