use std::sync::RwLock;
use std::iter::Map;
pub use crate::crypt::UserItem;
use sha2::{Sha512, Digest};
use bson::{Decoder, Bson};

pub struct AuthDB{
    collection: mongodb::Collection,
    usercache: RwLock<Map<String,UserItem>>
}
