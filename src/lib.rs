use std::{any::Any, collections::BTreeMap, marker::PhantomData, ops::{Deref, DerefMut, Index, IndexMut}, rc::Rc, sync::Arc};
use bevy::prelude::Resource;
use map::InsertMap;
use xxhash_rust::xxh64::xxh64;
use colored::Colorize;

mod key;
mod entry;
mod map;
mod ext;

pub use key::{GlobalKey, LocalKey};
pub use entry::Entry;

pub mod prelude {
    pub use super::key::{GlobalKey, LocalKey};
    pub use super::entry::Entry;
    pub use super::Registry;
    pub use super::ext::AppRegistryExt;
}

const HASH_SEED: u64 = 123456789123456789;

/// An insert-only container for storing and accessing a type.
/// When an entry is added to a Registry, a `GlobalKey` and a `LocalKey` are created for it.
/// A `GlobalKey` is a hash of the entry's unique identifier. 
/// A `LocalKey` is an index of an entry in the Registry.
/// 
/// Insert into the Registry with .add().
/// ```
/// fn example() -> Registry<i32> {
///     let mut reg = Registry::new();
///     reg.add("item:0", 0);
///     reg.add("item:1", 1);
///     reg.add("item:2", 2);
///     reg.add("item:3", 3);
/// }
/// ```
/// 
/// Index a registry with a LocalKey
/// ```
/// fn index(reg: &Registry<i32>, key: LocalKey<i32>) -> &Entry<i32> {
///     reg[key]
/// }
/// ```
/// 
/// Search a Registry with a GlobalKey
/// ```
/// fn search(reg: &Registry<i32>, key: GlobalKey<i32>) -> Option<&Entry<i32>> {
///     reg.search(key)
/// }
/// ```
#[derive(Resource)]
pub struct Registry<I: Any> {
    /// An Insert-only Vector for storage.
    items: Vec<Entry<I>>,
    /// Lookup table for GlobalKey hashes.
    table: InsertMap,
}

impl<I: Any> Registry<I> {
    /// Construct a new Registry with the default capacity and hash seed. 
    pub fn new() -> Self {
        Self {
            items: Vec::with_capacity(64),
            table: InsertMap { entries: Vec::with_capacity(64) },
        }
    }

    /// Configure the capacity and hash seed. 
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
            table: InsertMap { entries: Vec::with_capacity(capacity) },
        }
    }

    /// Reserve space for more entries. 
    pub fn reserve(&mut self, amount: usize) {
        self.items.reserve(amount);
    }

    /// Insert a new entry into the Registry.
    pub fn add(&mut self, ident: &str, item: I) -> &Entry<I> {
        // registries must not be greater than 65535
        // because the local key is a u16 index.
        if self.items.len() >= u16::MAX as usize {
            let header = "!! FATAL REGISTRY ERROR !!".red();
            let s = ">".red();
            let ident = ident.magenta();
            let _type = std::any::type_name::<I>().magenta();
            panic!("
# {header}
# {s} Attempted to insert an item '{ident}' into registry of type '{_type}'.
# {s} However, the entries buffer in the registry is full. (len=65535)
# {s} This limitation exists because LocalKeys are an index in the buffer and are 16 bits.
# {s} If you need more entries, you will need to use another library. 
            ")
        }

        // get the hash value and index. 
        let global = xxh64(ident.as_bytes(), HASH_SEED);
        let local = self.items.len() as u16;

        // Insert the global key into its table.
        if let Some(collision) = self.table.insert(global, local) {
            // if insertion into the hash table fails, there
            // has either been a collision (very unlikely), or
            // the same value was inserted twice. 
            let _type = std::any::type_name::<I>().magenta();
            let other = &*self.items[collision as usize].ident().magenta();
            let ident = ident.magenta();
            let header = "!! FATAL REGISTRY ERROR !!".red();
            let fixes = "> Possible Fixes".cyan();
            let s = ">".red();
            panic!("
# {header}
# {s} Attempted to insert an item '{ident}' into registry of type '{_type}'.
# {s} However, another item with the ident '{other}' has the same hash.
# {s} Registries require that every entry have a unique identifier. 
# {s} This error only occurs under two conditions:
#    1. Two identifiers hash to the same u64 (collision).
#    2. Two entries have the same identifier (duplication).
# {fixes}:
#    1. Validate your code to ensure there is no duplication.
#    2. Namespace your identifiers. (e.g. 'my_ns:ident').
#    3. Change the name of one of the entries. 
            ");
        }

        // insert the item into the Vec. 
        self.items.push(
            Entry {
                ident: Arc::from(ident),
                local: LocalKey { index: local, marker: PhantomData },
                global: GlobalKey { hash: global, marker: PhantomData },
                item: item
            }
        );

        return &self.items[local as usize]
    }

    /// Search the for an Entry by its GlobalKey. 
    pub fn search(&self, key: GlobalKey<I>) -> Option<&Entry<I>> {
        self.table.get(key.hash).map(|idx| &self.items[idx as usize])
    }

    /// Search for an Entry by its GlobalKey, returning a mutable reference.
    pub fn search_mut(&mut self, key: GlobalKey<I>) -> Option<&mut Entry<I>> {
        self.table.get(key.hash).map(|idx| &mut self.items[idx as usize])
    }

    /// Iterate immutably over the entries in the Registry.
    pub fn iter(&self) -> std::slice::Iter<'_, Entry<I>> {
        self.items.iter()
    }

    /// Iterate mutably over the entries in the Registry. 
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Entry<I>> {
        self.items.iter_mut()
    }
}

impl<I: Any> Index<LocalKey<I>> for Registry<I> {
    type Output = Entry<I>;

    fn index(&self, key: LocalKey<I>) -> &Self::Output {
        &self.items[key.index as usize]
    }
}

impl<I: Any> IndexMut<LocalKey<I>> for Registry<I> {
    fn index_mut(&mut self, key: LocalKey<I>) -> &mut Self::Output {
        &mut self.items[key.index as usize]
    }
}

impl<'i, I: Any> IntoIterator for &'i Registry<I> {
    type IntoIter = std::slice::Iter<'i, Entry<I>>;
    type Item = &'i Entry<I>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'i, I: Any> IntoIterator for &'i mut Registry<I> {
    type IntoIter = std::slice::IterMut<'i, Entry<I>>;
    type Item = &'i mut Entry<I>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

unsafe impl<I: Any> Send for Registry<I> {}
unsafe impl<I: Any> Sync for Registry<I> {}

#[cfg(test)]
mod tests {
    use crate::{GlobalKey, Registry};

    #[test]
    #[should_panic]
    fn duplication() {
        let mut reg = Registry::<i32>::new();
        reg.add("a", 0);
        reg.add("a", 1);
    }

    #[test]
    fn add_similar() {
        let mut reg = Registry::<i32>::new();
        reg.add("item:a", 0);
        reg.add("item:b", 1);
        reg.add("item:c", 2);
        reg.add("item:d", 3);
        reg.add("item:e", 4);
        reg.add("item:f", 5);
        reg.add("item:g", 6);
    }

    #[test]
    fn search_global() {
        let mut reg = Registry::<i32>::new();
        reg.add("item:a", 0);
        reg.add("item:b", 1);
        reg.add("item:c", 2);
        reg.add("item:d", 3);
        reg.add("item:e", 4);
        reg.add("item:f", 5);
        reg.add("item:g", 6);

        assert!(
            reg
                .search(GlobalKey::new("item:f"))
                .is_some_and(|en| en.ident() == "item:f")
        );
    }

    #[test]
    fn search_local() {
        let mut reg = Registry::<i32>::new();
        reg.add("item:a", 0);
        reg.add("item:b", 1);
        reg.add("item:c", 2);
        reg.add("item:d", 3);
        reg.add("item:e", 4);
        reg.add("item:f", 5);
        reg.add("item:g", 6);

        let global = GlobalKey::new("item:f");
        let local = reg.search(global).unwrap().local_key();

        assert!(reg[local].ident() == "item:f")
    }
}