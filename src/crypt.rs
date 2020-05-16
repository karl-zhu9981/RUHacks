use std::io::Read;
use rocket::response::Body;
use std::option::NoneError;
use serde_json::Error;

pub struct UserItem{
    hash_addr: [u8;64],
    auth_salt: [u8;32],
    passwd: [u8;64],
    auth_part: [u8;32],
    blind_key: Box<[u8]>
}

#[derive(Serialize,Deserialize)]
pub struct UserAuth{
    userid: String,
    password: String
}

#[derive(Serialize,Deserialize)]
pub enum UserAuthResponse{
    Error{code: u32,msg: String},
    Success{auth_part: String,blinded_key: String},
    CreateUser{auth_part: String,response_identifier: String}
}

#[derive(Serialize,Deserialize)]
pub struct CreateUser{
    response_identifier: String,
    blinded_key: String
}

pub enum AuthError{
    BodyEmpty,
    JsonError(serde_json::Error),
    Unknown(String)
}

impl From<NoneError> for AuthError{
    fn from(_: NoneError) -> Self {
        Self::BodyEmpty
    }
}

impl From<serde_json::Error> for AuthError{
    fn from(v: Error) -> Self {
        Self::JsonError(v)
    }
}

impl<S: ToString> From<S> for AuthError{
    fn from(s: S) -> Self {
        Self::Unknown(s.to_string())
    }
}

impl UserAuth{
    pub fn read<R: Read>(b: Body<R>) -> Result<Self,AuthError>{
        let s = b.into_string()?;
        let ret = serde_json::from_str(&s)?;
        Ok(ret)
    }
}