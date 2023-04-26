use redis::Commands; // Import redis and the commands to interact with the server

fn main() -> redis::RedisResult<()> { // The -> operator specifies that this function will return a Redis result. The () inside RedisResult means it doesn't return a value, only if it was successful or failed
    let client = redis::Client::open("redis://127.0.0.1/")?; // Create a client to interact with a redis server at 127.0.0.1
    let mut con = client.get_connection()?; // Create a connection object with the client
    con.set("my_key", 42)?; // Use the connection to create and set a keypair called 'my_key' with a value of 42
    assert_eq!(con.get("my_key"), Ok(42)); // Use the connection to find if the 'my_key' value is 42 or not
    con.set("my_key", 20)?; // Set the 'my_key' value to 20
    assert_eq!(con.get("my_key"), Ok(42)); // The keypair now fails the check as it is no longer 42.
    Ok(())
}