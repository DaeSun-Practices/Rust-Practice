use mini_redis::{client, Result};

#[tokio::main]  //이 attribute가 async를 가능하게 해준다.
async fn main() -> Result<()> { //async 하위 코드는 다 async를 사용한다.
    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;

    // Get key "hello"
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}

// 위 작업을 2번을 하려면 원래 thread가 2개가 있어야 한다. 그러나, 이 경우, thread 하나만으로도 2 operation이 가능하다.
// -> 이를 통해 thread-overhead를 줄였다.
