#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::StructureMap_Parameter::StructureMap_Parameter;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A Map of relationships between 2 structures that can be used to transform data.

#[derive(Debug)]
pub struct StructureMap_Target<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl StructureMap_Target<'_> {
    pub fn new(value: &Value) -> StructureMap_Target {
        StructureMap_Target {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for context
    pub fn _context(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_context") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for element
    pub fn _element(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_element") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for listMode
    pub fn _list_mode(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_listMode") {
            return Some(
                val.into_iter()
                    .map(|e| Element {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for listRuleId
    pub fn _list_rule_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_listRuleId") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for transform
    pub fn _transform(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_transform") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for variable
    pub fn _variable(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_variable") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Variable this rule applies to.
    pub fn context(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("context") {
            return Some(string);
        }
        return None;
    }

    /// Field to create in the context.
    pub fn element(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("element") {
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

    /// If field is a list, how to manage the list.
    pub fn list_mode(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("listMode") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Internal rule reference for shared list items.
    pub fn list_rule_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("listRuleId") {
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

    /// Parameters to the transform.
    pub fn parameter(&self) -> Option<Vec<StructureMap_Parameter>> {
        if let Some(Value::Array(val)) = self.value.get("parameter") {
            return Some(
                val.into_iter()
                    .map(|e| StructureMap_Parameter {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// How the data is copied / created.
    pub fn transform(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("transform") {
            return Some(string);
        }
        return None;
    }

    /// Named context for field, if desired, and a field is specified.
    pub fn variable(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("variable") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._context() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._element() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._list_mode() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._list_rule_id() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._transform() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._variable() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.context() {}
        if let Some(_val) = self.element() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.list_mode() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.list_rule_id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.parameter() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.transform() {}
        if let Some(_val) = self.variable() {}
        return true;
    }
}

#[derive(Debug)]
pub struct StructureMap_TargetBuilder {
    pub(crate) value: Value,
}

impl StructureMap_TargetBuilder {
    pub fn build(&self) -> StructureMap_Target {
        StructureMap_Target {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: StructureMap_Target) -> StructureMap_TargetBuilder {
        StructureMap_TargetBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> StructureMap_TargetBuilder {
        let mut __value: Value = json!({});
        return StructureMap_TargetBuilder { value: __value };
    }

    pub fn _context<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_TargetBuilder {
        self.value["_context"] = json!(val.value);
        return self;
    }

    pub fn _element<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_TargetBuilder {
        self.value["_element"] = json!(val.value);
        return self;
    }

    pub fn _list_mode<'a>(&'a mut self, val: Vec<Element>) -> &'a mut StructureMap_TargetBuilder {
        self.value["_listMode"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _list_rule_id<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_TargetBuilder {
        self.value["_listRuleId"] = json!(val.value);
        return self;
    }

    pub fn _transform<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_TargetBuilder {
        self.value["_transform"] = json!(val.value);
        return self;
    }

    pub fn _variable<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_TargetBuilder {
        self.value["_variable"] = json!(val.value);
        return self;
    }

    pub fn context<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_TargetBuilder {
        self.value["context"] = json!(val);
        return self;
    }

    pub fn element<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_TargetBuilder {
        self.value["element"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut StructureMap_TargetBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_TargetBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn list_mode<'a>(&'a mut self, val: Vec<&str>) -> &'a mut StructureMap_TargetBuilder {
        self.value["listMode"] = json!(val);
        return self;
    }

    pub fn list_rule_id<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_TargetBuilder {
        self.value["listRuleId"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut StructureMap_TargetBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn parameter<'a>(
        &'a mut self,
        val: Vec<StructureMap_Parameter>,
    ) -> &'a mut StructureMap_TargetBuilder {
        self.value["parameter"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn transform<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_TargetBuilder {
        self.value["transform"] = json!(val);
        return self;
    }

    pub fn variable<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_TargetBuilder {
        self.value["variable"] = json!(val);
        return self;
    }
}