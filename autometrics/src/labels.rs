use crate::constants::*;
use std::ops::Deref;

pub(crate) type Label = (&'static str, &'static str);
type ResultAndReturnTypeLabels = (&'static str, Option<&'static str>);

pub struct CounterLabels {
    pub function: &'static str,
    pub module: &'static str,
    pub caller: &'static str,
    pub result: Option<ResultAndReturnTypeLabels>,
}

impl CounterLabels {
    pub fn to_vec(&self) -> Vec<Label> {
        let mut labels = vec![
            (FUNCTION_KEY, self.function),
            (MODULE_KEY, self.module),
            (CALLER_KEY, self.caller),
        ];
        if let Some((result, return_value_type)) = self.result {
            labels.push((RESULT_KEY, result));
            if let Some(return_value_type) = return_value_type {
                labels.push((result, return_value_type));
            }
        }

        labels
    }
}

pub struct HistogramLabels {
    pub function: &'static str,
    pub module: &'static str,
}

impl HistogramLabels {
    pub fn to_array(&self) -> [Label; 2] {
        [(FUNCTION_KEY, self.function), (MODULE_KEY, self.module)]
    }
}

pub struct GaugeLabels {
    pub function: &'static str,
    pub module: &'static str,
}

impl GaugeLabels {
    pub fn to_array(&self) -> [Label; 2] {
        [(FUNCTION_KEY, self.function), (MODULE_KEY, self.module)]
    }
}

// The following is a convoluted way to figure out if the return type resolves to a Result
// or not. We cannot simply parse the code using syn to figure out if it's a Result
// because syn doesn't do type resolution and thus would count any renamed version
// of Result as a different type. Instead, we define two traits with intentionally
// conflicting method names and use a trick based on the order in which Rust resolves
// method names to return a different value based on whether the return value is
// a Result or anything else.
// This approach is based on dtolnay's answer to this question:
// https://users.rust-lang.org/t/how-to-check-types-within-macro/33803/5
// and this answer explains why it works:
// https://users.rust-lang.org/t/how-to-check-types-within-macro/33803/8

pub trait GetLabelsFromResult {
    fn __autometrics_get_labels(&self) -> Option<ResultAndReturnTypeLabels> {
        None
    }
}

impl<T, E> GetLabelsFromResult for Result<T, E> {
    fn __autometrics_get_labels(&self) -> Option<ResultAndReturnTypeLabels> {
        match self {
            Ok(ok) => Some((OK_KEY, ok.__autometrics_static_str())),
            Err(err) => Some((ERROR_KEY, err.__autometrics_static_str())),
        }
    }
}

pub enum LabelArray {
    Three([Label; 3]),
    Four([Label; 4]),
    Five([Label; 5]),
}

impl Deref for LabelArray {
    type Target = [Label];

    fn deref(&self) -> &Self::Target {
        match self {
            LabelArray::Three(l) => l,
            LabelArray::Four(l) => l,
            LabelArray::Five(l) => l,
        }
    }
}

pub trait GetLabels {
    fn __autometrics_get_labels(&self) -> Option<ResultAndReturnTypeLabels> {
        None
    }
}

/// Implement the given trait for &T and all primitive types.
macro_rules! impl_trait_for_types {
    ($trait:ident) => {
        impl<T> $trait for &T {}
        impl $trait for i8 {}
        impl $trait for i16 {}
        impl $trait for i32 {}
        impl $trait for i64 {}
        impl $trait for i128 {}
        impl $trait for isize {}
        impl $trait for u8 {}
        impl $trait for u16 {}
        impl $trait for u32 {}
        impl $trait for u64 {}
        impl $trait for u128 {}
        impl $trait for usize {}
        impl $trait for f32 {}
        impl $trait for f64 {}
        impl $trait for char {}
        impl $trait for bool {}
        impl $trait for str {}
        impl $trait for () {}
        impl<A> $trait for (A,) {}
        impl<A, B> $trait for (A, B) {}
        impl<A, B, C> $trait for (A, B, C) {}
        impl<A, B, C, D> $trait for (A, B, C, D) {}
        impl<A, B, C, D, E> $trait for (A, B, C, D, E) {}
        impl<A, B, C, D, E, F> $trait for (A, B, C, D, E, F) {}
        impl<A, B, C, D, E, F, G> $trait for (A, B, C, D, E, F, G) {}
        impl<A, B, C, D, E, F, G, H> $trait for (A, B, C, D, E, F, G, H) {}
        impl<A, B, C, D, E, F, G, H, I> $trait for (A, B, C, D, E, F, G, H, I) {}
        impl<A, B, C, D, E, F, G, H, I, J> $trait for (A, B, C, D, E, F, G, H, I, J) {}
        impl<A, B, C, D, E, F, G, H, I, J, K> $trait for (A, B, C, D, E, F, G, H, I, J, K) {}
        impl<A, B, C, D, E, F, G, H, I, J, K, L> $trait for (A, B, C, D, E, F, G, H, I, J, K, L) {}
    };
}

impl_trait_for_types!(GetLabels);

pub trait GetStaticStrFromIntoStaticStr<'a> {
    fn __autometrics_static_str(&'a self) -> Option<&'static str>;
}

impl<'a, T: 'a> GetStaticStrFromIntoStaticStr<'a> for T
where
    &'static str: From<&'a T>,
{
    fn __autometrics_static_str(&'a self) -> Option<&'static str> {
        Some(self.into())
    }
}

pub trait GetStaticStr {
    fn __autometrics_static_str(&self) -> Option<&'static str> {
        None
    }
}
impl_trait_for_types!(GetStaticStr);
