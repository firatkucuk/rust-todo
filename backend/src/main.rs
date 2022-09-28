#[macro_use]
extern crate nickel;

use nickel::{HttpRouter, Nickel};

fn main() {
    let mut server = Nickel::new();

    server
        .get("/", middleware! { |_req|
            let index_content = include_bytes!("../static/index.html");
            format!("{}", String::from_utf8_lossy(index_content))
        })
        .post("/todo", middleware! { |_req|
            "Hello world!";
        });

    server.listen("127.0.0.1:6767").unwrap();
}
