#![feature(proc_macro_hygiene,decl_macro)]
#![feature(try_trait)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;

use std::io::Read;
use rocket::response::Body;

#[get("/")]
pub fn root() -> &'static str{
    "Hello World"
}

#[post("/auth")]
pub fn auth<R: Read>(b: Body<R>) -> AuthUserResponse{
    unimplemented!("Wire the Database")
}

pub mod crypt;

pub fn main(){
    rocket::ignite().mount("/",routes![root]).launch();
}