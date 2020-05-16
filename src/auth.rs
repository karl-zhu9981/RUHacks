use std::sync::RwLock;
pub use crate::crypt::UserItem;
use sha2::{Sha512, Digest};
use bson::{Decoder, Bson};
use std::collections::BTreeMap;
use crate::crypt::UserAuthResponse;
use serde::Deserialize;

pub struct AuthDB{
    collection: mongodb::Collection,
}

impl AuthDB{
    pub async fn auth(&self, id: String,passwd: String) -> Result<UserItem,UserAuthResponse>{
        let id = base64::encode(Sha512::digest(id.as_ref()));
        if let Some(doc) = self.collection.find_one(doc!{"UserId" : id},None){
            let item = Decoder::new(Bson::Document(doc));
            Ok(UserItem::deserialize(item)?)
        }else{
            Err(UserAuthResponse::Error {code: 0,msg: "No such user".to_string()})
        }
    }
}
