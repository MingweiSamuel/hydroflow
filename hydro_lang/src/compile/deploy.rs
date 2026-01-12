use std::collections::HashMap;
use std::io::Error;
use std::marker::PhantomData;
use std::pin::Pin;

use bytes::{Bytes, BytesMut};
use futures::{Sink, Stream};
use proc_macro2::Span;
use serde::Serialize;
use serde::de::DeserializeOwned;
use slotmap::{SecondaryMap, SlotMap, SparseSecondaryMap};
use stageleft::QuotedWithContext;

use super::built::build_inner;
use super::compiled::CompiledFlow;
use super::deploy_provider::{
    ClusterSpec, Deploy, ExternalSpec, IntoProcessSpec, Node, ProcessSpec, RegisterPort,
};
use super::ir::HydroRoot;
use crate::live_collections::stream::{Ordering, Retries};
use crate::location::dynamic::LocationId;
use crate::location::external_process::{
    ExternalBincodeBidi, ExternalBincodeSink, ExternalBincodeStream, ExternalBytesPort,
};
use crate::location::{Cluster, External, Location, LocationKey, LocationType, Process};
use crate::staging_util::Invariant;

pub struct DeployFlow<'a, D>
where
    D: Deploy<'a>,
{
    pub(super) ir: Vec<HydroRoot>,

    /// All locations (Process, Cluster, External).
    pub(super) locations: SlotMap<LocationKey, LocationType>,
    /// Human string name for all locations.
    pub(super) location_names: SecondaryMap<LocationKey, String>,

    pub(super) processes: SparseSecondaryMap<LocationKey, D::Process>,
    pub(super) clusters: SparseSecondaryMap<LocationKey, D::Cluster>,
    pub(super) externals: SparseSecondaryMap<LocationKey, D::External>,

    pub(super) _phantom: Invariant<'a, D>,
}

impl<'a, D: Deploy<'a>> DeployFlow<'a, D> {
    pub fn ir(&self) -> &Vec<HydroRoot> {
        &self.ir
    }

    pub fn with_process<P>(
        mut self,
        process: &Process<P>,
        spec: impl IntoProcessSpec<'a, D>,
    ) -> Self {
        self.processes.insert(
            process.key,
            spec.into_process_spec()
                .build(process.key, &self.location_names[process.key]),
        );
        self
    }

    pub fn with_remaining_processes<S: IntoProcessSpec<'a, D> + 'a>(
        mut self,
        spec: impl Fn() -> S,
    ) -> Self {
        for (loc_key, &loc_type) in self.locations.iter() {
            if LocationType::Process == loc_type && !self.processes.contains_key(loc_key) {
                self.processes.insert(
                    loc_key,
                    spec()
                        .into_process_spec()
                        .build(loc_key, &self.location_names[loc_key]),
                );
            }
        }
        self
    }

    pub fn with_cluster<C>(mut self, cluster: &Cluster<C>, spec: impl ClusterSpec<'a, D>) -> Self {
        self.clusters.insert(
            cluster.key,
            spec.build(cluster.key, &self.location_names[cluster.key]),
        );
        self
    }

    pub fn with_remaining_clusters<S: ClusterSpec<'a, D> + 'a>(
        mut self,
        spec: impl Fn() -> S,
    ) -> Self {
        for (loc_key, &loc_type) in self.locations.iter() {
            if LocationType::Cluster == loc_type && !self.clusters.contains_key(loc_key) {
                self.clusters.insert(
                    loc_key,
                    spec().build(loc_key, &self.location_names[loc_key]),
                );
            }
        }
        self
    }

    pub fn with_external<P>(
        mut self,
        external: &External<P>,
        spec: impl ExternalSpec<'a, D>,
    ) -> Self {
        self.externals.insert(
            external.key,
            spec.build(external.key, &self.location_names[external.key]),
        );
        self
    }

    pub fn with_remaining_externals<S: ExternalSpec<'a, D> + 'a>(
        mut self,
        spec: impl Fn() -> S,
    ) -> Self {
        for (loc_key, &loc_type) in self.locations.iter() {
            if LocationType::External == loc_type && !self.clusters.contains_key(loc_key) {
                self.externals.insert(
                    loc_key,
                    spec().build(loc_key, &self.location_names[loc_key]),
                );
            }
        }
        self
    }

    /// Compiles the flow into DFIR ([`dfir_lang::graph::DfirGraph`]) without networking.
    /// Useful for generating Mermaid diagrams of the DFIR.
    ///
    /// (This returned DFIR will not compile due to the networking missing).
    pub fn preview_compile(&mut self) -> CompiledFlow<'a, ()> {
        // NOTE: `build_inner` does not actually mutate the IR, but `&mut` is required
        // only because the shared traversal logic requires it
        CompiledFlow {
            dfir: build_inner::<D>(&mut self.ir),
            extra_stmts: SecondaryMap::new(),
            _phantom: PhantomData,
        }
    }
}

impl<'a, D: Deploy<'a>> DeployFlow<'a, D> {
    /// Compiles the flow into DFIR ([`dfir_lang::graph::DfirGraph`]) including networking.
    ///
    /// (This does not compile the DFIR itself, instead use [`Self::deploy`] to compile & deploy the DFIR).
    pub fn compile(&mut self) -> CompiledFlow<'a, D::GraphId> {
        let mut seen_tees: HashMap<_, _> = HashMap::new();
        let mut extra_stmts = SecondaryMap::new();
        self.ir.iter_mut().for_each(|leaf| {
            leaf.compile_network::<D>(
                &mut extra_stmts,
                &mut seen_tees,
                &self.processes,
                &self.clusters,
                &self.externals,
            );
        });

        CompiledFlow {
            dfir: build_inner::<D>(&mut self.ir),
            extra_stmts,
            _phantom: PhantomData,
        }
    }

    /// Creates the variables for cluster IDs and adds them into `extra_stmts`.
    fn cluster_id_stmts(&self, extra_stmts: &mut SecondaryMap<LocationKey, Vec<syn::Stmt>>) {
        let mut all_clusters_sorted = self.clusters.keys().collect::<Vec<_>>();
        all_clusters_sorted.sort();

        for c_key in all_clusters_sorted {
            let self_id_ident = syn::Ident::new(
                &format!("__hydro_lang_cluster_self_id_{:?}", c_key),
                Span::call_site(),
            );
            let self_id_expr = D::cluster_self_id().splice_untyped();
            extra_stmts
                .entry(c_key)
                .expect("location was removed")
                .or_default()
                .push(syn::parse_quote! {
                    let #self_id_ident = &*Box::leak(Box::new(#self_id_expr));
                });

            for other_location in self.processes.keys().chain(self.clusters.keys()) {
                let other_id_ident = syn::Ident::new(
                    &format!("__hydro_lang_cluster_ids_{:?}", c_key),
                    Span::call_site(),
                );
                let other_id_expr = D::cluster_ids(c_key).splice_untyped();
                extra_stmts
                    .entry(other_location)
                    .expect("location was removed")
                    .or_default()
                    .push(syn::parse_quote! {
                        let #other_id_ident = #other_id_expr;
                    });
            }
        }
    }

    /// Compiles and deploys the flow.
    ///
    /// Rough outline of steps:
    /// * Compiles the Hydro into DFIR.
    /// * Instantiates nodes as configured.
    /// * Compiles the corresponding DFIR into binaries for nodes as needed.
    /// * Connects up networking as needed.
    #[must_use]
    pub fn deploy(mut self, env: &mut D::InstantiateEnv) -> DeployResult<'a, D> {
        let CompiledFlow {
            dfir,
            mut extra_stmts,
            _phantom,
        } = self.compile();

        let mut compiled = dfir;
        self.cluster_id_stmts(&mut extra_stmts);
        let mut meta = D::Meta::default();

        let (mut processes, mut clusters, mut externals) = (
            std::mem::take(&mut self.processes)
                .into_iter()
                .filter_map(|(node_key, node)| {
                    if let Some(ir) = compiled.remove(node_key) {
                        node.instantiate(
                            env,
                            &mut meta,
                            ir,
                            extra_stmts.remove(node_key).unwrap_or_default(),
                        );
                        Some((node_key, node))
                    } else {
                        None
                    }
                })
                .collect::<SparseSecondaryMap<_, _>>(),
            std::mem::take(&mut self.clusters)
                .into_iter()
                .filter_map(|(cluster_key, cluster)| {
                    if let Some(ir) = compiled.remove(cluster_key) {
                        cluster.instantiate(
                            env,
                            &mut meta,
                            ir,
                            extra_stmts.remove(cluster_key).unwrap_or_default(),
                        );
                        Some((cluster_key, cluster))
                    } else {
                        None
                    }
                })
                .collect::<SparseSecondaryMap<_, _>>(),
            std::mem::take(&mut self.externals)
                .into_iter()
                .map(|(external_key, external)| {
                    external.instantiate(
                        env,
                        &mut meta,
                        Default::default(),
                        extra_stmts.remove(external_key).unwrap_or_default(),
                    );
                    (external_key, external)
                })
                .collect::<SparseSecondaryMap<_, _>>(),
        );

        for node in processes.values_mut() {
            node.update_meta(&meta);
        }

        for cluster in clusters.values_mut() {
            cluster.update_meta(&meta);
        }

        for external in externals.values_mut() {
            external.update_meta(&meta);
        }

        let mut seen_tees_connect = HashMap::new();
        self.ir.iter_mut().for_each(|leaf| {
            leaf.connect_network(&mut seen_tees_connect);
        });

        DeployResult {
            processes,
            clusters,
            externals,
            location_names: self.location_names,
        }
    }
}

pub struct DeployResult<'a, D: Deploy<'a>> {
    processes: SparseSecondaryMap<LocationKey, D::Process>,
    clusters: SparseSecondaryMap<LocationKey, D::Cluster>,
    externals: SparseSecondaryMap<LocationKey, D::External>,
    location_names: SecondaryMap<LocationKey, String>,
}

impl<'a, D: Deploy<'a>> DeployResult<'a, D> {
    pub fn get_process<P>(&self, p: &Process<P>) -> &D::Process {
        let key = match p.id() {
            LocationId::Process(key) => key,
            _ => panic!("Process ID expected"),
        };

        self.processes.get(key).unwrap()
    }

    pub fn get_cluster<C>(&self, c: &Cluster<'a, C>) -> &D::Cluster {
        let key: LocationKey = match c.id() {
            LocationId::Cluster(key) => key,
            _ => panic!("Cluster ID expected"),
        };

        self.clusters.get(key).unwrap()
    }

    pub fn get_all_clusters(&self) -> impl Iterator<Item = (LocationId, String, &D::Cluster)> {
        self.clusters.iter().map(|(key, c)| {
            (
                LocationId::Cluster(key),
                self.location_names.get(key).unwrap().clone(),
                c,
            )
        })
    }

    pub fn get_all_processes(&self) -> impl Iterator<Item = (LocationId, String, &D::Process)> {
        self.processes.iter().map(|(key, p)| {
            (
                LocationId::Process(key),
                self.location_names.get(key).unwrap().clone(),
                p,
            )
        })
    }

    pub fn get_external<P>(&self, p: &External<P>) -> &D::External {
        self.externals.get(p.key).unwrap()
    }

    pub fn raw_port<M>(&self, port: ExternalBytesPort<M>) -> D::ExternalRawPort {
        self.externals
            .get(port.process_key)
            .unwrap()
            .raw_port(port.port_id)
    }

    #[deprecated(note = "use `connect` instead")]
    pub async fn connect_bytes<M>(
        &self,
        port: ExternalBytesPort<M>,
    ) -> (
        Pin<Box<dyn Stream<Item = Result<BytesMut, Error>>>>,
        Pin<Box<dyn Sink<Bytes, Error = Error>>>,
    ) {
        self.connect(port).await
    }

    #[deprecated(note = "use `connect` instead")]
    pub async fn connect_sink_bytes<M>(
        &self,
        port: ExternalBytesPort<M>,
    ) -> Pin<Box<dyn Sink<Bytes, Error = Error>>> {
        self.connect(port).await.1
    }

    pub async fn connect_bincode<
        InT: Serialize + 'static,
        OutT: DeserializeOwned + 'static,
        Many,
    >(
        &self,
        port: ExternalBincodeBidi<InT, OutT, Many>,
    ) -> (
        Pin<Box<dyn Stream<Item = OutT>>>,
        Pin<Box<dyn Sink<InT, Error = Error>>>,
    ) {
        self.externals
            .get(port.process_key)
            .unwrap()
            .as_bincode_bidi(port.port_id)
            .await
    }

    #[deprecated(note = "use `connect` instead")]
    pub async fn connect_sink_bincode<T: Serialize + DeserializeOwned + 'static, Many>(
        &self,
        port: ExternalBincodeSink<T, Many>,
    ) -> Pin<Box<dyn Sink<T, Error = Error>>> {
        self.connect(port).await
    }

    #[deprecated(note = "use `connect` instead")]
    pub async fn connect_source_bytes(
        &self,
        port: ExternalBytesPort,
    ) -> Pin<Box<dyn Stream<Item = Result<BytesMut, Error>>>> {
        self.connect(port).await.0
    }

    #[deprecated(note = "use `connect` instead")]
    pub async fn connect_source_bincode<
        T: Serialize + DeserializeOwned + 'static,
        O: Ordering,
        R: Retries,
    >(
        &self,
        port: ExternalBincodeStream<T, O, R>,
    ) -> Pin<Box<dyn Stream<Item = T>>> {
        self.connect(port).await
    }

    pub async fn connect<'b, P: ConnectableAsync<&'b Self>>(
        &'b self,
        port: P,
    ) -> <P as ConnectableAsync<&'b Self>>::Output {
        port.connect(self).await
    }
}

pub trait ConnectableAsync<Ctx> {
    type Output;

    fn connect(self, ctx: Ctx) -> impl Future<Output = Self::Output>;
}

impl<'a, D: Deploy<'a>, M> ConnectableAsync<&DeployResult<'a, D>> for ExternalBytesPort<M> {
    type Output = (
        Pin<Box<dyn Stream<Item = Result<BytesMut, Error>>>>,
        Pin<Box<dyn Sink<Bytes, Error = Error>>>,
    );

    async fn connect(self, ctx: &DeployResult<'a, D>) -> Self::Output {
        ctx.externals
            .get(self.process_key)
            .unwrap()
            .as_bytes_bidi(self.port_id)
            .await
    }
}

impl<'a, D: Deploy<'a>, T: DeserializeOwned + 'static, O: Ordering, R: Retries>
    ConnectableAsync<&DeployResult<'a, D>> for ExternalBincodeStream<T, O, R>
{
    type Output = Pin<Box<dyn Stream<Item = T>>>;

    async fn connect(self, ctx: &DeployResult<'a, D>) -> Self::Output {
        ctx.externals
            .get(self.process_key)
            .unwrap()
            .as_bincode_source(self.port_id)
            .await
    }
}

impl<'a, D: Deploy<'a>, T: Serialize + 'static, Many> ConnectableAsync<&DeployResult<'a, D>>
    for ExternalBincodeSink<T, Many>
{
    type Output = Pin<Box<dyn Sink<T, Error = Error>>>;

    async fn connect(self, ctx: &DeployResult<'a, D>) -> Self::Output {
        ctx.externals
            .get(self.process_key)
            .unwrap()
            .as_bincode_sink(self.port_id)
            .await
    }
}
