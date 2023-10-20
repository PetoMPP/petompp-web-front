use super::{localizable::Localizable, tk::TK};
use petompp_web_models::error::{
    AuthError, Error, QueryValidationError, ResourceDataValidationError, UsernameValidationError,
    ValidationError,
};

impl Localizable for Error {
    fn localize(&self, locales: &super::store::LocalesStore) -> String {
        match self.to_owned() {
            Error::AuthError(e) => match e {
                AuthError::MissingClaim(c) => locales.get(TK::E_Auth_MissingClaim(c)),
                AuthError::InvalidFormat(c) => locales.get(TK::E_Auth_InvalidFormat(c)),
                AuthError::TokenExpiredS(s) => locales.get(TK::E_Auth_TokenExpiredS(s as i32)),
                AuthError::JwtError(s) => locales.get(TK::E_Auth_JwtError(s)),
            },
            Error::DatabaseError(db) => locales.get(TK::E_Database(db)),
            Error::DatabaseConnectionError(dbc) => locales.get(TK::E_DatabaseConnection(dbc)),
            Error::UserNameTaken(u) => locales.get(TK::E_UserNameTaken(u)),
            Error::UserNotFound(u) => locales.get(TK::E_UserNotFound(u)),
            Error::InvalidCredentials => locales.get(TK::E_InvalidCredentials),
            Error::UserNotConfirmed(u) => locales.get(TK::E_UserNotConfirmed(u)),
            Error::ValidationError(ve) => match ve {
                ValidationError::Username(ue) => match ue {
                    UsernameValidationError::InvalidLength(min, max) => {
                        locales.get(TK::E_Validation_Username_InvalidLength(min, max))
                    }
                    UsernameValidationError::InvalidCharacters(chars) => {
                        locales.get(TK::E_Validation_Username_InvalidCharacters(
                            chars
                                .iter()
                                .map(|c| c.to_string())
                                .collect::<Vec<_>>()
                                .join(", "),
                        ))
                    }
                },
                ValidationError::Password(pr) => locales.get(TK::E_Validation_Password(pr)),
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
