
# Registry Data Structure for Bevy

This crate exports a `Registry<I>` type that stores elements with a `LocalKey<I>`, a `GlobalKey<I>`, and a string identifier. The Registry is insert-only, so no removals are allowed, but the item can be mutated once it has been inserted. Registries are Resources, and can be initialized with `App::init_registry::<T>()` using the `AppRegistryExt` trait exported in the prelude.

## GlobalKeys vs LocalKeys

GlobalKeys are a hash of an entry identifier, while LocalKeys are an index in the Registry. This means the value of the LocalKey is based on the insertion order of the entries and should only be used locally, not for sending over the network or for saving data to disk. GlobalKeys will be the same regardless of runtimes, versions, or platforms, so can be used to reference registries over the network or for saving registry references to disk. GlobalKeys `Serialize` to a number and can `Deserialize` from a String, a number, or a tuple struct containing a String or number (RON only). LocalKeys cannot be Serialized or Deserialized because they are invalidated when the Registry is dropped. 

## Usage

```rs
// import the prelude
use bevy_registry::prelude::{Registry, GlobalKey, LocalKey, AppRegistryExt};

struct SomeData(i32);

// initialize a registry
fn main() {
    App::new()
        .init_registry::<SomeData>();
        // alternative - default growable cap is 64
        // .insert_registry(Registry::<SomeData>::with_capacity(512))
        .add_systems(Startup, insert_entries)
        .add_systems(Update, access_entries)
        .run();
}

// add some entries
fn insert_entries(mut registry: ResMut<Registry<SomeData>>) {
    registry.add("item:0", SomeData(0));
    registry.add("item:1", SomeData(1));
}

fn access_entries(mut registry: Res<Registry<SomeData>>) {
    // search the registry with GlobalKeys
    let entry = registry.search(GlobalKey::new("item:1")).unwrap();
    println!("The entry is: {}", entry.ident());

    // Index the registry with local keys.
    let entry = registry[entry.local_key()];
    println!("The entry is: {}", entry.ident());
}
```
