use std::collections::BTreeMap;

use dfir_lang::graph::DfirGraph;
use slotmap::SecondaryMap;
use syn::Stmt;

use crate::location::{Location, LocationKey};
use crate::staging_util::Invariant;

pub struct CompiledFlow<'a, ID> {
    pub(super) dfir: SecondaryMap<LocationKey, DfirGraph>,
    pub(super) extra_stmts: SecondaryMap<LocationKey, Vec<Stmt>>,
    pub(super) _phantom: Invariant<'a, ID>,
}

impl<'a, ID> CompiledFlow<'a, ID> {
    pub fn dfir_for(&self, location: &impl Location<'a>) -> &DfirGraph {
        self.dfir.get(Location::id(location).key()).unwrap()
    }

    pub fn all_dfir(&self) -> &SecondaryMap<LocationKey, DfirGraph> {
        &self.dfir
    }
}
