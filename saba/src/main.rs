#![no_std]
#![no_main]

extern crate alloc;

use crate::alloc::string::ToString;
use net_wasabi::http::HttpClient;
use noli::prelude::*;

fn main() -> u64 {
    let client = HttpClient::new();
    match client.get("host.test".to_string(), 5500, "sand_box/rust_sandbox/test.html".to_string()) {
        Ok(res) => {
            if res.status_code() == 400 {
                print!("Bad Request error: {:#?}", res);
            } else {
                print!("response:\n{:#?}", res);
            }
        }
        Err(e) => {
            print!("error:\n{:#?}", e);
        }
    }
    0
}

entry_point!(main);