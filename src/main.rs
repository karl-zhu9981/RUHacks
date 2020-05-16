#![feature(proc_macro_hygiene,decl_macro)]
#![feature(try_trait,specialization)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;
#[macro_use] extern crate bson;

use std::io::Read;
use rocket::response::Body;
use rocket::request::Form;

#[get("/")]
pub fn root() -> &'static str{
    "Hello World"
}

#[post("/auth", format="json", data="<input>")]
pub fn auth(input: Form<crypt::UserAuth>) -> crypt::UserAuthResponse{
    unimplemented!("Wire the Database")
}

pub mod crypt;
pub mod auth;

pub fn main(){
    rocket::ignite().mount("/",routes![root,auth]).launch();
}