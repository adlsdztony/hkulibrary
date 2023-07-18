# HKULibrary
a user-friendly client for HKU Library
## Usage
```rust
use hkulibrary::{LibClient，Task};

#[tokio::main]
async fn main() {
    let client = LibClient::new();
    let task = Task::new("2023-06-29","08300930","129");
    client.login("username", "password")
        .await.unwrap()
        .book(&task)
        .await.unwrap();
}
```
Task has a `From` implementation for `(&str, &str, &str)`, so you can also do
```rust
use hkulibrary::LibClient;

#[tokio::main]
async fn main() {
    let client = LibClient::new();
    client.login("username", "password")
        .await.unwrap()
        .book(&("2023-06-29","08300930","129").into())
        .await.unwrap();
}
```
or
```rust
use hkulibrary::LibClient;

async fn book() -> Result<(), Box<dyn std::error::Error>> {
    let client = LibClient::new();
    client.login("username", "password")
        .await?
        .book(&("2023-06-29","08300930","129").into())
        .await?;
    Ok(())
}
```
## TODO
- [ ] Facilities
  - [x] Discussion Room
  - [ ] Study Room
  - [ ] Single Study Room
- [ ] Functions
  - [x] Book
  - [ ] Cancel
  - [x] Get Booked