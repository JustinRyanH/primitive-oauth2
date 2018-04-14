use std::borrow::Cow;

use client::params::*;
use spectral::{AssertionFailure, Spec};


pub trait ParamAssertion<'s> {
    fn have_a_single_value(&mut self) -> Spec<'s, Cow<'s, str>>;
    fn have_multiple_values(&mut self) -> Spec<'s, Vec<Cow<'s, str>>>;
}

impl<'s> ParamAssertion<'s> for Spec<'s, ParamValue<'s>> {
    fn have_a_single_value(&mut self) -> Spec<'s, Cow<'s, str>> {
        let subject = self.subject;

        if let Some(value) = subject.single() {
            return Spec {
                subject: value,
                subject_name: self.subject_name,
                location: self.location.clone(),
                description: self.description.clone(),
            };
        } else {
            AssertionFailure::from_spec(self)
                .with_expected(format!("ParamValue to be: Single(Str)"))
                .with_actual(format!("ParamValue to be: {:?}", subject))
                .fail();
        }
        unreachable!()
    }

    fn have_multiple_values(&mut self) -> Spec<'s, Vec<Cow<'s, str>>> {
        let subject = self.subject;

        if let Some(value) = subject.multi() {
            return Spec {
                subject: value,
                subject_name: self.subject_name,
                location: self.location.clone(),
                description: self.description.clone(),
            };
        } else {
            AssertionFailure::from_spec(self)
                .with_expected(format!("ParamValue to be: Multi(Vec<Str>)"))
                .with_actual(format!("ParamValue to be: {:?}", subject))
                .fail();
        }
        unreachable!()
    }
}