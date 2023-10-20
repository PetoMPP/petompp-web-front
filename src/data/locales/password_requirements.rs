use super::{localizable::Localizable, tk::TK};
use petompp_web_models::models::password_requirements::PasswordRequirements;

impl Localizable for PasswordRequirements {
    fn localize(&self, locales: &super::store::LocalesStore) -> String {
        let mut requirements = vec![];
        if self.numbers {
            requirements.push(locales.get(TK::E_Validation_PasswordRequirement_ContainsNumber));
        }
        if self.uppercase {
            requirements.push(locales.get(TK::E_Validation_PasswordRequirement_ContainsUppercase));
        }
        if self.lowercase {
            requirements.push(locales.get(TK::E_Validation_PasswordRequirement_ContainsLowercase));
        }
        if self.special {
            requirements
                .push(locales.get(TK::E_Validation_PasswordRequirement_ContainsSpecialCharacter));
        }
        locales.get(TK::E_Validation_PasswordRequirement(
            self.min_length,
            self.passes_required,
            requirements.join(", "),
        ))
    }
}
