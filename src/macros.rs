#[macro_export]
macro_rules! declare_personnel_models {
    ($id_type:ty) => {
        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct PersonnelClass {
            pub id: $id_type,
            pub name: String,
            pub properties: Vec<PersonnelClassProperty>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct PersonnelClassProperty {
            pub id: $id_type,
            pub name: String,
            pub nested_properties: Vec<PersonnelClassProperty>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct Person {
            pub id: $id_type,
            pub name: String,
            pub personnel_classes: Vec<$id_type>,
            pub properties: Vec<PersonProperty>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct PersonProperty {
            pub id: $id_type,
            pub name: String,
            pub maps_to_personnel_class_property_id: Option<$id_type>,
            pub nested_properties: Vec<PersonProperty>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct QualificationTestSpecification {
            pub id: $id_type,
            pub name: String,
            pub person_ids: Vec<$id_type>,
            pub personnel_class_ids: Vec<$id_type>,
            pub person_property_ids: Vec<$id_type>,
            pub personnel_class_property_ids: Vec<$id_type>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct QualificationTestResult {
            pub id: $id_type,
            pub qualification_test_specification_id: $id_type,
            pub person_property_ids: Vec<$id_type>,
        }
    };
}

#[macro_export]
macro_rules! declare_equipment_models {
    ($id_type:ty) => {
        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct EquipmentClass {
            pub id: $id_type,
            pub name: String,
            pub properties: Vec<EquipmentClassProperty>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct EquipmentClassProperty {
            pub id: $id_type,
            pub name: String,
            pub nested_properties: Vec<EquipmentClassProperty>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct Equipment {
            pub id: $id_type,
            pub name: String,
            pub equipment_classes: Vec<$id_type>,
            pub properties: Vec<EquipmentProperty>,
            pub sub_equipment: Vec<Equipment>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct EquipmentProperty {
            pub id: $id_type,
            pub name: String,
            pub maps_to_equipment_class_property_id: Option<$id_type>,
            pub nested_properties: Vec<EquipmentProperty>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct EquipmentCapabilityTestSpecification {
            pub id: $id_type,
            pub name: String,
            pub equipment_ids: Vec<$id_type>,
            pub equipment_class_ids: Vec<$id_type>,
            pub equipment_property_ids: Vec<$id_type>,
            pub equipment_class_property_ids: Vec<$id_type>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct EquipmentCapabilityTestResult {
            pub id: $id_type,
            pub equipment_capability_test_specification_id: $id_type,
            pub equipment_property_ids: Vec<$id_type>,
        }
    };
}

#[macro_export]
macro_rules! declare_physical_asset_models {
    ($id_type:ty) => {
        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct PhysicalAssetClass {
            pub id: $id_type,
            pub name: String,
            pub properties: Vec<PhysicalAssetClassProperty>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct PhysicalAssetClassProperty {
            pub id: $id_type,
            pub name: String,
            pub nested_properties: Vec<PhysicalAssetClassProperty>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct PhysicalAsset {
            pub id: $id_type,
            pub name: String,
            pub physical_asset_class_id: $id_type,
            pub properties: Vec<PhysicalAssetProperty>,
            pub sub_assets: Vec<PhysicalAsset>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct PhysicalAssetProperty {
            pub id: $id_type,
            pub name: String,
            pub maps_to_physical_asset_class_property_id: Option<$id_type>,
            pub nested_properties: Vec<PhysicalAssetProperty>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct PhysicalAssetCapabilityTestSpecification {
            pub id: $id_type,
            pub name: String,
            pub physical_asset_ids: Vec<$id_type>,
            pub physical_asset_class_ids: Vec<$id_type>,
            pub physical_asset_property_ids: Vec<$id_type>,
            pub physical_asset_class_property_ids: Vec<$id_type>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct PhysicalAssetCapabilityTestResult {
            pub id: $id_type,
            pub physical_asset_capability_test_specification_id: $id_type,
            pub physical_asset_property_ids: Vec<$id_type>,
        }
    };
}

#[macro_export]
macro_rules! declare_equipment_hierarchy_models {
    ($id_type:ty) => {
        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum WorkCenterType {
            ProcessCell,
            ProductionLine,
            ProductionUnit,
            StorageZone,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum WorkUnitType {
            Unit,
            WorkCell,
            StorageUnit,
            ProductionEquipment,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct EnterpriseProperty {
            pub id: $id_type,
            pub name: String,
            pub nested_properties: Vec<EnterpriseProperty>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct Enterprise {
            pub id: $id_type,
            pub name: String,
            pub properties: Vec<EnterpriseProperty>,
            pub sites: Vec<Site>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct SiteProperty {
            pub id: $id_type,
            pub name: String,
            pub nested_properties: Vec<SiteProperty>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct Site {
            pub id: $id_type,
            pub name: String,
            pub properties: Vec<SiteProperty>,
            pub areas: Vec<Area>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct AreaProperty {
            pub id: $id_type,
            pub name: String,
            pub nested_properties: Vec<AreaProperty>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct Area {
            pub id: $id_type,
            pub name: String,
            pub properties: Vec<AreaProperty>,
            pub work_centers: Vec<WorkCenter>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct WorkCenterProperty {
            pub id: $id_type,
            pub name: String,
            pub maps_to_equipment_class_property_id: Option<$id_type>,
            pub nested_properties: Vec<WorkCenterProperty>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct WorkCenter {
            pub id: $id_type,
            pub name: String,
            pub work_center_type: WorkCenterType,
            pub equipment_classes: Vec<$id_type>, // IDs of EquipmentClass
            pub properties: Vec<WorkCenterProperty>,
            pub work_units: Vec<WorkUnit>,
            pub equipment_capability_test_specification_ids: Vec<$id_type>,
            pub equipment_capability_test_result_ids: Vec<$id_type>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct WorkUnitProperty {
            pub id: $id_type,
            pub name: String,
            pub maps_to_equipment_class_property_id: Option<$id_type>,
            pub nested_properties: Vec<WorkUnitProperty>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct WorkUnit {
            pub id: $id_type,
            pub name: String,
            pub work_unit_type: WorkUnitType,
            pub equipment_classes: Vec<$id_type>, // IDs of EquipmentClass
            pub properties: Vec<WorkUnitProperty>,
            pub equipment_ids: Vec<$id_type>, // IDs of Equipment
            pub equipment_capability_test_specification_ids: Vec<$id_type>,
            pub equipment_capability_test_result_ids: Vec<$id_type>,
        }
    };
}

#[macro_export]
macro_rules! declare_material_models {
    ($id_type:ty) => {
        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct MaterialClass {
            pub id: $id_type,
            pub name: String,
            pub properties: Vec<MaterialClassProperty>,
            pub assembled_from: Vec<$id_type>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct MaterialClassProperty {
            pub id: $id_type,
            pub name: String,
            pub nested_properties: Vec<MaterialClassProperty>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct MaterialDefinition {
            pub id: $id_type,
            pub name: String,
            pub material_classes: Vec<$id_type>,
            pub properties: Vec<MaterialDefinitionProperty>,
            pub assembled_from: Vec<$id_type>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct MaterialDefinitionProperty {
            pub id: $id_type,
            pub name: String,
            pub maps_to_material_class_property_id: Option<$id_type>,
            pub nested_properties: Vec<MaterialDefinitionProperty>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct MaterialLot {
            pub id: $id_type,
            pub name: String,
            pub material_definition_id: $id_type,
            pub properties: Vec<MaterialLotProperty>,
            pub assembled_from: Vec<$id_type>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct MaterialSublot {
            pub id: $id_type,
            pub name: String,
            pub material_lot_id: $id_type,
            pub properties: Vec<MaterialLotProperty>,
            pub sublots: Vec<MaterialSublot>,
            pub assembled_from: Vec<$id_type>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct MaterialLotProperty {
            pub id: $id_type,
            pub name: String,
            pub maps_to_material_definition_property_id: Option<$id_type>,
            pub nested_properties: Vec<MaterialLotProperty>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct MaterialTestSpecification {
            pub id: $id_type,
            pub name: String,
            pub material_class_ids: Vec<$id_type>,
            pub material_definition_ids: Vec<$id_type>,
            pub material_lot_ids: Vec<$id_type>,
            pub material_class_property_ids: Vec<$id_type>,
            pub material_definition_property_ids: Vec<$id_type>,
            pub material_lot_property_ids: Vec<$id_type>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct QATestResult {
            pub id: $id_type,
            pub name: String,
            pub material_test_specification_id: $id_type,
            pub material_lot_property_ids: Vec<$id_type>,
        }
    };
}
