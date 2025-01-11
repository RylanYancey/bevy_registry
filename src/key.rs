use std::{fmt, hash::Hash, marker::PhantomData};
use serde::{de::Visitor, Deserialize, Serialize};
use xxhash_rust::xxh64::xxh64;
use super::HASH_SEED;

use crate::*;

/// A LocalKey is an index in a Registry.
/// It is not guaranteed to be the same across
/// versions, platforms, or runtimes. Its value
/// is based on the insertion order into the
/// Registry and therefore should only be used for 
/// referencing Registry entries locally. 
/// 
/// Lookup time is O(1) because it is an index.
/// 
/// LocalKeys are created when an entry is added.
pub struct LocalKey<I: Any> {
    pub(in super) index: u16,
    pub(in super) marker: PhantomData<I>,
}

impl<I: Any> Copy for LocalKey<I> {}
impl<I: Any> Clone for LocalKey<I> {
    fn clone(&self) -> Self {
        Self {
            index: self.index,
            marker: PhantomData,
        }
    }
}

impl<I: Any> PartialEq for LocalKey<I> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl<I: Any> Eq for LocalKey<I> {}

/// A GlobalKey is a hash of an entry identifier.
/// It is guaranteed to be the same across versions,
/// platforms, or runtimes. Thus, it is suitable for
/// serializing/deserializing references to registry 
/// entries or for sending entries over the network. 
/// Lookup time is O(log n) because it traverses a binary tree.
/// GlobalKeys are created from a string Identifier
/// or can be retrieved from an entry with `Entry::global_key()`.
/// 
/// ```
/// fn search(registry: Registry<i32>) {
///     if let Some(item) = registry.search(GlobalKey::new("my:ident")) {
///         println!("{}", item.ident());
///     }
/// }
/// ```
/// 
/// GlobalKeys will Serialize as a u64 and Deserialize from
/// either a u64 or hash a String into it. Because of this, the hash seed
/// must be a constant and is not configurable.
pub struct GlobalKey<I: Any> {
    pub(in super) hash: u64,
    pub(in super) marker: PhantomData<I>,
}

impl<I: Any> GlobalKey<I> {
    pub fn new(ident: &str) -> Self {
        Self {
            hash: xxh64(ident.as_bytes(), HASH_SEED),
            marker: PhantomData::<I>,
        }
    }
}

impl<'de, I: Any> Deserialize<'de> for GlobalKey<I> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> 
    {
        struct GlobalKeyVisitor;
        impl<'de> Visitor<'de> for GlobalKeyVisitor {
            type Value = u64;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "A u64 hash or a string identifier.")
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error, 
            {
                Ok(v)    
            }

            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
            where
                E: serde::de::Error, 
            {
                Ok(v as u64)    
            }

            fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
            where
                E: serde::de::Error, 
            {
                Ok(v as u64)    
            }

            fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
            where
                E: serde::de::Error, 
            {
                Ok(v as u64)    
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error, 
            {
                    Ok(xxh64(v.as_bytes(), HASH_SEED))
            }
        } 

        Ok(
            Self {
                hash: deserializer.deserialize_any(GlobalKeyVisitor)?,
                marker: PhantomData::<I>
            }
        )
    }
}

impl<I: Any> Serialize for GlobalKey<I> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer 
    {
        serializer.serialize_u64(self.hash)    
    }
}

impl<I: Any> Copy for GlobalKey<I> {}
impl<I: Any> Clone for GlobalKey<I> {
    fn clone(&self) -> Self {
        Self {
            hash: self.hash,
            marker: PhantomData,
        }
    }
}

impl<I: Any> Hash for GlobalKey<I> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash)
    }
}

impl<I: Any> PartialEq for GlobalKey<I> {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl<I: Any> Eq for GlobalKey<I> {}