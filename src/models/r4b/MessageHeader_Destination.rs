#![allow(unused_imports, non_camel_case_types)]

use crate::models::r4b::Element::Element;
use crate::models::r4b::Extension::Extension;
use crate::models::r4b::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The header for a message exchange that is either requesting or responding to an
/// action.  The reference(s) that are the subject of the action as well as other
/// information related to the action are typically transmitted in a bundle in which
/// the MessageHeader resource instance is the first resource in the bundle.

#[derive(Debug)]
pub struct MessageHeader_Destination<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MessageHeader_Destination<'_> {
    pub fn new(value: &Value) -> MessageHeader_Destination {
        MessageHeader_Destination {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for endpoint
    pub fn _endpoint(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_endpoint") {
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

    /// Indicates where the message should be routed to.
    pub fn endpoint(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("endpoint") {
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

    /// Human-readable name for the target system.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// Allows data conveyed by a message to be addressed to a particular person or
    /// department when routing to a specific application isn't sufficient.
    pub fn receiver(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("receiver") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Identifies the target end system in situations where the initial message
    /// transmission is to an intermediary system.
    pub fn target(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("target") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._endpoint() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.endpoint() {}
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
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.receiver() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.target() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct MessageHeader_DestinationBuilder {
    pub(crate) value: Value,
}

impl MessageHeader_DestinationBuilder {
    pub fn build(&self) -> MessageHeader_Destination {
        MessageHeader_Destination {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: MessageHeader_Destination) -> MessageHeader_DestinationBuilder {
        MessageHeader_DestinationBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> MessageHeader_DestinationBuilder {
        let mut __value: Value = json!({});
        return MessageHeader_DestinationBuilder { value: __value };
    }

    pub fn _endpoint<'a>(&'a mut self, val: Element) -> &'a mut MessageHeader_DestinationBuilder {
        self.value["_endpoint"] = json!(val.value);
        return self;
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut MessageHeader_DestinationBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn endpoint<'a>(&'a mut self, val: &str) -> &'a mut MessageHeader_DestinationBuilder {
        self.value["endpoint"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MessageHeader_DestinationBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut MessageHeader_DestinationBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MessageHeader_DestinationBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut MessageHeader_DestinationBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn receiver<'a>(&'a mut self, val: Reference) -> &'a mut MessageHeader_DestinationBuilder {
        self.value["receiver"] = json!(val.value);
        return self;
    }

    pub fn target<'a>(&'a mut self, val: Reference) -> &'a mut MessageHeader_DestinationBuilder {
        self.value["target"] = json!(val.value);
        return self;
    }
}
