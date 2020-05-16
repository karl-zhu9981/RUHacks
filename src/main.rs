#![feature(proc_macro_hygiene,decl_macro)]
#![feature(try_trait)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;

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

pub fn main(){
    rocket::ignite().mount("/",routes![root,auth]).launch();
}