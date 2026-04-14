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
            pub equipment_classes: Vec<$id_type>,
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
            pub equipment_classes: Vec<$id_type>,
            pub properties: Vec<WorkUnitProperty>,
            pub equipment_ids: Vec<$id_type>,
            pub equipment_capability_test_specification_ids: Vec<$id_type>,
            pub equipment_capability_test_result_ids: Vec<$id_type>,
        }
    };
}

#[macro_export]
macro_rules! declare_operational_location_models {
    ($id_type:ty) => {
        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct OperationalLocationClass {
            pub id: $id_type,
            pub name: String,
            pub properties: Vec<OperationalLocationClassProperty>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct OperationalLocationClassProperty {
            pub id: $id_type,
            pub name: String,
            pub nested_properties: Vec<OperationalLocationClassProperty>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct OperationalLocation {
            pub id: $id_type,
            pub name: String,
            pub operational_location_classes: Vec<$id_type>,
            pub properties: Vec<OperationalLocationProperty>,
            pub sub_locations: Vec<OperationalLocation>,
            pub equipment_ids: Vec<$id_type>,
            pub personnel_ids: Vec<$id_type>,
            pub physical_asset_ids: Vec<$id_type>,
            pub material_lot_ids: Vec<$id_type>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct OperationalLocationProperty {
            pub id: $id_type,
            pub name: String,
            pub maps_to_operational_location_class_property_id: Option<$id_type>,
            pub nested_properties: Vec<OperationalLocationProperty>,
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

/// Generates concrete process segment and operations types for a given identifier type.
///
/// Both groups of types are generated in the same module so that `OperationsSegment` can
/// embed the same `EquipmentSegmentSpecification`, `MaterialSegmentSpecification`, etc. types
/// that `ProcessSegment` uses — no cross-module imports needed.
///
/// # Example
/// ```rust
/// mod my_models {
///     rs95::declare_process_segment_and_operations_models!(uuid::Uuid);
/// }
/// ```
#[macro_export]
macro_rules! declare_process_segment_and_operations_models {
    ($id_type:ty) => {
        // ── Process segment enumerations ──────────────────────────────────────

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum ResourceUse {
            Used,
            Any,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum MaterialUse {
            Consumed,
            Produced,
            ByProduct,
            CoProduct,
            Yield,
            Sample,
            Any,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum ProcessSegmentDependencyType {
            AtStart,
            AfterStart,
            AfterEnd,
            NotFollow,
            PossibleParallel,
            Parallel,
        }

        // ── Segment specifications (shared by ProcessSegment and OperationsSegment)

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct EquipmentSegmentSpecificationProperty {
            pub id: $id_type,
            pub name: String,
            pub nested_properties: Vec<EquipmentSegmentSpecificationProperty>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct EquipmentSegmentSpecification {
            pub id: $id_type,
            pub equipment_class_id: Option<$id_type>,
            pub equipment_id: Option<$id_type>,
            pub quantity: Option<String>,
            pub unit: Option<String>,
            pub use_type: ResourceUse,
            pub properties: Vec<EquipmentSegmentSpecificationProperty>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct PersonnelSegmentSpecificationProperty {
            pub id: $id_type,
            pub name: String,
            pub nested_properties: Vec<PersonnelSegmentSpecificationProperty>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct PersonnelSegmentSpecification {
            pub id: $id_type,
            pub personnel_class_id: Option<$id_type>,
            pub person_id: Option<$id_type>,
            pub quantity: Option<String>,
            pub unit: Option<String>,
            pub use_type: ResourceUse,
            pub properties: Vec<PersonnelSegmentSpecificationProperty>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct MaterialSegmentSpecificationProperty {
            pub id: $id_type,
            pub name: String,
            pub nested_properties: Vec<MaterialSegmentSpecificationProperty>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct MaterialSegmentSpecification {
            pub id: $id_type,
            pub material_class_id: Option<$id_type>,
            pub material_definition_id: Option<$id_type>,
            pub quantity: Option<String>,
            pub unit: Option<String>,
            pub use_type: MaterialUse,
            pub properties: Vec<MaterialSegmentSpecificationProperty>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct PhysicalAssetSegmentSpecificationProperty {
            pub id: $id_type,
            pub name: String,
            pub nested_properties: Vec<PhysicalAssetSegmentSpecificationProperty>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct PhysicalAssetSegmentSpecification {
            pub id: $id_type,
            pub physical_asset_class_id: Option<$id_type>,
            pub physical_asset_id: Option<$id_type>,
            pub quantity: Option<String>,
            pub unit: Option<String>,
            pub use_type: ResourceUse,
            pub properties: Vec<PhysicalAssetSegmentSpecificationProperty>,
        }

        // ── Process segment ───────────────────────────────────────────────────

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct ProcessSegmentParameter {
            pub id: $id_type,
            pub name: String,
            pub value: String,
            pub unit: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct ProcessSegmentDependency {
            pub id: $id_type,
            pub dependency_type: ProcessSegmentDependencyType,
            pub from_process_segment_id: $id_type,
            pub to_process_segment_id: $id_type,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct ProcessSegment {
            pub id: $id_type,
            pub name: String,
            pub duration: Option<String>,
            pub equipment_segment_specifications: Vec<EquipmentSegmentSpecification>,
            pub personnel_segment_specifications: Vec<PersonnelSegmentSpecification>,
            pub material_segment_specifications: Vec<MaterialSegmentSpecification>,
            pub physical_asset_segment_specifications: Vec<PhysicalAssetSegmentSpecification>,
            pub parameters: Vec<ProcessSegmentParameter>,
            pub sub_segments: Vec<ProcessSegment>,
            pub dependencies: Vec<ProcessSegmentDependency>,
        }

        // ── Operations enumerations ───────────────────────────────────────────

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum OperationType {
            Production,
            Maintenance,
            Quality,
            Inventory,
            Mixed,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum JobOrderCommandType {
            Start,
            Stop,
            Hold,
            Restart,
            Abort,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum JobOrderStatus {
            Waiting,
            Ready,
            Running,
            Completed,
            Aborted,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum OperationsResponseResult {
            Completed,
            PartCompleted,
            Aborted,
        }

        // ── Work master ───────────────────────────────────────────────────────

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct WorkMasterParameter {
            pub id: $id_type,
            pub name: String,
            pub value: String,
            pub unit: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct WorkMaster {
            pub id: $id_type,
            pub name: String,
            pub version: String,
            pub parameters: Vec<WorkMasterParameter>,
            pub assembled_from: Vec<$id_type>,
        }

        // ── Operations definition ─────────────────────────────────────────────

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct OperationsDefinitionProperty {
            pub id: $id_type,
            pub name: String,
            pub nested_properties: Vec<OperationsDefinitionProperty>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct OperationsSegmentParameter {
            pub id: $id_type,
            pub name: String,
            pub value: String,
            pub unit: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct OperationsSegmentDependency {
            pub id: $id_type,
            pub dependency_type: ProcessSegmentDependencyType,
            pub from_operations_segment_id: $id_type,
            pub to_operations_segment_id: $id_type,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct OperationsSegment {
            pub id: $id_type,
            pub name: String,
            pub process_segment_id: Option<$id_type>,
            pub equipment_segment_specifications: Vec<EquipmentSegmentSpecification>,
            pub personnel_segment_specifications: Vec<PersonnelSegmentSpecification>,
            pub material_segment_specifications: Vec<MaterialSegmentSpecification>,
            pub physical_asset_segment_specifications: Vec<PhysicalAssetSegmentSpecification>,
            pub parameters: Vec<OperationsSegmentParameter>,
            pub sub_segments: Vec<OperationsSegment>,
            pub dependencies: Vec<OperationsSegmentDependency>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct OperationsDefinition {
            pub id: $id_type,
            pub name: String,
            pub version: String,
            pub operation_type: OperationType,
            pub work_master_id: Option<$id_type>,
            pub properties: Vec<OperationsDefinitionProperty>,
            pub operations_segments: Vec<OperationsSegment>,
        }

        // ── Operations scheduling ─────────────────────────────────────────────

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct SegmentRequirementParameter {
            pub id: $id_type,
            pub name: String,
            pub value: String,
            pub unit: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct SegmentRequirement {
            pub id: $id_type,
            pub operations_segment_id: $id_type,
            pub earliest_start_time: Option<String>,
            pub latest_end_time: Option<String>,
            pub duration: Option<String>,
            pub parameters: Vec<SegmentRequirementParameter>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct OperationsRequestProperty {
            pub id: $id_type,
            pub name: String,
            pub nested_properties: Vec<OperationsRequestProperty>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct OperationsRequest {
            pub id: $id_type,
            pub operation_type: OperationType,
            pub operations_definition_id: Option<$id_type>,
            pub start_time: Option<String>,
            pub end_time: Option<String>,
            pub priority: Option<i32>,
            pub properties: Vec<OperationsRequestProperty>,
            pub segment_requirements: Vec<SegmentRequirement>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct OperationsSchedule {
            pub id: $id_type,
            pub operation_type: OperationType,
            pub start_time: Option<String>,
            pub end_time: Option<String>,
            pub operations_requests: Vec<OperationsRequest>,
        }

        // ── Job orders ────────────────────────────────────────────────────────

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct JobOrderParameter {
            pub id: $id_type,
            pub name: String,
            pub value: String,
            pub unit: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct JobOrder {
            pub id: $id_type,
            pub work_type: OperationType,
            pub command: JobOrderCommandType,
            pub status: JobOrderStatus,
            pub priority: Option<i32>,
            pub operations_request_id: Option<$id_type>,
            pub operations_definition_id: Option<$id_type>,
            pub start_time: Option<String>,
            pub end_time: Option<String>,
            pub parameters: Vec<JobOrderParameter>,
        }

        // ── Operations performance ────────────────────────────────────────────

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct JobResponseParameter {
            pub id: $id_type,
            pub name: String,
            pub value: String,
            pub unit: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct JobResponse {
            pub id: $id_type,
            pub job_order_id: $id_type,
            pub actual_start_time: Option<String>,
            pub actual_end_time: Option<String>,
            pub result: OperationsResponseResult,
            pub parameters: Vec<JobResponseParameter>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct SegmentResponse {
            pub id: $id_type,
            pub operations_segment_id: $id_type,
            pub actual_start_time: Option<String>,
            pub actual_end_time: Option<String>,
            pub result: OperationsResponseResult,
            pub actual_equipment_ids: Vec<$id_type>,
            pub actual_person_ids: Vec<$id_type>,
            pub actual_material_lot_ids: Vec<$id_type>,
            pub actual_physical_asset_ids: Vec<$id_type>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct OperationsResponse {
            pub id: $id_type,
            pub operations_request_id: $id_type,
            pub actual_start_time: Option<String>,
            pub actual_end_time: Option<String>,
            pub result: OperationsResponseResult,
            pub segment_responses: Vec<SegmentResponse>,
            pub job_responses: Vec<JobResponse>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct OperationsPerformance {
            pub id: $id_type,
            pub operation_type: OperationType,
            pub start_time: Option<String>,
            pub end_time: Option<String>,
            pub operations_responses: Vec<OperationsResponse>,
        }

        // ── Operations capability ─────────────────────────────────────────────

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct OperationsCapabilityProperty {
            pub id: $id_type,
            pub name: String,
            pub nested_properties: Vec<OperationsCapabilityProperty>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct OperationsCapabilityElement {
            pub id: $id_type,
            pub operation_type: OperationType,
            pub operations_definition_id: Option<$id_type>,
            pub available_start_time: Option<String>,
            pub available_end_time: Option<String>,
            pub properties: Vec<OperationsCapabilityProperty>,
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct OperationsCapability {
            pub id: $id_type,
            pub operation_type: OperationType,
            pub start_time: Option<String>,
            pub end_time: Option<String>,
            pub elements: Vec<OperationsCapabilityElement>,
        }
    };
}
