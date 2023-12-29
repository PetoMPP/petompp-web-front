use super::tk::TK;
use petompp_web_models::models::country::Country;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use strum::IntoEnumIterator;
use wasm_bindgen::{prelude::Closure, JsCast};
use yewdux::prelude::*;

#[derive(PartialEq, Clone, Debug, Store, Serialize, Deserialize)]
#[store(storage = "local")]
pub struct LocalesStore {
    pub curr: Country,
    #[serde(skip_serializing, skip_deserializing)]
    pl: HashMap<String, String>,
    #[serde(skip_serializing, skip_deserializing)]
    en: HashMap<String, String>,
}

impl Default for LocalesStore {
    fn default() -> Self {
        Self {
            curr: Country::get_current(),
            pl: Default::default(),
            en: Default::default(),
        }
    }
}

impl LocalesStore {
    pub fn is_loaded(&self, country: Country) -> bool {
        match country {
            Country::Poland => !self.pl.is_empty(),
            Country::UnitedKingdom => !self.en.is_empty(),
        }
    }

    pub fn get(&self, key: TK) -> String {
        let val = match self.curr {
            Country::Poland => self.pl.get(&key.to_string()),
            Country::UnitedKingdom => self.en.get(&key.to_string()),
        }
        .cloned()
        .unwrap_or_default();
        match key {
            TK::ActivateUserQuestion(s)
            | TK::DeleteUserQuestion(s)
            | TK::Username_OnlyAlphanumericOrSelectedChars(s)
            | TK::Username_NameTaken(s)
            | TK::E_Auth_MissingClaim(s)
            | TK::E_Auth_InvalidFormat(s)
            | TK::E_Auth_JwtError(s)
            | TK::E_Database(s)
            | TK::E_DatabaseConnection(s)
            | TK::E_UserNameTaken(s)
            | TK::E_UserNotFound(s)
            | TK::E_UserNotConfirmed(s)
            | TK::E_Validation_Query_InvalidColumn(s) => val.replace("%{0}", &s),
            TK::Password_MinLength(s) | TK::E_Auth_TokenExpiredS(s) => {
                val.replace("%{0}", &s.to_string())
            }
            TK::Username_InvalidLength(min, max) => val
                .replace("%{0}", &min.to_string())
                .replace("%{1}", &max.to_string()),
            TK::E_Validation_ResourceData_KeyMismatch(exp, act) => {
                val.replace("%{0}", &exp).replace("%{1}", &act)
            }
            _ => val,
        }
    }

    pub fn load(&mut self, country: Country, data: HashMap<String, String>) {
        let data = data.iter().map(|(k, v)| (k.into(), v.clone())).collect();
        if let Err(diff) = Self::validate_data(&data) {
            gloo::console::warn!(&format!(
                "Invalid data for country {:?}:\nmissing: {:?}\nextra: {:?}",
                country, diff.missing, diff.extra
            ));
        }
        match country {
            Country::Poland => self.pl = data,
            Country::UnitedKingdom => self.en = data,
        };
    }

    pub fn add_lang_change_event_listener(dispatch: Dispatch<Self>) {
        let closure = Closure::wrap(Box::new(move || {
            dispatch.reduce_mut(|state| state.curr = Country::get_current())
        }) as Box<dyn FnMut()>);
        web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("languagechange", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    fn validate_data(data: &HashMap<String, String>) -> Result<(), DataDiff> {
        let tks = TK::iter().map(|tk| tk.to_string()).collect::<HashSet<_>>();
        let data_tks = data.keys().cloned().collect::<HashSet<_>>();
        let missing = tks.difference(&data_tks).cloned().collect::<Vec<_>>();
        let extra = data_tks.difference(&tks).cloned().collect::<Vec<_>>();
        match missing.is_empty() && extra.is_empty() {
            true => Ok(()),
            false => Err(DataDiff { missing, extra }),
        }
    }
}

struct DataDiff {
    missing: Vec<String>,
    extra: Vec<String>,
}
