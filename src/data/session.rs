use petompp_web_models::models::user::UserData;
use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Default, PartialEq, Clone, Debug, Store, Serialize, Deserialize)]
#[store(storage = "session", storage_tab_sync)]
pub struct SessionStore {
    pub user: Option<UserData>,
    pub token: Option<String>,
}
