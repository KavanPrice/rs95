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

    mod _process_segment_and_operations {
        //! All process segment and operations types generated together so that
        //! `OperationsSegment` shares the same segment specification types as `ProcessSegment`.
        declare_process_segment_and_operations_models!(uuid::Uuid);
    }

    pub mod process_segment {
        //! Process segment models with UUID identifiers.
        pub use super::_process_segment_and_operations::{
            ResourceUse,
            MaterialUse,
            ProcessSegmentDependencyType,
            EquipmentSegmentSpecificationProperty,
            EquipmentSegmentSpecification,
            PersonnelSegmentSpecificationProperty,
            PersonnelSegmentSpecification,
            MaterialSegmentSpecificationProperty,
            MaterialSegmentSpecification,
            PhysicalAssetSegmentSpecificationProperty,
            PhysicalAssetSegmentSpecification,
            ProcessSegmentParameter,
            ProcessSegmentDependency,
            ProcessSegment,
        };
    }

    pub mod operations {
        //! Operations models with UUID identifiers.
        pub use super::_process_segment_and_operations::{
            OperationType,
            JobOrderCommandType,
            JobOrderStatus,
            OperationsResponseResult,
            WorkMasterParameter,
            WorkMaster,
            OperationsDefinitionProperty,
            OperationsSegmentParameter,
            OperationsSegmentDependency,
            OperationsSegment,
            OperationsDefinition,
            SegmentRequirementParameter,
            SegmentRequirement,
            OperationsRequestProperty,
            OperationsRequest,
            OperationsSchedule,
            JobOrderParameter,
            JobOrder,
            JobResponseParameter,
            JobResponse,
            SegmentResponse,
            OperationsResponse,
            OperationsPerformance,
            OperationsCapabilityProperty,
            OperationsCapabilityElement,
            OperationsCapability,
        };
    }

    pub mod operational_location {
        //! A version of the Operational Location models using UUIDs as identifiers.
        //! You probably want to use these, unless you'd prefer to "bring your own identifier type."
        declare_operational_location_models!(uuid::Uuid);
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
