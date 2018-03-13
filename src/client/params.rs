use std::collections::HashMap;

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

pub fn params_into_hash(params: &Vec<(String, String)>) -> HashMap<String, ParamValue> {
    params
        .into_iter()
        .fold(HashMap::new(), |mut acc, &(ref key, ref value)| {
            let new_value: ParamValue = match acc.get(key) {
                Some(v) => match v {
                    &ParamValue::Single(ref sv) => {
                        ParamValue::Multi(vec![sv.clone(), value.clone()])
                    }
                    &ParamValue::Multi(ref mv) => ParamValue::Multi(
                        mv.clone()
                            .into_iter()
                            .chain(vec![value.clone()].into_iter())
                            .collect(),
                    ),
                },
                None => ParamValue::Single(value.clone()),
            };

            acc.insert(key.clone(), new_value);
            acc
        })
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
