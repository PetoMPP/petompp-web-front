use super::{localizable::Localizable, tk::TK};
use petompp_web_models::error::{
    AuthError, Error, QueryValidationError, ResourceDataValidationError, UserError,
    ValidationError,
};

impl Localizable for Error {
    fn localize(&self, locales: &super::store::LocalesStore) -> String {
        match self.to_owned() {
            Error::Auth(e) => match e {
                AuthError::MissingClaim(c) => locales.get(TK::E_Auth_MissingClaim(c)),
                AuthError::InvalidFormat(c) => locales.get(TK::E_Auth_InvalidFormat(c)),
                AuthError::TokenExpiredS(s) => locales.get(TK::E_Auth_TokenExpiredS(s as i32)),
                AuthError::JwtError(s) => locales.get(TK::E_Auth_JwtError(s)),
            },
            Error::Database(db) => locales.get(TK::E_Database(db)),
            Error::DatabaseConnection(dbc) => locales.get(TK::E_DatabaseConnection(dbc)),
            Error::User(ue) => match ue {
                UserError::NameTaken(u) => locales.get(TK::E_UserNameTaken(u)),
                UserError::NotFound(u) => locales.get(TK::E_UserNotFound(u)),
                UserError::InvalidCredentials => locales.get(TK::E_InvalidCredentials),
                UserError::NotConfirmed(u) => locales.get(TK::E_UserNotConfirmed(u)),
            },
            Error::Register(_) => "use localize with Requirements!".into(),
            Error::Validation(ve) => match ve {
                ValidationError::Country => locales.get(TK::E_Validation_Country),
                ValidationError::Query(qe) => match qe {
                    QueryValidationError::InvalidColumn(c) => {
                        locales.get(TK::E_Validation_Query_InvalidColumn(c))
                    }
                },
                ValidationError::ResourceData(re) => match re {
                    ResourceDataValidationError::KeyMismatch(k1, k2) => {
                        locales.get(TK::E_Validation_ResourceData_KeyMismatch(k1, k2))
                    }
                    ResourceDataValidationError::KeyMissing => {
                        locales.get(TK::E_Validation_ResourceData_KeyMissing)
                    }
                    ResourceDataValidationError::ValueMissing => {
                        locales.get(TK::E_Validation_ResourceData_ValueMissing)
                    }
                },
            },
            Error::Status(_, m) => m,
        }
    }
}
