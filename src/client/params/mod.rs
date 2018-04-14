use std::borrow::Cow;
#[cfg(test)]
mod spec;

use std::collections::HashMap;
use std::iter::FromIterator;
use std::ops::Deref;

use url;

#[derive(Debug, Clone, PartialEq)]
pub enum ParamValue<'a> {
    Single(Cow<'a, str>),
    Multi(Vec<Cow<'a, str>>),
}

impl<'a> ParamValue<'a> {
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

    pub fn single(&self) -> Option<&Cow<'a, str>> {
        match self {
            &ParamValue::Single(ref s) => Some(s),
            _ => None,
        }
    }

    pub fn multi(&self) -> Option<&Vec<Cow<'a, str>>> {
        match self {
            &ParamValue::Multi(ref v) => Some(v),
            _ => None,
        }
    }
}

impl<'a, T> From<T> for ParamValue<'a>
where
    T: Into<Cow<'a, str>>,
{
    #[inline]
    fn from(v: T) -> ParamValue<'a> {
        ParamValue::Single(v.into())
    }
}

impl<'a, T> FromIterator<T> for ParamValue<'a>
where
    T: Into<Cow<'a, str>>,
{
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let as_vec: Vec<Cow<'a, str>> = iter.into_iter().map(|v| v.into()).collect();
        let count = as_vec.len();
        match count {
            0 => ParamValue::Single("".into()),
            1 => ParamValue::Single(as_vec.first().unwrap().clone()),
            _ => ParamValue::Multi(as_vec),
        }
    }
}

impl<'a> IntoIterator for ParamValue<'a> {
    type Item = Cow<'a, str>;
    type IntoIter = ::std::vec::IntoIter<Cow<'a, str>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        match self {
            ParamValue::Single(v) => vec![v].into_iter(),
            ParamValue::Multi(v) => v.into_iter(),
        }
    }
}

pub struct UrlQueryParams<'a>(HashMap<Cow<'a, str>, ParamValue<'a>>);

impl<'a> UrlQueryParams<'a> {
    #[inline]
    pub fn get<T: Into<Cow<'a, str>>>(&self, key: T) -> Option<&ParamValue<'a>> {
        self.0.get(&key.into())
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<'a> IntoIterator for UrlQueryParams<'a> {
    type Item = (Cow<'a, str>, Cow<'a, str>);
    type IntoIter = ::std::vec::IntoIter<(Cow<'a, str>, Cow<'a, str>)>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0
            .into_iter()
            .map(|(k, v): (Cow<'a, str>, ParamValue<'a>)| {
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

impl<'a> Deref for UrlQueryParams<'a> {
    type Target = HashMap<Cow<'a, str>, ParamValue<'a>>;
    fn deref(&self) -> &HashMap<Cow<'a, str>, ParamValue<'a>> {
        &self.0
    }
}

impl<'a, T, S> FromIterator<(T, S)> for UrlQueryParams<'a>
where
    T: Into<Cow<'a, str>> + Clone,
    S: Into<Cow<'a, str>>,
{
    #[inline]
    fn from_iter<I: IntoIterator<Item = (T, S)>>(i: I) -> UrlQueryParams<'a> {
        UrlQueryParams(i.into_iter().fold(
            HashMap::<Cow<'a, str>, ParamValue<'a>>::new(),
            |mut acc, (key, value)| {
                let new_value: ParamValue = match acc.get(&key.clone().into()) {
                    None => ParamValue::Single(value.into().clone()),
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
                };

                acc.insert(key.into(), new_value);
                acc
            },
        ))
    }
}

impl<'a> From<&'a url::Url> for UrlQueryParams<'a> {
    #[inline]
    fn from(v: &'a url::Url) -> UrlQueryParams {
        v.query_pairs().into()
    }
}

impl<'a> From<url::form_urlencoded::Parse<'a>> for UrlQueryParams<'a> {
    #[inline]
    fn from(v: url::form_urlencoded::Parse<'a>) -> UrlQueryParams {
        UrlQueryParams::from_iter(v)
    }
}
