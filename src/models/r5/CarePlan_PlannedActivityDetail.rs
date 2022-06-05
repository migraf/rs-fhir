#![allow(unused_imports, non_camel_case_types)]

use crate::models::r5::CodeableConcept::CodeableConcept;
use crate::models::r5::CodeableReference::CodeableReference;
use crate::models::r5::Element::Element;
use crate::models::r5::Extension::Extension;
use crate::models::r5::Period::Period;
use crate::models::r5::Quantity::Quantity;
use crate::models::r5::Reference::Reference;
use crate::models::r5::Timing::Timing;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Describes the intention of how one or more practitioners intend to deliver care
/// for a particular patient, group or community for a period of time, possibly
/// limited to care for a specific condition or set of conditions.

#[derive(Debug)]
pub struct CarePlan_PlannedActivityDetail<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl CarePlan_PlannedActivityDetail<'_> {
    pub fn new(value: &Value) -> CarePlan_PlannedActivityDetail {
        CarePlan_PlannedActivityDetail {
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

    /// Extensions for doNotPerform
    pub fn _do_not_perform(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_doNotPerform") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for instantiatesUri
    pub fn _instantiates_uri(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_instantiatesUri") {
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

    /// Extensions for kind
    pub fn _kind(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_kind") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for reportedBoolean
    pub fn _reported_boolean(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_reportedBoolean") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for scheduledString
    pub fn _scheduled_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_scheduledString") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Detailed description of the type of planned activity; e.g. what lab test, what
    /// procedure, what kind of encounter.
    pub fn code(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("code") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Identifies the quantity expected to be consumed in a given day.
    pub fn daily_amount(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("dailyAmount") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// This provides a textual description of constraints on the intended activity
    /// occurrence, including relation to other activities.  It may also include
    /// objectives, pre-conditions and end-conditions.  Finally, it may convey specifics
    /// about the activity such as body site, method, route, etc.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// If true, indicates that the described activity is one that must NOT be engaged
    /// in when following the plan.  If false, or missing, indicates that the described
    /// activity is one that should be engaged in when following the plan.
    pub fn do_not_perform(&self) -> Option<bool> {
        if let Some(val) = self.value.get("doNotPerform") {
            return Some(val.as_bool().unwrap());
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

    /// Internal reference that identifies the goals that this activity is intended to
    /// contribute towards meeting.
    pub fn goal(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("goal") {
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// The URL pointing to a FHIR-defined protocol, guideline, questionnaire or other
    /// definition that is adhered to in whole or in part by this CarePlan activity.
    pub fn instantiates_canonical(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("instantiatesCanonical") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The URL pointing to an externally maintained protocol, guideline, questionnaire
    /// or other definition that is adhered to in whole or in part by this CarePlan
    /// activity.
    pub fn instantiates_uri(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("instantiatesUri") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A description of the kind of resource the in-line definition of a care plan
    /// activity is representing.  The CarePlan.activity.detail is an in-line definition
    /// when a resource is not referenced using CarePlan.activity.reference.  For
    /// example, a MedicationRequest, a ServiceRequest, or a CommunicationRequest.
    pub fn kind(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("kind") {
            return Some(string);
        }
        return None;
    }

    /// Identifies the facility where the activity will occur; e.g. home, hospital,
    /// specific clinic, etc.
    pub fn location(&self) -> Option<CodeableReference> {
        if let Some(val) = self.value.get("location") {
            return Some(CodeableReference {
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

    /// Identifies who's expected to be involved in the activity.
    pub fn performer(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("performer") {
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

    /// Identifies the food, drug or other product to be consumed or supplied in the
    /// activity.
    pub fn product_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("productCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Identifies the food, drug or other product to be consumed or supplied in the
    /// activity.
    pub fn product_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("productReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Identifies the quantity expected to be supplied, administered or consumed by the
    /// subject.
    pub fn quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("quantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Provides the rationale that drove the inclusion of this particular activity as
    /// part of the plan or the reason why the activity was prohibited - either a coded
    /// concept, or another resource, such as the health condition(s), whose existence
    /// justifies this request and drove the inclusion of this particular activity as
    /// part of the plan.
    pub fn reason(&self) -> Option<Vec<CodeableReference>> {
        if let Some(Value::Array(val)) = self.value.get("reason") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableReference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Indicates if this record was captured as a secondary 'reported' record rather
    /// than as an original primary source-of-truth record.  It may also indicate the
    /// source of the report.
    pub fn reported_boolean(&self) -> Option<bool> {
        if let Some(val) = self.value.get("reportedBoolean") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Indicates if this record was captured as a secondary 'reported' record rather
    /// than as an original primary source-of-truth record.  It may also indicate the
    /// source of the report.
    pub fn reported_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("reportedReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The period, timing or frequency upon which the described activity is to occur.
    pub fn scheduled_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("scheduledPeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The period, timing or frequency upon which the described activity is to occur.
    pub fn scheduled_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("scheduledString") {
            return Some(string);
        }
        return None;
    }

    /// The period, timing or frequency upon which the described activity is to occur.
    pub fn scheduled_timing(&self) -> Option<Timing> {
        if let Some(val) = self.value.get("scheduledTiming") {
            return Some(Timing {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Identifies what progress is being made for the specific activity.
    pub fn status(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("status") {
            return Some(string);
        }
        return None;
    }

    /// Provides reason why the activity isn't yet started, is on hold, was cancelled,
    /// etc.
    pub fn status_reason(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("statusReason") {
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
        if let Some(_val) = self._do_not_perform() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._instantiates_uri() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._kind() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._reported_boolean() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._scheduled_string() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.code() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.daily_amount() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.do_not_perform() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.goal() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.instantiates_canonical() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.instantiates_uri() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.kind() {}
        if let Some(_val) = self.location() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.performer() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.product_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.product_reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.quantity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.reason() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.reported_boolean() {}
        if let Some(_val) = self.reported_reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.scheduled_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.scheduled_string() {}
        if let Some(_val) = self.scheduled_timing() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.status_reason() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct CarePlan_PlannedActivityDetailBuilder {
    pub(crate) value: Value,
}

impl CarePlan_PlannedActivityDetailBuilder {
    pub fn build(&self) -> CarePlan_PlannedActivityDetail {
        CarePlan_PlannedActivityDetail {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: CarePlan_PlannedActivityDetail) -> CarePlan_PlannedActivityDetailBuilder {
        CarePlan_PlannedActivityDetailBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> CarePlan_PlannedActivityDetailBuilder {
        let mut __value: Value = json!({});
        return CarePlan_PlannedActivityDetailBuilder { value: __value };
    }

    pub fn _description<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut CarePlan_PlannedActivityDetailBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _do_not_perform<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut CarePlan_PlannedActivityDetailBuilder {
        self.value["_doNotPerform"] = json!(val.value);
        return self;
    }

    pub fn _instantiates_uri<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut CarePlan_PlannedActivityDetailBuilder {
        self.value["_instantiatesUri"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _kind<'a>(&'a mut self, val: Element) -> &'a mut CarePlan_PlannedActivityDetailBuilder {
        self.value["_kind"] = json!(val.value);
        return self;
    }

    pub fn _reported_boolean<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut CarePlan_PlannedActivityDetailBuilder {
        self.value["_reportedBoolean"] = json!(val.value);
        return self;
    }

    pub fn _scheduled_string<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut CarePlan_PlannedActivityDetailBuilder {
        self.value["_scheduledString"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut CarePlan_PlannedActivityDetailBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn code<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut CarePlan_PlannedActivityDetailBuilder {
        self.value["code"] = json!(val.value);
        return self;
    }

    pub fn daily_amount<'a>(
        &'a mut self,
        val: Quantity,
    ) -> &'a mut CarePlan_PlannedActivityDetailBuilder {
        self.value["dailyAmount"] = json!(val.value);
        return self;
    }

    pub fn description<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut CarePlan_PlannedActivityDetailBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn do_not_perform<'a>(
        &'a mut self,
        val: bool,
    ) -> &'a mut CarePlan_PlannedActivityDetailBuilder {
        self.value["doNotPerform"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut CarePlan_PlannedActivityDetailBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn goal<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut CarePlan_PlannedActivityDetailBuilder {
        self.value["goal"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut CarePlan_PlannedActivityDetailBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn instantiates_canonical<'a>(
        &'a mut self,
        val: Vec<&str>,
    ) -> &'a mut CarePlan_PlannedActivityDetailBuilder {
        self.value["instantiatesCanonical"] = json!(val);
        return self;
    }

    pub fn instantiates_uri<'a>(
        &'a mut self,
        val: Vec<&str>,
    ) -> &'a mut CarePlan_PlannedActivityDetailBuilder {
        self.value["instantiatesUri"] = json!(val);
        return self;
    }

    pub fn kind<'a>(&'a mut self, val: &str) -> &'a mut CarePlan_PlannedActivityDetailBuilder {
        self.value["kind"] = json!(val);
        return self;
    }

    pub fn location<'a>(
        &'a mut self,
        val: CodeableReference,
    ) -> &'a mut CarePlan_PlannedActivityDetailBuilder {
        self.value["location"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut CarePlan_PlannedActivityDetailBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn performer<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut CarePlan_PlannedActivityDetailBuilder {
        self.value["performer"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn product_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut CarePlan_PlannedActivityDetailBuilder {
        self.value["productCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn product_reference<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut CarePlan_PlannedActivityDetailBuilder {
        self.value["productReference"] = json!(val.value);
        return self;
    }

    pub fn quantity<'a>(
        &'a mut self,
        val: Quantity,
    ) -> &'a mut CarePlan_PlannedActivityDetailBuilder {
        self.value["quantity"] = json!(val.value);
        return self;
    }

    pub fn reason<'a>(
        &'a mut self,
        val: Vec<CodeableReference>,
    ) -> &'a mut CarePlan_PlannedActivityDetailBuilder {
        self.value["reason"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn reported_boolean<'a>(
        &'a mut self,
        val: bool,
    ) -> &'a mut CarePlan_PlannedActivityDetailBuilder {
        self.value["reportedBoolean"] = json!(val);
        return self;
    }

    pub fn reported_reference<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut CarePlan_PlannedActivityDetailBuilder {
        self.value["reportedReference"] = json!(val.value);
        return self;
    }

    pub fn scheduled_period<'a>(
        &'a mut self,
        val: Period,
    ) -> &'a mut CarePlan_PlannedActivityDetailBuilder {
        self.value["scheduledPeriod"] = json!(val.value);
        return self;
    }

    pub fn scheduled_string<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut CarePlan_PlannedActivityDetailBuilder {
        self.value["scheduledString"] = json!(val);
        return self;
    }

    pub fn scheduled_timing<'a>(
        &'a mut self,
        val: Timing,
    ) -> &'a mut CarePlan_PlannedActivityDetailBuilder {
        self.value["scheduledTiming"] = json!(val.value);
        return self;
    }

    pub fn status<'a>(&'a mut self, val: &str) -> &'a mut CarePlan_PlannedActivityDetailBuilder {
        self.value["status"] = json!(val);
        return self;
    }

    pub fn status_reason<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut CarePlan_PlannedActivityDetailBuilder {
        self.value["statusReason"] = json!(val.value);
        return self;
    }
}
