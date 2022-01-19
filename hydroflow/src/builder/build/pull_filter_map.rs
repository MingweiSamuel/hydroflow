use super::{PullBuild, PullBuildBase};

use crate::scheduled::handoff::HandoffList;

macro_rules! pull_build {
    (
        struct_name => $struct_name:ident <
            $(
                $struct_type_param:ident
            ),+
        >,
        struct_where => {
            $(
                $struct_where_type:ty :
                ($struct_where_bound_first:path) $( + ($struct_where_bound_rest:path) )*,
            )*
        },
        struct_body => {
            $(
                $struct_body_field:ident : $struct_body_type:ty,
            )+
        },
        struct_phantom => $struct_phantom:ty,

        impl_types => <
            $(
                $impl_type_param:ident
            ),+
        >,
        impl_where => {
            $(
                $impl_where_type:ty :
                ($impl_where_bound_first:path) $( + ($impl_where_bound_rest:path) )*,
            )*
        },

        build_out => $build_out:ty,
    ) => {
        pub struct $struct_name < $( $struct_type_param ),+ >
        where
            $(
                $struct_where_type :
                $struct_where_bound_first $( + $struct_where_bound_rest )*,
            )*
        {
            $(
                $struct_body_field : $struct_body_type,
            )+
            _phantom: std::marker::PhantomData< $struct_phantom >,
        }

        impl < $( $impl_type_param ),+ > $struct_name < $( $struct_type_param ),+ >
        where
            $(
                $struct_where_type :
                $struct_where_bound_first $( + $impl_where_bound_rest )*,
            )*
            $(
                $impl_where_type :
                $impl_where_bound_first $( + $struct_where_bound_rest )*,
            )*
        {
            pub fn new(
                $(
                    $struct_body_field : $struct_body_type,
                )+
            ) -> Self {
                Self {
                    $(
                        $struct_body_field,
                    )+
                    _phantom: std::marker::PhantomData,
                }
            }
        }
    };
}

pull_build! {
    struct_name => FilterMapPullBuild<Prev, Func>,
    struct_where => {
        Prev: (PullBuild),
    },
    struct_body => {
        prev: Prev,
        func: Func,
    },
    struct_phantom => (),

    impl_types => <Prev, Func, Out>,
    impl_where => {
        Func: (FnMut(Prev::ItemOut) -> Option<Out>),
    },

    build_out =>
        std::iter::FilterMap<Prev::Build<'slf, 'hof>, impl FnMut(Prev::ItemOut) -> Option<Out>>,
//     build_fn<'slf, 'hof>(self, handoffs) => {
//         self.prev.build(handoffs).filter_map(|x| (self.func)(x))
//     },
}

// pub struct FilterMapPullBuild<Prev, Func>
// where
//     Prev: PullBuild,
// {
//     prev: Prev,
//     func: Func,
// }
// impl<Prev, Func, Out> FilterMapPullBuild<Prev, Func>
// where
//     Prev: PullBuild,
//     Func: FnMut(Prev::ItemOut) -> Option<Out>,
// {
//     pub fn new(prev: Prev, func: Func) -> Self {
//         Self { prev, func }
//     }
// }

// #[allow(type_alias_bounds)]
// type PullBuildImpl<'slf, 'hof, Prev, Func, Out>
// where
//     Prev: PullBuild,
// = std::iter::FilterMap<Prev::Build<'slf, 'hof>, impl FnMut(Prev::ItemOut) -> Option<Out>>;

// impl<Prev, Func, Out> PullBuildBase for FilterMapPullBuild<Prev, Func>
// where
//     Prev: PullBuild,
//     Func: FnMut(Prev::ItemOut) -> Option<Out>,
// {
//     type ItemOut = Out;
//     type Build<'slf, 'hof> = PullBuildImpl<'slf, 'hof, Prev, Func, Out>;
// }

// impl<Prev, Func, Out> PullBuild for FilterMapPullBuild<Prev, Func>
// where
//     Prev: PullBuild,
//     Func: FnMut(Prev::ItemOut) -> Option<Out>,
// {
//     type InputHandoffs = Prev::InputHandoffs;

//     fn build<'slf, 'hof>(
//         &'slf mut self,
//         handoffs: <Self::InputHandoffs as HandoffList>::RecvCtx<'hof>,
//     ) -> Self::Build<'slf, 'hof> {
//         self.prev.build(handoffs).filter_map(|x| (self.func)(x))
//     }
// }
