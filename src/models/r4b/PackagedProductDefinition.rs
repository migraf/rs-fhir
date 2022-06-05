#![allow(unused_imports, non_camel_case_types)]

use crate::models::r4b::CodeableConcept::CodeableConcept;
use crate::models::r4b::Element::Element;
use crate::models::r4b::Extension::Extension;
use crate::models::r4b::Identifier::Identifier;
use crate::models::r4b::MarketingStatus::MarketingStatus;
use crate::models::r4b::Meta::Meta;
use crate::models::r4b::Narrative::Narrative;
use crate::models::r4b::PackagedProductDefinition_LegalStatusOfSupply::PackagedProductDefinition_LegalStatusOfSupply;
use crate::models::r4b::PackagedProductDefinition_Package::PackagedProductDefinition_Package;
use crate::models::r4b::Quantity::Quantity;
use crate::models::r4b::Reference::Reference;
use crate::models::r4b::ResourceList::ResourceList;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A medically related item or items, in a container or package.

#[derive(Debug)]
pub struct PackagedProductDefinition<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl PackagedProductDefinition<'_> {
    pub fn new(value: &Value) -> PackagedProductDefinition {
        PackagedProductDefinition {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for copackagedIndicator
    pub fn _copackaged_indicator(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_copackagedIndicator") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for name
    pub fn _name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_name") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for statusDate
    pub fn _status_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_statusDate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Allows the key features to be recorded, such as "hospital pack", "nurse
    /// prescribable", "calendar pack".
    pub fn characteristic(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("characteristic") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, and nor can they
    /// have their own independent transaction scope.
    pub fn contained(&self) -> Option<Vec<ResourceList>> {
        if let Some(Value::Array(val)) = self.value.get("contained") {
            return Some(
                val.into_iter()
                    .map(|e| ResourceList {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A total of the complete count of contained items of a particular type/form,
    /// independent of sub-packaging or organization. This can be considered as the pack
    /// size. This attribute differs from containedItem.amount in that it can give a
    /// single aggregated count of all tablet types in a pack, even when these are
    /// different manufactured items. For example a pill pack of 21 tablets plus 7 sugar
    /// tablets, can be denoted here as '28 tablets'. This attribute is repeatable so
    /// that the different item types in one pack type can be counted (e.g. a count of
    /// vials and count of syringes). Each repeat must have different units, so that it
    /// is clear what the different sets of counted items are, and it is not intended to
    /// allow different counts of similar items (e.g. not '2 tubes and 3 tubes').
    /// Repeats are not to be used to represent different pack sizes (e.g. 20 pack vs.
    /// 50 pack) - which would be different instances of this resource.
    pub fn contained_item_quantity(&self) -> Option<Vec<Quantity>> {
        if let Some(Value::Array(val)) = self.value.get("containedItemQuantity") {
            return Some(
                val.into_iter()
                    .map(|e| Quantity {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// States whether a drug product is supplied with another item such as a diluent or
    /// adjuvant.
    pub fn copackaged_indicator(&self) -> Option<bool> {
        if let Some(val) = self.value.get("copackagedIndicator") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Textual description. Note that this is not the name of the package or product.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the resource. To make the use of extensions safe and manageable,
    /// there is a strict set of governance  applied to the definition and use of
    /// extensions. Though any implementer can define an extension, there is a set of
    /// requirements that SHALL be met as part of the definition of the extension.
    pub fn extension(&self) -> Option<Vec<Extension>> {
        if let Some(Value::Array(val)) = self.value.get("extension") {
            return Some(
                val.into_iter()
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// A unique identifier for this package as whole. Unique instance identifiers
    /// assigned to a package by manufacturers, regulators, drug catalogue custodians or
    /// other organizations.
    pub fn identifier(&self) -> Option<Vec<Identifier>> {
        if let Some(Value::Array(val)) = self.value.get("identifier") {
            return Some(
                val.into_iter()
                    .map(|e| Identifier {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub fn implicit_rules(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("implicitRules") {
            return Some(string);
        }
        return None;
    }

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
        }
        return None;
    }

    /// The legal status of supply of the packaged item as classified by the regulator.
    pub fn legal_status_of_supply(
        &self,
    ) -> Option<Vec<PackagedProductDefinition_LegalStatusOfSupply>> {
        if let Some(Value::Array(val)) = self.value.get("legalStatusOfSupply") {
            return Some(
                val.into_iter()
                    .map(|e| PackagedProductDefinition_LegalStatusOfSupply {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Manufacturer of this package type. When there are multiple it means these are
    /// all possible manufacturers.
    pub fn manufacturer(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("manufacturer") {
            return Some(
                val.into_iter()
                    .map(|e| Reference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Allows specifying that an item is on the market for sale, or that it is not
    /// available, and the dates and locations associated.
    pub fn marketing_status(&self) -> Option<Vec<MarketingStatus>> {
        if let Some(Value::Array(val)) = self.value.get("marketingStatus") {
            return Some(
                val.into_iter()
                    .map(|e| MarketingStatus {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the resource and that modifies the understanding of the element
    /// that contains it and/or the understanding of the containing element's
    /// descendants. Usually modifier elements provide negation or qualification. To
    /// make the use of extensions safe and manageable, there is a strict set of
    /// governance applied to the definition and use of extensions. Though any
    /// implementer is allowed to define an extension, there is a set of requirements
    /// that SHALL be met as part of the definition of the extension. Applications
    /// processing a resource are required to check for modifier extensions.    Modifier
    /// extensions SHALL NOT change the meaning of any elements on Resource or
    /// DomainResource (including cannot change the meaning of modifierExtension
    /// itself).
    pub fn modifier_extension(&self) -> Option<Vec<Extension>> {
        if let Some(Value::Array(val)) = self.value.get("modifierExtension") {
            return Some(
                val.into_iter()
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A name for this package. Typically what it would be listed as in a drug
    /// formulary or catalogue, inventory etc.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// A packaging item, as a container for medically related items, possibly with
    /// other packaging items within, or a packaging component, such as bottle cap
    /// (which is not a device or a medication manufactured item).
    pub fn package(&self) -> Option<PackagedProductDefinition_Package> {
        if let Some(val) = self.value.get("package") {
            return Some(PackagedProductDefinition_Package {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The product that this is a pack for.
    pub fn package_for(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("packageFor") {
            return Some(
                val.into_iter()
                    .map(|e| Reference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The status within the lifecycle of this item. A high level status, this is not
    /// intended to duplicate details carried elsewhere such as legal status, or
    /// authorization or marketing status.
    pub fn status(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("status") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The date at which the given status became applicable.
    pub fn status_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("statusDate") {
            return Some(string);
        }
        return None;
    }

    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need not
    /// encode all the structured data, but is required to contain sufficient detail to
    /// make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub fn text(&self) -> Option<Narrative> {
        if let Some(val) = self.value.get("text") {
            return Some(Narrative {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A high level category e.g. medicinal product, raw material, shipping/transport
    /// container, etc.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._copackaged_indicator() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._implicit_rules() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._language() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.characteristic() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.contained_item_quantity() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.copackaged_indicator() {}
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.identifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.legal_status_of_supply() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.manufacturer() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.marketing_status() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.meta() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.package() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.package_for() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.status_date() {}
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct PackagedProductDefinitionBuilder {
    pub(crate) value: Value,
}

impl PackagedProductDefinitionBuilder {
    pub fn build(&self) -> PackagedProductDefinition {
        PackagedProductDefinition {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: PackagedProductDefinition) -> PackagedProductDefinitionBuilder {
        PackagedProductDefinitionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> PackagedProductDefinitionBuilder {
        let mut __value: Value = json!({});
        return PackagedProductDefinitionBuilder { value: __value };
    }

    pub fn _copackaged_indicator<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut PackagedProductDefinitionBuilder {
        self.value["_copackagedIndicator"] = json!(val.value);
        return self;
    }

    pub fn _description<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut PackagedProductDefinitionBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut PackagedProductDefinitionBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut PackagedProductDefinitionBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut PackagedProductDefinitionBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn _status_date<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut PackagedProductDefinitionBuilder {
        self.value["_statusDate"] = json!(val.value);
        return self;
    }

    pub fn characteristic<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut PackagedProductDefinitionBuilder {
        self.value["characteristic"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained<'a>(
        &'a mut self,
        val: Vec<ResourceList>,
    ) -> &'a mut PackagedProductDefinitionBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained_item_quantity<'a>(
        &'a mut self,
        val: Vec<Quantity>,
    ) -> &'a mut PackagedProductDefinitionBuilder {
        self.value["containedItemQuantity"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn copackaged_indicator<'a>(
        &'a mut self,
        val: bool,
    ) -> &'a mut PackagedProductDefinitionBuilder {
        self.value["copackagedIndicator"] = json!(val);
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut PackagedProductDefinitionBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut PackagedProductDefinitionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut PackagedProductDefinitionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(
        &'a mut self,
        val: Vec<Identifier>,
    ) -> &'a mut PackagedProductDefinitionBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut PackagedProductDefinitionBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut PackagedProductDefinitionBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn legal_status_of_supply<'a>(
        &'a mut self,
        val: Vec<PackagedProductDefinition_LegalStatusOfSupply>,
    ) -> &'a mut PackagedProductDefinitionBuilder {
        self.value["legalStatusOfSupply"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn manufacturer<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut PackagedProductDefinitionBuilder {
        self.value["manufacturer"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn marketing_status<'a>(
        &'a mut self,
        val: Vec<MarketingStatus>,
    ) -> &'a mut PackagedProductDefinitionBuilder {
        self.value["marketingStatus"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut PackagedProductDefinitionBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut PackagedProductDefinitionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut PackagedProductDefinitionBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn package<'a>(
        &'a mut self,
        val: PackagedProductDefinition_Package,
    ) -> &'a mut PackagedProductDefinitionBuilder {
        self.value["package"] = json!(val.value);
        return self;
    }

    pub fn package_for<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut PackagedProductDefinitionBuilder {
        self.value["packageFor"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn status<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut PackagedProductDefinitionBuilder {
        self.value["status"] = json!(val.value);
        return self;
    }

    pub fn status_date<'a>(&'a mut self, val: &str) -> &'a mut PackagedProductDefinitionBuilder {
        self.value["statusDate"] = json!(val);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut PackagedProductDefinitionBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut PackagedProductDefinitionBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }
}
