#[macro_use]
pub mod macros;
pub mod core;

/// A version of the domain model using UUIDs as identifiers.
/// You probably want to use these, unless you'd prefer to "bring your own identifier type."
///
/// Take a look at the `declare` macros to see how to parameterise the models for some type of identifier.
pub mod default_models {
    /// A version of the Personnel models using UUIDs as identifiers.
    /// You probably want to use these, unless you'd prefer to "bring your own identifier type."
    pub mod personnel {
        declare_personnel_models!(uuid::Uuid);
    }

    /// A version of the Equipment models using UUIDs as identifiers.
    /// You probably want to use these, unless you'd prefer to "bring your own identifier type."
    pub mod equipment {
        declare_equipment_models!(uuid::Uuid);
    }

    /// A version of the Physical Asset models using UUIDs as identifiers.
    /// You probably want to use these, unless you'd prefer to "bring your own identifier type."
    pub mod physical_asset {
        declare_physical_asset_models!(uuid::Uuid);
    }

    /// A version of the Material models using UUIDs as identifiers.
    /// You probably want to use these, unless you'd prefer to "bring your own identifier type."
    pub mod material {
        declare_material_models!(uuid::Uuid);
    }
}

#[cfg(test)]
mod tests {
    use super::default_models::personnel::Person;
    use uuid::Uuid;

    #[test]
    fn test_macro_generation() {
        let p = Person {
            id: Uuid::new_v4(),
            name: "John".to_string(),
            personnel_classes: vec![],
            properties: vec![],
        };
        assert_eq!(p.name, "John");
    }
}
