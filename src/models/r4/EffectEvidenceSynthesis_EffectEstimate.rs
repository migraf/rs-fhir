#![allow(unused_imports, non_camel_case_types)]

use crate::models::r4::CodeableConcept::CodeableConcept;
use crate::models::r4::EffectEvidenceSynthesis_PrecisionEstimate::EffectEvidenceSynthesis_PrecisionEstimate;
use crate::models::r4::Element::Element;
use crate::models::r4::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The EffectEvidenceSynthesis resource describes the difference in an outcome
/// between exposures states in a population where the effect estimate is derived
/// from a combination of research studies.

#[derive(Debug)]
pub struct EffectEvidenceSynthesis_EffectEstimate<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl EffectEvidenceSynthesis_EffectEstimate<'_> {
    pub fn new(value: &Value) -> EffectEvidenceSynthesis_EffectEstimate {
        EffectEvidenceSynthesis_EffectEstimate {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
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

    /// Extensions for value
    pub fn _value(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_value") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Human-readable summary of effect estimate.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
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

    /// A description of the precision of the estimate for the effect.
    pub fn precision_estimate(&self) -> Option<Vec<EffectEvidenceSynthesis_PrecisionEstimate>> {
        if let Some(Value::Array(val)) = self.value.get("precisionEstimate") {
            return Some(
                val.into_iter()
                    .map(|e| EffectEvidenceSynthesis_PrecisionEstimate {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Examples include relative risk and mean difference.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Specifies the UCUM unit for the outcome.
    pub fn unit_of_measure(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("unitOfMeasure") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The point estimate of the effect estimate.
    pub fn value(&self) -> Option<f64> {
        if let Some(val) = self.value.get("value") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Used to define variant exposure states such as low-risk state.
    pub fn variant_state(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("variantState") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.precision_estimate() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.unit_of_measure() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value() {}
        if let Some(_val) = self.variant_state() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct EffectEvidenceSynthesis_EffectEstimateBuilder {
    pub(crate) value: Value,
}

impl EffectEvidenceSynthesis_EffectEstimateBuilder {
    pub fn build(&self) -> EffectEvidenceSynthesis_EffectEstimate {
        EffectEvidenceSynthesis_EffectEstimate {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: EffectEvidenceSynthesis_EffectEstimate,
    ) -> EffectEvidenceSynthesis_EffectEstimateBuilder {
        EffectEvidenceSynthesis_EffectEstimateBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> EffectEvidenceSynthesis_EffectEstimateBuilder {
        let mut __value: Value = json!({});
        return EffectEvidenceSynthesis_EffectEstimateBuilder { value: __value };
    }

    pub fn _description<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut EffectEvidenceSynthesis_EffectEstimateBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _value<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut EffectEvidenceSynthesis_EffectEstimateBuilder {
        self.value["_value"] = json!(val.value);
        return self;
    }

    pub fn description<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut EffectEvidenceSynthesis_EffectEstimateBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut EffectEvidenceSynthesis_EffectEstimateBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut EffectEvidenceSynthesis_EffectEstimateBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut EffectEvidenceSynthesis_EffectEstimateBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn precision_estimate<'a>(
        &'a mut self,
        val: Vec<EffectEvidenceSynthesis_PrecisionEstimate>,
    ) -> &'a mut EffectEvidenceSynthesis_EffectEstimateBuilder {
        self.value["precisionEstimate"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut EffectEvidenceSynthesis_EffectEstimateBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }

    pub fn unit_of_measure<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut EffectEvidenceSynthesis_EffectEstimateBuilder {
        self.value["unitOfMeasure"] = json!(val.value);
        return self;
    }

    pub fn value<'a>(
        &'a mut self,
        val: f64,
    ) -> &'a mut EffectEvidenceSynthesis_EffectEstimateBuilder {
        self.value["value"] = json!(val);
        return self;
    }

    pub fn variant_state<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut EffectEvidenceSynthesis_EffectEstimateBuilder {
        self.value["variantState"] = json!(val.value);
        return self;
    }
}
