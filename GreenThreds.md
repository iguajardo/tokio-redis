# Green Threads
Green Threads a special "thread" created and managed by a programming language runtime. The are used to handle multiple tasks in a program concurrently in a single Operate System thread. They are virtual, the operate system don't know about them. They don't use the OS thread scheduling and management.

## Key Characteristics of Green Threads
- User-space management: Green threads are managed in user space, meaning that the laguange runtime or a library controls them, not the OS.
- Lightweight: They have lower memory and resource requirements compared to OS threads because they don't require as much overhead for scheduling, context switching, and memory allocation.
- Cooperative or Preemptive Scheduling: The runtime can choose either:
    - Cooperative scheduling, where threads yield control back to the scheduler manually, oftern at specific points in the code (e.g., when waiting for I/O).
    - Preemptive scheduling, where the runtime periodically interrupts green threads toswtich between them.
- Concurrency, not parallelism: Because green threads are typically executed within a single OS thread, they provice concurrency but may not utilize multiple CPU cores. They can run on multiple cores if the runtime uses multiple OS threads for scheduling, as in many asynchronous runtimes like `tokio` in Rust.

## Green Threads vs. OS Threads
Green threads differ from OS threads in that they aren't directly managed by the operating system. OS hreads are scheduled by the OS kernel and can un on multiple CPU cores, supporting parallel execution. In contrast, green threads are virtual and are multiplexed onto one or more OS threads by the runtime.

## Benefits of Green Threads
1. Lower Overhead: They're generally cheaper to create and manage compared to OS threads.
2. Efficient for I/O-bound tasks: Because green threads can yield control during I/O operations, they'are very efficient for I/O-bound applications, like network services.
3. Fine-grained control: They give the runtime full control over scheduling, allowing for optimizations specific to the application or environment.

## Limitations of Green Threads
1. No true parallelism: Since they usually run on a single OS thread, they cannot leverage multi-core CPUs for parallel computation. However, some modern runtimes (e.g., `tokio`) work around this by scheduling green threads across a pool of OS threads.
2. Blocking operations are problematic: If one green thread performs a blocking operation it can block the entire OS thread, stalling all other green threads on that OS thread.
