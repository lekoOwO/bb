use chrono::NaiveDateTime;
use rocket::request::FromFormValue;
use std::ops::Deref;

pub struct NaiveDateTimeForm(NaiveDateTime);
impl<'v> FromFormValue<'v> for NaiveDateTimeForm {
    type Error = &'v rocket::http::RawStr;

    fn from_form_value(form_value: &'v rocket::http::RawStr) -> Result<NaiveDateTimeForm, &'v rocket::http::RawStr> {
        let value = form_value.url_decode_lossy();
        let inner = NaiveDateTime::parse_from_str(value.as_str(), "%Y-%m-%dT%H:%M:%S%.f").unwrap();
        Ok(NaiveDateTimeForm(inner))
    }
}
impl Deref for NaiveDateTimeForm {
    type Target = NaiveDateTime;

    fn deref(&self) -> &Self::Target{
        &self.0
    }
}

use uuid::Uuid;
pub struct UuidForm(Uuid);
impl<'v> FromFormValue<'v> for UuidForm {
    type Error = &'v rocket::http::RawStr;

    fn from_form_value(form_value: &'v rocket::http::RawStr) -> Result<UuidForm, &'v rocket::http::RawStr> {
        let value = form_value.url_decode_lossy();
        let inner = Uuid::parse_str(value.as_str()).unwrap();
        Ok(UuidForm(inner))
    }
}
impl Deref for UuidForm {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target{
        &self.0
    }
}