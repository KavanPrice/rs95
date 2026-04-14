#[macro_use]
pub mod macros;
pub mod core;

pub mod default_models {
    //! A version of the domain model using UUIDs as identifiers.
    //! You probably want to use these, unless you'd prefer to "bring your own identifier type."
    //!
    //! Take a look at the `declare` macros to see how to parameterise the models for some type of identifier.
    pub mod personnel {
        //! A version of the Personnel models using UUIDs as identifiers.
        //! You probably want to use these, unless you'd prefer to "bring your own identifier type."
        declare_personnel_models!(uuid::Uuid);
    }

    pub mod equipment {
        //! A version of the Equipment models using UUIDs as identifiers.
        //! You probably want to use these, unless you'd prefer to "bring your own identifier type."
        declare_equipment_models!(uuid::Uuid);
    }

    pub mod equipment_hierarchy {
        //! A version of the Equipment Hierarchy models using UUIDs as identifiers.
        //! You probably want to use these, unless you'd prefer to "bring your own identifier type."
        declare_equipment_hierarchy_models!(uuid::Uuid);
    }

    pub mod process_segment {
        //! A version of the Process Segment models using UUIDs as identifiers.
        //! You probably want to use these, unless you'd prefer to "bring your own identifier type."
        declare_process_segment_models!(uuid::Uuid);
    }

    pub mod physical_asset {
        //! A version of the Physical Asset models using UUIDs as identifiers.
        //! You probably want to use these, unless you'd prefer to "bring your own identifier type."
        declare_physical_asset_models!(uuid::Uuid);
    }

    pub mod material {
        //! A version of the Material models using UUIDs as identifiers.
        //! You probably want to use these, unless you'd prefer to "bring your own identifier type."
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
