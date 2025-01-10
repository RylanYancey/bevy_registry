
use super::*;

/// Wrapper for an item in the Registry.
pub struct Entry<I: Any> {
    /// The string Identifier of the Item.
    pub(in super) ident: Arc<str>,
    /// The Index of this entry in its registry.
    pub(in super) local: LocalKey<I>,
    /// The Hash of the identifier.
    pub(in super) global: GlobalKey<I>,
    pub item: I,
}

impl<I: Any> Entry<I> {
    pub fn ident(&self) -> &str {
        &self.ident
    }

    pub fn local_key(&self) -> LocalKey<I> {
        self.local
    }

    pub fn global_key(&self) -> GlobalKey<I> {
        self.global
    }
}

impl<I: Any> Deref for Entry<I> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<I: Any> DerefMut for Entry<I> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.item
    }
}
