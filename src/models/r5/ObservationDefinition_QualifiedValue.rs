#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Range::Range;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Set of definitional characteristics for a kind of observation or measurement
/// produced or consumed by an orderable health care service.

#[derive(Debug)]
pub struct ObservationDefinition_QualifiedValue<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ObservationDefinition_QualifiedValue<'_> {
    pub fn new(value: &Value) -> ObservationDefinition_QualifiedValue {
        ObservationDefinition_QualifiedValue {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for condition
    pub fn _condition(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_condition") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for gender
    pub fn _gender(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_gender") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for rangeCategory
    pub fn _range_category(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_rangeCategory") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The set of abnormal coded results for qualitative observations  that match the
    /// criteria of this set of qualified values.
    pub fn abnormal_coded_value_set(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("abnormalCodedValueSet") {
            return Some(string);
        }
        return None;
    }

    /// The age range this  set of qualified values applies to.
    pub fn age(&self) -> Option<Range> {
        if let Some(val) = self.value.get("age") {
            return Some(Range {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The target population this  set of qualified values applies to.
    pub fn applies_to(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("appliesTo") {
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

    /// Text based condition for which the the set of qualified values is valid.
    pub fn condition(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("condition") {
            return Some(string);
        }
        return None;
    }

    /// A concept defining the context for this set of qualified values.
    pub fn context(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("context") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The set of critical coded results for qualitative observations  that match the
    /// criteria of this set of qualified values.
    pub fn critical_coded_value_set(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("criticalCodedValueSet") {
            return Some(string);
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element. To make the use of extensions safe and manageable,
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

    /// The gender this  set of qualified values applies to.
    pub fn gender(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("gender") {
            return Some(string);
        }
        return None;
    }

    /// The gestational age this  set of qualified values applies to.
    pub fn gestational_age(&self) -> Option<Range> {
        if let Some(val) = self.value.get("gestationalAge") {
            return Some(Range {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element and that modifies the understanding of the element in
    /// which it is contained and/or the understanding of the containing element's
    /// descendants. Usually modifier elements provide negation or qualification. To
    /// make the use of extensions safe and manageable, there is a strict set of
    /// governance applied to the definition and use of extensions. Though any
    /// implementer can define an extension, there is a set of requirements that SHALL
    /// be met as part of the definition of the extension. Applications processing a
    /// resource are required to check for modifier extensions.    Modifier extensions
    /// SHALL NOT change the meaning of any elements on Resource or DomainResource
    /// (including cannot change the meaning of modifierExtension itself).
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

    /// The set of normal coded results for qualitative observations  that match the
    /// criteria of this set of qualified values.
    pub fn normal_coded_value_set(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("normalCodedValueSet") {
            return Some(string);
        }
        return None;
    }

    /// The range of values defined for continuous or ordinal observations that match
    /// the criteria of this set of qualified values.
    pub fn range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("range") {
            return Some(Range {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The category of range of values for continuous or ordinal observations that
    /// match the criteria of this set of qualified values.
    pub fn range_category(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("rangeCategory") {
            return Some(string);
        }
        return None;
    }

    /// The set of valid coded results for qualitative observations  that match the
    /// criteria of this set of qualified values.
    pub fn valid_coded_value_set(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("validCodedValueSet") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._condition() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._gender() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._range_category() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.abnormal_coded_value_set() {}
        if let Some(_val) = self.age() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.applies_to() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.condition() {}
        if let Some(_val) = self.context() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.critical_coded_value_set() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.gender() {}
        if let Some(_val) = self.gestational_age() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.normal_coded_value_set() {}
        if let Some(_val) = self.range() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.range_category() {}
        if let Some(_val) = self.valid_coded_value_set() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ObservationDefinition_QualifiedValueBuilder {
    pub(crate) value: Value,
}

impl ObservationDefinition_QualifiedValueBuilder {
    pub fn build(&self) -> ObservationDefinition_QualifiedValue {
        ObservationDefinition_QualifiedValue {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: ObservationDefinition_QualifiedValue,
    ) -> ObservationDefinition_QualifiedValueBuilder {
        ObservationDefinition_QualifiedValueBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ObservationDefinition_QualifiedValueBuilder {
        let mut __value: Value = json!({});
        return ObservationDefinition_QualifiedValueBuilder { value: __value };
    }

    pub fn _condition<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ObservationDefinition_QualifiedValueBuilder {
        self.value["_condition"] = json!(val.value);
        return self;
    }

    pub fn _gender<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ObservationDefinition_QualifiedValueBuilder {
        self.value["_gender"] = json!(val.value);
        return self;
    }

    pub fn _range_category<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ObservationDefinition_QualifiedValueBuilder {
        self.value["_rangeCategory"] = json!(val.value);
        return self;
    }

    pub fn abnormal_coded_value_set<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut ObservationDefinition_QualifiedValueBuilder {
        self.value["abnormalCodedValueSet"] = json!(val);
        return self;
    }

    pub fn age<'a>(
        &'a mut self,
        val: Range,
    ) -> &'a mut ObservationDefinition_QualifiedValueBuilder {
        self.value["age"] = json!(val.value);
        return self;
    }

    pub fn applies_to<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut ObservationDefinition_QualifiedValueBuilder {
        self.value["appliesTo"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn condition<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut ObservationDefinition_QualifiedValueBuilder {
        self.value["condition"] = json!(val);
        return self;
    }

    pub fn context<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut ObservationDefinition_QualifiedValueBuilder {
        self.value["context"] = json!(val.value);
        return self;
    }

    pub fn critical_coded_value_set<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut ObservationDefinition_QualifiedValueBuilder {
        self.value["criticalCodedValueSet"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ObservationDefinition_QualifiedValueBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn gender<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut ObservationDefinition_QualifiedValueBuilder {
        self.value["gender"] = json!(val);
        return self;
    }

    pub fn gestational_age<'a>(
        &'a mut self,
        val: Range,
    ) -> &'a mut ObservationDefinition_QualifiedValueBuilder {
        self.value["gestationalAge"] = json!(val.value);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ObservationDefinition_QualifiedValueBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ObservationDefinition_QualifiedValueBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn normal_coded_value_set<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut ObservationDefinition_QualifiedValueBuilder {
        self.value["normalCodedValueSet"] = json!(val);
        return self;
    }

    pub fn range<'a>(
        &'a mut self,
        val: Range,
    ) -> &'a mut ObservationDefinition_QualifiedValueBuilder {
        self.value["range"] = json!(val.value);
        return self;
    }

    pub fn range_category<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut ObservationDefinition_QualifiedValueBuilder {
        self.value["rangeCategory"] = json!(val);
        return self;
    }

    pub fn valid_coded_value_set<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut ObservationDefinition_QualifiedValueBuilder {
        self.value["validCodedValueSet"] = json!(val);
        return self;
    }
}