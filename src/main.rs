use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move { // This is a green thread. (what is this?). Is a type of thread
            // managed by the programming language instead of the operating system.
            // They simulate multiple "threads" to run taks, working in a single operating system
            // thread. This allow concurrency without relying on the OS's native trhead scheduling
            // and management. Read GreenThreads.md for more info.
            
            // The async block returns a JoinHandle, that can be used to interact with the spawned
            // task. If the async function returns a value. It can be retrieved using .await on the
            // JoinHandle response. If an error is encounters an error during execution, the
            // JoinHandle will return an Err.
            
            // A new task is spawned for each inbound socket. The socket is moved to the new task
            // and processed there.
            // What means to move it? It works for any closure. When you use move, the ownership is
            // given to the closure, where the outer scope loses it, even the value will no longer
            // available in the outer scope.
            // The values used in the closure are moved, all of them.
            // The Task can me moved between threads, they can use the same thread where they got
            // called. All depends on the runtime and the processes that are being executed.

            // Tasks in TOkio are very lightweight. Under the good, they require only a single
            // allocation and 64 bytes of memory. Applications should feel free to spawn thousands,
            // if not millions of tasks.

            // 'static bound
            // When you sppawn a task on the Tokio runtime, its type's lifetime must be 'static.
            // This means that the spawned task must not contain any references to data owned
            // outside the task. That is why the values are moved. 'static they live almost
            // forever, meanwhile they are used. The reason to use move, is because if a value is
            // borrowed inside the block, that value has to out live the block. Having a 'static
            // type too, or just move it.

            // Send bound
            // Tasks spawned by `tokio::spawn` must implement Send. This allows the Tokio runtime
            // to move the tasks between threads while they are suspended at an .await.
            // More info: https://tokio.rs/tokio/tutorial/spawning
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    // The `Connection` lets us read/write redis **frames** instead of byte streams. The
    // `Connection` type is defined by mini-redis.
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);
        // example output: GOT: Array([Bulk(b"set"), Bulk(b"hello"), Bulk(b"world")])

        // Respond with an error
        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}

