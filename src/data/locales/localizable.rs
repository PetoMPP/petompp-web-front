use super::store::LocalesStore;

pub trait Localizable {
    fn localize(&self, locales: &LocalesStore) -> String;
}