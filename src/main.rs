// enable the await! macro, async support, and the new std::Futures api.
#![feature(await_macro, async_await, futures_api)]
// only needed if we want to manually write a method to go forward from 0.1 to 0.3 future,
// or manually implement a std future (it provides Pin and Unpin):
#![feature(pin)]
// only needed to manually implement a std future:
#![feature(arbitrary_self_types)]

#[macro_use]
extern crate tokio;
use tokio::await;
use tokio::prelude::*;

async fn hello_world() -> &'static str {
    "Hello World"
}

// https://github.com/jsdw/jsdw.me/blob/master/content/posts/rust-asyncawait-preview/src/main.rs

fn main() {
    println!("Hello, world!");
}
