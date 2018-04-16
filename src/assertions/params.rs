use std::{fmt, borrow::Cow};

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

pub trait UrlparamsAssertion<'s> {
    fn has_param<T>(&mut self, param: T) -> Spec<'s, ParamValue<'s>>
    where
        T: Clone + fmt::Debug + Into<Cow<'s, str>>;
    fn has_no_param<T>(&mut self, param: T)
    where
        T: Clone + fmt::Debug + Into<Cow<'s, str>>;
}

impl<'s> UrlparamsAssertion<'s> for Spec<'s, UrlQueryParams<'s>> {
    fn has_param<T: fmt::Debug + Into<Cow<'s, str>>>(
        &mut self,
        param: T,
    ) -> Spec<'s, ParamValue<'s>>
    where
        T: Clone + fmt::Debug + Into<Cow<'s, str>>,
    {
        let subject = self.subject;
        let get_result = subject.get(param.clone());
        if let Some(value) = get_result {
            return Spec {
                subject: value,
                subject_name: self.subject_name,
                location: self.location.clone(),
                description: self.description.clone(),
            };
        } else {
            AssertionFailure::from_spec(self)
                .with_expected(format!("UrlQueryParams to have Param {:?}", param))
                .with_actual(format!(
                    "UrlQueryParams has the keys of: {:?}",
                    subject.keys()
                ))
                .fail();
        }
        unreachable!()
    }

    fn has_no_param<T: fmt::Debug + Into<Cow<'s, str>>>(&mut self, param: T)
    where
        T: Clone + fmt::Debug + Into<Cow<'s, str>>,
    {
        let get_result = self.subject.get(param.clone());
        if let Some(_) = get_result {
            AssertionFailure::from_spec(self)
                .with_expected(format!("UrlQueryParams to not have Param {:?}", param))
                .with_actual(format!(
                    "UrlQueryParams has the keys of: {:?}",
                    self.subject.keys()
                ))
                .fail();
        }
    }
}
