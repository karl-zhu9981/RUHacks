#![feature(proc_macro_hygiene,decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
pub fn root() -> &'static str{
    "Hello World"
}

pub fn main(){
    rocket::ignite().mount("/",routes![root]).launch();
}