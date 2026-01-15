use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use super::UserAttribute;

#[derive(Debug, Default)]
/// Mapping of attribute key to UserAttribute
/// TODO: rewrite to map from datafile::Attribute to value
pub(crate) struct UserAttributeMap(HashMap<String, UserAttribute>);

impl<'a> Deref for UserAttributeMap {
    type Target = HashMap<String, UserAttribute>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for UserAttributeMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
