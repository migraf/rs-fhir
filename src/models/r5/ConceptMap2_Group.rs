#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::ConceptMap2_Element::ConceptMap2_Element;
use crate::models::r5::ConceptMap2_Unmapped::ConceptMap2_Unmapped;
use crate::models::r5::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A statement of relationships from one set of concepts to one or more other
/// concepts - either concepts in code systems, or data element/data element
/// concepts, or classes in class models.

#[derive(Debug)]
pub struct ConceptMap2_Group<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ConceptMap2_Group<'_> {
    pub fn new(value: &Value) -> ConceptMap2_Group {
        ConceptMap2_Group {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Mappings for an individual concept in the source to one or more concepts in the
    /// target.
    pub fn element(&self) -> Vec<ConceptMap2_Element> {
        self.value
            .get("element")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(|e| ConceptMap2_Element {
                value: Cow::Borrowed(e),
            })
            .collect::<Vec<_>>()
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

    /// An absolute URI that identifies the source system where the concepts to be
    /// mapped are defined.
    pub fn source(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("source") {
            return Some(string);
        }
        return None;
    }

    /// An absolute URI that identifies the target system that the concepts will be
    /// mapped to.
    pub fn target(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("target") {
            return Some(string);
        }
        return None;
    }

    /// What to do when there is no mapping to a target concept from the source concept.
    /// This provides the "default" to be applied when there is no target concept
    /// mapping specified.  The 'unmapped' element is ignored if a code is specified to
    /// have relationship = not-related-to.
    pub fn unmapped(&self) -> Option<ConceptMap2_Unmapped> {
        if let Some(val) = self.value.get("unmapped") {
            return Some(ConceptMap2_Unmapped {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if !self
            .element()
            .into_iter()
            .map(|e| e.validate())
            .all(|x| x == true)
        {
            return false;
        }
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
        if let Some(_val) = self.source() {}
        if let Some(_val) = self.target() {}
        if let Some(_val) = self.unmapped() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ConceptMap2_GroupBuilder {
    pub(crate) value: Value,
}

impl ConceptMap2_GroupBuilder {
    pub fn build(&self) -> ConceptMap2_Group {
        ConceptMap2_Group {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ConceptMap2_Group) -> ConceptMap2_GroupBuilder {
        ConceptMap2_GroupBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(element: Vec<ConceptMap2_Element>) -> ConceptMap2_GroupBuilder {
        let mut __value: Value = json!({});
        __value["element"] = json!(element.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return ConceptMap2_GroupBuilder { value: __value };
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ConceptMap2_GroupBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ConceptMap2_GroupBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ConceptMap2_GroupBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn source<'a>(&'a mut self, val: &str) -> &'a mut ConceptMap2_GroupBuilder {
        self.value["source"] = json!(val);
        return self;
    }

    pub fn target<'a>(&'a mut self, val: &str) -> &'a mut ConceptMap2_GroupBuilder {
        self.value["target"] = json!(val);
        return self;
    }

    pub fn unmapped<'a>(
        &'a mut self,
        val: ConceptMap2_Unmapped,
    ) -> &'a mut ConceptMap2_GroupBuilder {
        self.value["unmapped"] = json!(val.value);
        return self;
    }
}
