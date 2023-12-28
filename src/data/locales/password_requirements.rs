use super::{localizable::Localizable, tk::TK};
use petompp_web_models::models::{
    password_requirements::PasswordRequirements, requirement::Requirements,
};

impl Localizable for (&PasswordRequirements, &String) {
    fn localize(&self, locales: &super::store::LocalesStore) -> String {
        let (pr, password) = self;
        let password = password.as_str();
        let validation = match (password.is_empty(), pr.validate(&password)) {
            (true, _) | (_, Ok(_)) => None,
            (_, Err(e)) => Some(e),
        };
        match validation {
            Some(validation) => localize_errors(&validation, pr, locales),
            None => String::new(),
        }
    }
}

impl Localizable for (&PasswordRequirements, &Vec<&str>) {
    fn localize(&self, locales: &super::store::LocalesStore) -> String {
        let (pr, validation) = self;
        localize_errors(validation, pr, locales)
    }
}

fn localize_errors(
    errors: &Vec<&str>,
    pr: &PasswordRequirements,
    locales: &super::store::LocalesStore,
) -> String {
    let mut localized = vec![];
    if errors.contains(&TK::Password_MinLength(0).to_string().as_str()) {
        localized.push(locales.get(TK::Password_MinLength(pr.min_length)));
    }
    if errors.contains(&TK::Password_ContainsNumber.to_string().as_str()) {
        localized.push(locales.get(TK::Password_ContainsNumber));
    }
    if errors.contains(&TK::Password_ContainsUppercase.to_string().as_str()) {
        localized.push(locales.get(TK::Password_ContainsUppercase));
    }
    if errors.contains(&TK::Password_ContainsLowercase.to_string().as_str()) {
        localized.push(locales.get(TK::Password_ContainsLowercase));
    }
    if errors.contains(&TK::Password_ContainsSpecial.to_string().as_str()) {
        localized.push(locales.get(TK::Password_ContainsSpecial));
    }
    localized.join("\n")
}
