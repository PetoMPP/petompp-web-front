use yewdux::prelude::*;

#[derive(Default, PartialEq, Clone, Debug, Store)]
pub struct BlobStore(usize);

impl BlobStore {
    pub fn invalidate(&mut self) {
        self.0 += 1;
    }
}