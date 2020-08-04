#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

// 	#[macro_use] extern crate rocket;

// 	#[cfg(test)] mod tests;

use std::sync::atomic::{AtomicUsize, Ordering};

use rocket::response::content;
use rocket::State;
use rocket_contrib::serve::StaticFiles;
struct HitCount(AtomicUsize);

#[get("/")]
fn index(hit_count: State<HitCount>) -> content::Html<String> {
    hit_count.0.fetch_add(1, Ordering::Relaxed);
    let msg = "Your visit has been recorded!";
    let count = format!("Visits: {}", count(hit_count));
    content::Html(format!("{}<br /><br />{}", msg, count))
}

#[get("/count")]
fn count(hit_count: State<HitCount>) -> String {
    hit_count.0.load(Ordering::Relaxed).to_string()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", StaticFiles::from("static"))
        .manage(HitCount(AtomicUsize::new(0)))
}

fn main() {
    rocket().launch();
}

//extern crate rocket_contrib;

// extern crate lazy_static;
// extern crate rocket_cors;
// extern crate serde_derive; //allow us to serializ and deserialize string to json
//                            //and jason to string format
// use rocket_cors::{
//     AllowedHeaders,
//     AllowedOrigins,

// }
// use std::collections::HashMap; //stors data in terms of key and value pair
// use std::sync::{Arc, Mutex};
// use rocket_contrib::{Json, Jsonvalue}; //to use jason format
// // The JSON format is often used for serializing and transmitting structured data
// // over a network connection. It is used primarily to transmit data between a
// // server and web application,
// use rocket::state;

// type ID = usize;

// #[derive(Debug)]
// struct Message {
//     id: ID,
//     contents: String,
// }

// fn make_cors()-> Cors{

// }

// fn main() {
//     println!("Hello, world!");
// }
