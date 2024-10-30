# Arc type
Arc means `Atomic Reference Counted` is a thread-safe, reference-counted smart pointer provided by the standard library (std::sync::Arc). It allows multiple threads to share owndership of data by creating multiple references to the same value in memory (that's why it needs to be copied).
Unlike Rc (Reference Counted), which is for single-threaded scenarios, `Arc` is designed for safe use across multiple threads.

## Key Features of `Arc`
1. Reference Counting: `Arc` keeps track of how many references (owners) exist to the data it points to. When the last reference is dropped, the data is deallocated.
2. Thread Safety: `Arc` uses atomic operations to manage the refence count safely across threads. This adds a small overhead but is necessary to prevent data races.
3. Immutability: `Arc` only provides shared ownership of immutable data. For shared, mutable access, you need to combine it with other types like `Mutex` or `RwLock` to ensure safe, synchronized access.

## Basic Usage of `Arc`
To create an `Arc`, you can wrap your data in `Arc::new`, then clone the `Arc` to create new references. Each clone increments the reference count, and when each reference is dropped, the count decreases. When it reaches zero, the data is freed.

## Basic Usage of `Arc`
To create an `Arc`, you can wrap your data in `Arc::new`, then clone the `Arc` to create new references. Each clone increments the reference count, and when each reference is dropped, the count decreases. When it reaches zero, the data is freed.

### Observations
To modify an Arc you need to mutate it, you need to use `Arc` with `Mutex` or `RwLock`. That means that you don't need to use `mut` alongside with this kind of type.

### Shared Data
In Rust, when using `Tokio` or any asynchronous framework, the concept of **shared data between tasks** refers to situations where multiple asynchronous tasks need to access and possibly modify the same data concurrently. This is common in scenarios like counters, shared states, caches, or any kind of data that multiple parts of an application need to read from or write to at the same time.

To ensure safety in concurrent access, Rust has strict rules requiring that data shared across threads or sync tasks is both `Send` and `Sync`.
1. Shared Data Between Tasks
In async applications, tasks are lightweight, oftern similar to threads but more efficient. For example, imagine a server that receives multiple client requests and needs to:
- Update a shared counter that tracks the number of active users.
- Maintain a shared cache that many tasks can access to void duplicate calculations.

In these cases, **shared data** (like the counter or cache) needs to be accessible to multiple async tasks. This means:
- Tasks need to **share ownership** of this data safely.
- Tasks need a way to **coordinate access** to avoid issues like data races, where multiple tasks read and write to the same data concurrently.

2. Why `Send` and `Sync` Are Required for Shared Data in Async Tasks
The need for `Send` and `Sync` comes from Rust's guarantees about memory safety and concurrency, especially across threads or assynchronous boundaries.

`Send` and `Sync` Trais
- `Send`: This trait means that a type cn be safely transferred (moved) to another thread. If a type implements `Send`. Rust knows it's safe to move it between threads, preventing any data from being used incorrectly when switching between async tasks.
- `Sync`: This trait means that it's safe for multiple threads to reference the same data simultaneously (assuming any mutation is handled safely, e.g., with a `Mutex` or `RwLock`).

### Why They Matter in Async (Concurrent) Contexts
1. Safety Across Asynchronous Boundaries: When you create an async task in Tokio, it may execute on a different thread or in a different part of the event loop. For a task to be moved between threads or await points, any data it holds must implement `Send` to ensure it can be safely moved.

2. Shared Access in Multi-threaded Contexts: If you want data to be accessible by multiple tasks concurrently, it must be `Sync`. This ensures taht if multiple async tasks try to read or write to this data, it won't lead to undefined behavior. For example, `Arc<Mutex<T>>` is both `Send` and `Sync` when `T` itself is `Send` and `Sync`, making it a common way to safely share data across tasks.

3. Concurrency Safety: Without `Send` and `Sync`, ddata access could result in race conditions or memory safety issues because one task could be modifying data while another task reads it, potentially leading to undefined behavior. `Send` and `Sync` prevent this by enforcing compile-time checks on data sharing.

