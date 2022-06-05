#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::DeviceDefinition_MarketDistribution1::DeviceDefinition_MarketDistribution1;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The characteristics, operational status and capabilities of a medical-related
/// component of a medical device.

#[derive(Debug)]
pub struct DeviceDefinition_UdiDeviceIdentifier1<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl DeviceDefinition_UdiDeviceIdentifier1<'_> {
    pub fn new(value: &Value) -> DeviceDefinition_UdiDeviceIdentifier1 {
        DeviceDefinition_UdiDeviceIdentifier1 {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for deviceIdentifier
    pub fn _device_identifier(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_deviceIdentifier") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for issuer
    pub fn _issuer(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_issuer") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for jurisdiction
    pub fn _jurisdiction(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_jurisdiction") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Identifier to be associated with every instance for issuer and jurisdiction.
    pub fn device_identifier(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("deviceIdentifier") {
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

    /// The organization that assigns the identifier algorithm.
    pub fn issuer(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("issuer") {
            return Some(string);
        }
        return None;
    }

    /// The jurisdiction to which the deviceIdentifier applies.
    pub fn jurisdiction(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("jurisdiction") {
            return Some(string);
        }
        return None;
    }

    /// The organization that assigns the identifier algorithm.
    pub fn market_distribution(&self) -> Option<DeviceDefinition_MarketDistribution1> {
        if let Some(val) = self.value.get("marketDistribution") {
            return Some(DeviceDefinition_MarketDistribution1 {
                value: Cow::Borrowed(val),
            });
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._device_identifier() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._issuer() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._jurisdiction() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.device_identifier() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.issuer() {}
        if let Some(_val) = self.jurisdiction() {}
        if let Some(_val) = self.market_distribution() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct DeviceDefinition_UdiDeviceIdentifier1Builder {
    pub(crate) value: Value,
}

impl DeviceDefinition_UdiDeviceIdentifier1Builder {
    pub fn build(&self) -> DeviceDefinition_UdiDeviceIdentifier1 {
        DeviceDefinition_UdiDeviceIdentifier1 {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: DeviceDefinition_UdiDeviceIdentifier1,
    ) -> DeviceDefinition_UdiDeviceIdentifier1Builder {
        DeviceDefinition_UdiDeviceIdentifier1Builder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> DeviceDefinition_UdiDeviceIdentifier1Builder {
        let mut __value: Value = json!({});
        return DeviceDefinition_UdiDeviceIdentifier1Builder { value: __value };
    }

    pub fn _device_identifier<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut DeviceDefinition_UdiDeviceIdentifier1Builder {
        self.value["_deviceIdentifier"] = json!(val.value);
        return self;
    }

    pub fn _issuer<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut DeviceDefinition_UdiDeviceIdentifier1Builder {
        self.value["_issuer"] = json!(val.value);
        return self;
    }

    pub fn _jurisdiction<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut DeviceDefinition_UdiDeviceIdentifier1Builder {
        self.value["_jurisdiction"] = json!(val.value);
        return self;
    }

    pub fn device_identifier<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut DeviceDefinition_UdiDeviceIdentifier1Builder {
        self.value["deviceIdentifier"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut DeviceDefinition_UdiDeviceIdentifier1Builder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut DeviceDefinition_UdiDeviceIdentifier1Builder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn issuer<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut DeviceDefinition_UdiDeviceIdentifier1Builder {
        self.value["issuer"] = json!(val);
        return self;
    }

    pub fn jurisdiction<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut DeviceDefinition_UdiDeviceIdentifier1Builder {
        self.value["jurisdiction"] = json!(val);
        return self;
    }

    pub fn market_distribution<'a>(
        &'a mut self,
        val: DeviceDefinition_MarketDistribution1,
    ) -> &'a mut DeviceDefinition_UdiDeviceIdentifier1Builder {
        self.value["marketDistribution"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut DeviceDefinition_UdiDeviceIdentifier1Builder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
