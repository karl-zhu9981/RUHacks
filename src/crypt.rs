use std::io::Read;
use rocket::response::{Body, Responder};
use std::option::NoneError;
use rocket::Request;
use std::error::Error;

pub struct UserItem{
    hash_addr: [u8;64],
    auth_salt: [u8;32],
    passwd: [u8;64],
    auth_part: [u8;32],
    blind_key: Box<[u8]>
}

#[derive(Serialize,Deserialize,Debug)]
pub struct UserAuth{
    userid: String,
    password: String
}

impl UserAuth{
    pub fn read<R: Read>(b: Body<R>) -> Result<Self,AuthError>{
        let s = b.into_string()?;
        let ret = serde_json::from_str(&s)?;
        Ok(ret)
    }
}

#[derive(Serialize,Deserialize,Debug)]
pub enum UserAuthResponse{
    Error{code: u32,msg: String},
    Success{auth_part: String,blinded_key: String},
    CreateUser{auth_part: String,response_identifier: String}
}

impl From<Result<&'_ UserItem,UserAuthResponse>> for UserAuthResponse{
    fn from(r: Result<&UserItem, UserAuthResponse>) -> Self {
        match r{
            Ok(u) => UserAuthResponse::Success {
                auth_part: base64::encode(u.auth_part),
                blinded_key: base64::encode(&u.blind_key)
            },
            Err(e) => e
        }
    }
}

impl<'r> Responder<'r> for UserAuthResponse{
    fn respond_to(self, request: &Request<'r>) -> rocket::request::Result<'r> {
        let st = serde_json::to_string(&self)?;
        st.respond_to(request)
    }
}

impl<'r> Responder<'r> for Result<&'_ UserItem,UserAuthResponse>{
    fn respond_to(self, request: &Request<'r>) -> rocket::request::Result<'r> {
        let st = serde_json::to_string(&UserAuthResponse::from(self))?;
        st.respond_to(request)
    }
}

#[derive(Serialize,Deserialize,Debug)]
pub struct CreateUser{
    response_identifier: String,
    blinded_key: String
}

#[derive(Debug)]
pub enum AuthError{
    BodyEmpty,
    JsonError(serde_json::Error),
    Unknown(String)
}

impl CreateUser{
    pub fn read<R: Read>(b: Body<R>) -> Result<Self,AuthError>{
        let s = b.into_string()?;
        let ret = serde_json::from_str(&s)?;
        Ok(ret)
    }
}

impl Error for AuthError{}


impl From<NoneError> for AuthError{
    fn from(_: NoneError) -> Self {
        Self::BodyEmpty
    }
}

impl From<serde_json::Error> for AuthError{
    fn from(v: serde_json::Error) -> Self {
        Self::JsonError(v)
    }
}

impl<S: ToString> From<S> for AuthError{
    fn from(s: S) -> Self {
        Self::Unknown(s.to_string())
    }
}

