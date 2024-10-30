use bytes::Bytes;
use mini_redis::{Connection, Frame};
use tokio::net::TcpStream;

type Db = Arc<Mutex<HashMap<String, Bytes>>>;
// `Mutex` from std::sync is used because `Mutex` from tokio locks across calls to .await.
// A synchronous mutex will block the current thread when waiting to acquire the lock. This, in
// turn, will block other tasks from processing. However, switching to `tokio::sync::Mutex` usually
// does not help as the asynchronous mutex uses a synchronous mutex internally.
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening");

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap(); // The loop is stopped until a connection is accepted.
        let db = db.clone();

        tokio::spawn(async move {
            // This is a green thread. (what is this?). Is a type of thread
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
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};

    // Connection, provided by `mini-redis`, handles parsing frames from the socket.
    // It allows to read frames.
    let mut connection = Connection::new(socket);

    // Use `read_frame` to receive a command from the connection.
    while let Some(frame) = connection.read_frame().await.unwrap() {
        // remember that unwrap panics
        // if the function returns an error or None, depending if used with `Result` or `Option`
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                // Conditional assignment. If the assignment of an Option or Result is succesful,
                // the if passes, and the value of the result or option is available
                if let Some(value) = db.get(cmd.key()) {
                    // `Frame::Bulk` expects data to be of type `Bytes`. This type will be covered
                    // later in the tutorial. For now, `&Vec<u8>` is converted to `Bytes` using
                    // `into()`.
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("uninplemented {:?}", cmd),
        };

        // Write the response to the client
        connection.write_frame(&response).await.unwrap();
    }
}
