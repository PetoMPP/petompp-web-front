use super::{localizable::Localizable, tk::TK};
use petompp_web_models::models::{
    requirement::Requirements, username_requirements::UsernameRequirements,
};

impl Localizable for (&UsernameRequirements, &String) {
    fn localize(&self, locales: &super::store::LocalesStore) -> String {
        let (ur, name) = self;
        let name = name.as_str();
        let validation = match (name.is_empty(), ur.validate(&name)) {
            (true, _) | (_, Ok(_)) => None,
            (_, Err(e)) => Some(e),
        };
        match validation {
            Some(validation) => localize_errors(&validation, ur, locales),
            None => String::new(),
        }
    }
}

impl Localizable for (&UsernameRequirements, &Vec<&str>) {
    fn localize(&self, locales: &super::store::LocalesStore) -> String {
        let (ur, validation) = self;
        localize_errors(validation, ur, locales)
    }
}

fn localize_errors(
    errors: &Vec<&str>,
    ur: &UsernameRequirements,
    locales: &super::store::LocalesStore,
) -> String {
    let mut localized = vec![];
    if errors.contains(&TK::Username_InvalidLength(0, 0).to_string().as_str()) {
        localized.push(locales.get(TK::Username_InvalidLength(ur.min_length, ur.max_length)));
    }
    if errors.contains(
        &TK::Username_OnlyAlphanumericOrSelectedChars("".into())
            .to_string()
            .as_str(),
    ) {
        localized.push(locales.get(TK::Username_OnlyAlphanumericOrSelectedChars(
            ur.special_chars.clone(),
        )));
    }
    localized.join("\n")
}
