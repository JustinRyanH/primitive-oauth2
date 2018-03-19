use std::iter::FromIterator;
use std::ops::Deref;
use std::collections::HashMap;

use url;

#[derive(Debug, Clone, PartialEq)]
pub enum ParamValue {
    Single(String),
    Multi(Vec<String>),
}

impl ParamValue {
    pub fn method(self) -> bool {
        false
    }
    pub fn is_single(&self) -> bool {
        match self {
            &ParamValue::Single(_) => true,
            &ParamValue::Multi(_) => false,
        }
    }

    pub fn is_multi(&self) -> bool {
        return !self.is_single();
    }

    pub fn single(&self) -> Option<&String> {
        match self {
            &ParamValue::Single(ref s) => Some(s),
            _ => None,
        }
    }

    pub fn multi(&self) -> Option<&Vec<String>> {
        match self {
            &ParamValue::Multi(ref v) => Some(v),
            _ => None,
        }
    }
}

impl<T> From<T> for ParamValue
where
    T: Into<String>,
{
    fn from(v: T) -> ParamValue {
        ParamValue::Single(v.into())
    }
}

impl<T> FromIterator<T> for ParamValue
where
    T: Into<String>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let as_vec: Vec<String> = iter.into_iter().map(|v| v.into()).collect();
        let count = as_vec.len();
        match count {
            0 => ParamValue::Single("".into()),
            1 => ParamValue::Single(as_vec.first().unwrap().clone()),
            _ => ParamValue::Multi(as_vec),
        }
    }
}

impl IntoIterator for ParamValue {
    type Item = String;
    type IntoIter = ::std::vec::IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            ParamValue::Single(v) => vec![v].into_iter(),
            ParamValue::Multi(v) => v.into_iter(),
        }
    }
}

pub struct UrlQueryParams(HashMap<String, ParamValue>);

impl UrlQueryParams {
    pub fn new() -> UrlQueryParams {
        UrlQueryParams(HashMap::new())
    }

    pub fn get<T: Into<String>>(&self, key: T) -> Option<ParamValue> {
        self.0.get(&key.into()).map(|v| v.clone())
    }
}

impl IntoIterator for UrlQueryParams {
    type Item = (String, String);
    type IntoIter = ::std::vec::IntoIter<(String, String)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0
            .into_iter()
            .map(|(k, v): (String, ParamValue)| {
                v.into_iter()
                    .map(move |inner_v| (k.clone(), inner_v))
                    .collect()
            })
            .fold(Vec::new(), |mut acc, ref mut i| {
                acc.append(i);
                return acc;
            })
            .into_iter()
    }
}

impl Deref for UrlQueryParams {
    type Target = HashMap<String, ParamValue>;
    fn deref(&self) -> &HashMap<String, ParamValue> {
        &self.0
    }
}

impl<'a, T, S> FromIterator<&'a (T, S)> for UrlQueryParams
where
    T: Into<String> + Clone,
    S: Into<String> + Clone,
{
    #[inline]
    fn from_iter<I: IntoIterator<Item = &'a (T, S)>>(i: I) -> UrlQueryParams {
        UrlQueryParams(i.into_iter().fold(
            HashMap::<String, ParamValue>::new(),
            |mut acc, &(ref key, ref value)| {
                let new_value: ParamValue = match acc.get(&key.clone().into()) {
                    Some(v) => match v {
                        &ParamValue::Single(ref sv) => {
                            ParamValue::Multi(vec![sv.clone(), value.clone().into()])
                        }
                        &ParamValue::Multi(ref mv) => ParamValue::Multi(
                            mv.clone()
                                .into_iter()
                                .chain(vec![value.clone().into()].into_iter())
                                .collect(),
                        ),
                    },
                    None => ParamValue::Single(value.clone().into()),
                };

                acc.insert(key.clone().into(), new_value);
                acc
            },
        ))
    }
}

impl<T, S> FromIterator<(T, S)> for UrlQueryParams
where
    T: Into<String> + Clone,
    S: Into<String>,
{
    #[inline]
    fn from_iter<I: IntoIterator<Item = (T, S)>>(i: I) -> UrlQueryParams {
        UrlQueryParams(i.into_iter().fold(
            HashMap::<String, ParamValue>::new(),
            |mut acc, (key, value)| {
                let new_value: ParamValue = match acc.get(&key.clone().into()) {
                    Some(v) => match v {
                        &ParamValue::Single(ref sv) => {
                            ParamValue::Multi(vec![sv.clone(), value.into()])
                        }
                        &ParamValue::Multi(ref mv) => ParamValue::Multi(
                            mv.clone()
                                .into_iter()
                                .chain(vec![value.into()].into_iter())
                                .collect(),
                        ),
                    },
                    None => ParamValue::Single(value.into()),
                };

                acc.insert(key.into(), new_value);
                acc
            },
        ))
    }
}

impl From<url::Url> for UrlQueryParams {
    #[inline]
    fn from(v: url::Url) -> UrlQueryParams {
        v.query_pairs().into()
    }
}

impl<'a> From<url::form_urlencoded::Parse<'a>> for UrlQueryParams {
    #[inline]
    fn from(v: url::form_urlencoded::Parse<'a>) -> UrlQueryParams {
        v.into_owned().into()
    }
}

impl<'a> From<url::form_urlencoded::ParseIntoOwned<'a>> for UrlQueryParams {
    #[inline]
    fn from(v: url::form_urlencoded::ParseIntoOwned<'a>) -> UrlQueryParams {
        UrlQueryParams::from_iter(v)
    }
}

#[cfg(test)]
pub mod test_helpers {
    use super::*;
    use spectral::{AssertionFailure, Spec};

    pub trait ParamValueHelper<'s> {
        fn have_a_single_value(&mut self) -> Spec<'s, String>;
        fn have_multiple_values(&mut self) -> Spec<'s, Vec<String>>;
    }

    impl<'s> ParamValueHelper<'s> for Spec<'s, ParamValue> {
        fn have_a_single_value(&mut self) -> Spec<'s, String> {
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
                    .with_expected(format!("ParamValue to be: Single(String)"))
                    .with_actual(format!("ParamValue to be: {:?}", subject))
                    .fail();
            }
            unreachable!()
        }

        fn have_multiple_values(&mut self) -> Spec<'s, Vec<String>> {
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
                    .with_expected(format!("ParamValue to be: Multi(Vec<String>)"))
                    .with_actual(format!("ParamValue to be: {:?}", subject))
                    .fail();
            }
            unreachable!()
        }
    }
}
