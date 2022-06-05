#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CapabilityStatement2_Feature::CapabilityStatement2_Feature;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A Capability Statement documents a set of capabilities (behaviors) of a FHIR
/// Server for a particular version of FHIR that may be used as a statement of
/// actual server functionality or a statement of required or desired server
/// implementation.

#[derive(Debug)]
pub struct CapabilityStatement2_SearchParam<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl CapabilityStatement2_SearchParam<'_> {
    pub fn new(value: &Value) -> CapabilityStatement2_SearchParam {
        CapabilityStatement2_SearchParam {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for documentation
    pub fn _documentation(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_documentation") {
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

    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// An absolute URI that is a formal reference to where this parameter was first
    /// defined, so that a client can be confident of the meaning of the search
    /// parameter (a reference to [SearchParameter.url](searchparameter-
    /// definitions.html#SearchParameter.url)).
    /// This element SHALL be populated if the search parameter refers to a
    /// SearchParameter defined by the FHIR core specification or externally defined
    /// IGs.
    pub fn definition(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("definition") {
            return Some(string);
        }
        return None;
    }

    /// This allows documentation of any distinct behaviors about how the search
    /// parameter is used.  For example, text matching algorithms.
    pub fn documentation(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("documentation") {
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

    /// A statement that affirms support for a feature, in this context.
    pub fn feature(&self) -> Option<Vec<CapabilityStatement2_Feature>> {
        if let Some(Value::Array(val)) = self.value.get("feature") {
            return Some(
                val.into_iter()
                    .map(|e| CapabilityStatement2_Feature {
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

    /// The name of the search parameter used in the interface.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// The type of value a search parameter refers to, and how the content is
    /// interpreted.
    pub fn fhir_type(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("type") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._documentation() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.definition() {}
        if let Some(_val) = self.documentation() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.feature() {
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
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.fhir_type() {}
        return true;
    }
}

#[derive(Debug)]
pub struct CapabilityStatement2_SearchParamBuilder {
    pub(crate) value: Value,
}

impl CapabilityStatement2_SearchParamBuilder {
    pub fn build(&self) -> CapabilityStatement2_SearchParam {
        CapabilityStatement2_SearchParam {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: CapabilityStatement2_SearchParam,
    ) -> CapabilityStatement2_SearchParamBuilder {
        CapabilityStatement2_SearchParamBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> CapabilityStatement2_SearchParamBuilder {
        let mut __value: Value = json!({});
        return CapabilityStatement2_SearchParamBuilder { value: __value };
    }

    pub fn _documentation<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut CapabilityStatement2_SearchParamBuilder {
        self.value["_documentation"] = json!(val.value);
        return self;
    }

    pub fn _name<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut CapabilityStatement2_SearchParamBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn _type<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut CapabilityStatement2_SearchParamBuilder {
        self.value["_type"] = json!(val.value);
        return self;
    }

    pub fn definition<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut CapabilityStatement2_SearchParamBuilder {
        self.value["definition"] = json!(val);
        return self;
    }

    pub fn documentation<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut CapabilityStatement2_SearchParamBuilder {
        self.value["documentation"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut CapabilityStatement2_SearchParamBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn feature<'a>(
        &'a mut self,
        val: Vec<CapabilityStatement2_Feature>,
    ) -> &'a mut CapabilityStatement2_SearchParamBuilder {
        self.value["feature"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut CapabilityStatement2_SearchParamBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut CapabilityStatement2_SearchParamBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut CapabilityStatement2_SearchParamBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut CapabilityStatement2_SearchParamBuilder {
        self.value["type"] = json!(val);
        return self;
    }
}
