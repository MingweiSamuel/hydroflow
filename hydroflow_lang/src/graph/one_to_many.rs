//! Helper datastructure for de-duplicating serialization/deserialization of parent-child AKA
//! one-to-many relationships.

use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

#[repr(transparent)]
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
struct ParentChildSerde<O2M>(pub O2M);

/// See module-level description.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(from = "ParentChildSerde<O2M>", into = "ParentChildSerde<O2M>")]
pub struct OneToMany<One, Many, O2M, M2O>
where
    One: Copy,
    Many: Copy,
    O2M: Clone,
    M2O: Clone + FromIterator<(Many, One)>,
    for<'a> &'a O2M: IntoIterator<Item = (One, &'a Vec<Many>)>,
{
    _phantom: PhantomData<(One, Many)>,
    /// One-to-many mapping, keys of `One`, values of `Vec<Many>`.
    pub one_to_many: O2M,
    /// Many-to-one mapping, keys of `Many`, values of `One`.
    pub many_to_one: M2O,
}

impl<One, Many, O2M, M2O> Default for OneToMany<One, Many, O2M, M2O>
where
    One: Copy,
    Many: Copy,
    O2M: Clone + Default,
    M2O: Clone + FromIterator<(Many, One)> + Default,
    for<'a> &'a O2M: IntoIterator<Item = (One, &'a Vec<Many>)>,
{
    fn default() -> Self {
        let (_phantom, parent_children, child_parent) = Default::default();
        Self {
            _phantom,
            one_to_many: parent_children,
            many_to_one: child_parent,
        }
    }
}

impl<One, Many, O2M, M2O> From<ParentChildSerde<O2M>> for OneToMany<One, Many, O2M, M2O>
where
    One: Copy,
    Many: Copy,
    O2M: Clone,
    M2O: Clone + FromIterator<(Many, One)>,
    for<'a> &'a O2M: IntoIterator<Item = (One, &'a Vec<Many>)>,
{
    fn from(value: ParentChildSerde<O2M>) -> Self {
        let ParentChildSerde(parent_children) = value;
        let child_parent: M2O = (&parent_children)
            .into_iter()
            .flat_map(|(p, cs)| cs.iter().map(move |&c| (c, p)))
            .collect();
        Self {
            _phantom: PhantomData,
            one_to_many: parent_children,
            many_to_one: child_parent,
        }
    }
}

impl<One, Many, O2M, M2O> From<OneToMany<One, Many, O2M, M2O>> for ParentChildSerde<O2M>
where
    One: Copy,
    Many: Copy,
    O2M: Clone,
    M2O: Clone + FromIterator<(Many, One)>,
    for<'a> &'a O2M: IntoIterator<Item = (One, &'a Vec<Many>)>,
{
    fn from(value: OneToMany<One, Many, O2M, M2O>) -> Self {
        Self(value.one_to_many)
    }
}
