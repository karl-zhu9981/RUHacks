#![feature(proc_macro_hygiene,decl_macro)]
#![feature(try_trait)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;

#[get("/")]
pub fn root() -> &'static str{
    "Hello World"
}

pub mod crypt;

pub fn main(){
    rocket::ignite().mount("/",routes![root]).launch();
}