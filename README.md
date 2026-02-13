# rs95

A Rust library implementing the **ISA-95** (Enterprise-Control System Integration) domain models for manufacturing
operations management.

This library provides a type-safe, flexible implementation of the ISA-95 object models for Personnel, Equipment,
Physical Assets, and Materials. Most references are implemented using identifiers to allow flexibility in calling
applications.

## Features and Details

The implementation is split across four modules for equipment, materials, personnel, and physical assets. These are all
available in `rs95::core`. All models are parameterised by an `ID` type, allowing you to use `Uuid`, `String`, `u64`, or
any custom type as identifiers; if you've got external identifiers from other systems, like resource URIs, you can bring
these in. You can use the declarative macros in `rs95::macros` to generate concrete, non-generic versions of the models
for your specific identifier type.

Optional `Serialize` and `Deserialize` support is available via the `serde` feature flag. This will derive `Serialize`
and `Deserialize` implementations for all models - useful for interacting with outside systems. You'll need to make sure
you do this for your own identifiers as well.

## Usage

### Using Default Models (UUID)

By default, the library provides a set of pre-generated models using `uuid::Uuid` in the `default_models` module.

```rust
use rs95::default_models::personnel::Person;
use uuid::Uuid;

let person = Person {
id: Uuid::new_v4(),
name: "John Doe".to_string(),
personnel_classes: vec![],
properties: vec![],
};
```

### Using Generic Models

If you want to use a custom ID type (e.g. a URL string), use the `core` module:

```rust
use rs95::core::equipment::Equipment;

let cnc_machine = Equipment::<String> {
id: "http://factory.com/assets/cnc-01".to_string(),
name: "CNC 01".to_string(),
equipment_classes: vec !["http://factory.com/classes/milling".to_string()],
properties: vec ! [],
sub_equipment: vec ! [],
};
```

### Generating Custom Modules

You can use the provided macros to generate a specialised module for your ID type, removing the need for generic
parameters in your application code:

```rust
pub mod my_models {
    use rs95::declare_physical_asset_models;
    declare_physical_asset_models!(String);
}

// Now use them directly
use my_models::PhysicalAsset;

let asset = PhysicalAsset {
id: "ID-123".to_string(),
name: "My Asset".to_string(),
physical_asset_class_id: "CLASS-A".to_string(),
properties: vec![],
sub_assets: vec![],
};
```

## Feature Flags

- `serde`: Enables `Serialize` and `Deserialize` derivations for all models. It also enables the `serde` feature for the
  `uuid` dependency.

```toml
[dependencies]
rs95 = { version = "0.1.0", features = ["serde"] }
```
