
# Integra Web Framework

Integra is a sleek, performant web framework for Rust, harnessing the power of the `hyper` library.
Will become the first highly pefrormant backend framework with MVC structure!

![Crates.io](https://img.shields.io/crates/v/integra)

## 🌟 Features

- **Fast**: Built on top of `hyper`, one of the Rust's fastest web libraries.
- **Explicit Routing**: Define routes explicitly with a clear and intuitive router reminding you Laravel.
- **Safety First**: Benefit from Rust's strong safety guarantees.
- **MVC**: Integra is leading the way with MVC (Model-View-Controller) in Rust. Model support via Diesel ORM integration is under development.
- **Minimalistic Design**: No bloat, just the essentials.

## 🔥🔥🔥 Performance Benchmarks 🔥🔥🔥

The following benchmarks were conducted on an M2 Pro machine using a single CPU core. The test was performed using the `wrk` benchmarking tool on a basic project setup. This results were close when I used hello world with salvo.

```bash
wrk http://127.0.0.1:3029/hello -t12 -c400 -d30s
Running 30s test @ http://127.0.0.1:3029/hello
  12 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     3.80ms  593.67us   8.34ms   74.98%
    Req/Sec     8.71k     0.88k   11.93k    59.64%
  3122975 requests in 30.04s, 869.66MB read
  Socket errors: connect 0, read 232, write 0, timeout 0
Requests/sec: 103977.74
Transfer/sec:     28.95MB
```

Please note that performance can vary based on many factors, including the specific workload, the configuration of the server, and the client's network conditions.

## 🖥️ Test Environment

- **OS**: Mac OS Ventura 13.5.1
- **CPU**: M2 Pro
- **Model**: MacBook Pro M2 Pro (16" and 16GB)

## 🚀 Quickstart with Integra

1. Clone this repository:
```bash
git clone https://github.com/SapphoTech/integra-starter-template.git my_project_name
```

## 📘 Usage

To-do

## 🎯 ROADMAP 
  - [ ] **Modules**
    - [x] Define application's structure using modules
    - [x] Module encapsulation
    - [ ] Shared modules
  - [x] **Controllers**
    - [x] Route parameters
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
  - [x] **Views**
    - [x] Template engine integration
    - [x] Support for dynamic content rendering
    - [x] Layouts and partials

## 🤝 Contributing

To-do
