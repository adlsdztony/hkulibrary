# HKULibrary

a user-friendly client for HKU Library

## Usage

```rust
use hkulibrary::{LibClient，BookTask};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LibClient::new();
    let task = BookTask::new("2023-06-29","08300930","129");
    client.login("username", "password")
        .await?
        .book(&task)
        .await?;
    Ok(())
}
```

Task has a `From` implementation for `(&str, &str, &str)`, so you can also do

```rust
use hkulibrary::LibClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LibClient::new();
    client.login("username", "password")
        .await?
        .book(&("2023-06-29","08300930","129").into())
        .await?;
    Ok(())
}
```

You can use fetch_state to get the state of a room

```rust
use hkulibrary::LibClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LibClient::new();
    let states = client.login("username", "password")
        .await?
        .fetch_state()
        .await?;
    states.iter().for_each(|state| println!("{:?}", state));
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
