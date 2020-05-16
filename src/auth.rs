use std::sync::RwLock;
pub use crate::crypt::UserItem;
use sha2::{Sha512, Digest};
use std::collections::BTreeMap;
use crate::crypt::UserAuthResponse;
use serde::Deserialize;
use sqlite::State;

pub struct AuthDB{
    db: sqlite::Connection
}

impl AuthDB{
    pub fn auth(&self, id: String,mut passwd: String) -> Result<UserItem,UserAuthResponse>{
        let id = base64::encode(Sha512::digest(id.as_ref()));
        let mut stat = self.db.prepare("SELECT * FROM Users WHERE UserId=?")?;
        stat.bind(0,id.as_str())?;
        if let State::Row = stat.next()?{
            let item = UserItem{
                hash_addr: stat.read(0)?,
                auth_salt: stat.read(1)?,
                passwd: stat.read(2)?,
                auth_part: stat.read(3)?,
                blind_key: stat.read(4)?
            };
            passwd += &item.auth_salt;
            let check = base64::encode(Sha512::digest(passwd.as_ref()));
            if item.passwd == check{
                Err(UserAuthResponse::Error {code: 1,msg: "Password Check Failed".to_string()})?
            }
            Ok(item)
        }else{
            Err(UserAuthResponse::Error {code: 0,msg: "User not found".to_string()})
        }
    }
}
