use mini_redis::{client, Result};

// Example of what gets transformed
// #[tokio::main]
// async fn main() {
//  println!("hello");
// }
//
// gets transformed:
// fn main() {
//  let mut rt = tokio::runtime::Runtime::new().unwrap();
//  rt.block_on(async {
//    println!("hello");
//  })
// }
#[tokio::main] // run a normal main function and starts the async runtime
async fn main() -> Result<()> {
    //  Open a connection to the mini-redis address.
    //  default port?
    //  await in rust is lazy, at compilation, the control is getting back to the thread when is
    //  needed.
    //  Calling await executes the body of an async function. If you call the function and dont use
    //  await, the body doesnt execute. It returns a Future
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    // into() converts some value to a defined interface? defined by the method in this case?
    // the question mark `?` shortcircuit the function call. If it returns a Result, the program
    // panics with the error, if implements Option, just return None immediately
    client.set("hello", "world".into()).await?; // for now is crashing with Error: unimplemented

    // Get key "hello"
    let result = client.get("hello").await?;
    println!("got value from the server; result={:?}", result);

    Ok(()) // Returns a Result Tuple
}
