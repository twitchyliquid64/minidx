use crate::{PaintParams, ParamVisOpts};
use minidx_core::Dtype;
use raqote::DrawTarget;

/// Identifies layer types which are native to minidx: needed to get around
/// trait conflicts between impls on M and (M,)
trait LayerMarker {}

impl<E: Dtype, const I: usize> LayerMarker for minidx_core::layers::Bias1d<E, I> {}
impl<E: Dtype, const I: usize, const O: usize> LayerMarker for minidx_core::layers::Dense<E, I, O> {}

/// Some composition of parameters which can be visualized.
pub trait VisualizableNetwork<DT> {
    type Params: std::fmt::Debug + Sized;

    fn visualize(&self, dt: &mut DT, opts: ParamVisOpts) -> (f32, f32);
}

impl<M: minidx_core::VisualizableLayer + LayerMarker> VisualizableNetwork<DrawTarget> for M
where
    DrawTarget: PaintParams<M::Params>,
{
    type Params = M::Params;

    fn visualize(&self, dt: &mut DrawTarget, opts: ParamVisOpts) -> (f32, f32) {
        let bounds = dt.layout_bounds(&opts);
        dt.paint_params(self.params(), &opts);
        (0.0, bounds.1)
    }
}

macro_rules! tuple_impls {
    ([$($name:ident),+] [$($idx:tt),*], $last:ident, [$($rev_tail:ident),*]) => {
        impl<
            $last:
            $(VisualizableNetwork<DrawTarget>, $rev_tail: )*
            VisualizableNetwork<DrawTarget>
        > VisualizableNetwork<DrawTarget> for ($($name,)+) {
            type Params = (
                $($name::Params,)+
            );

            fn visualize(&self, dt: &mut DrawTarget, mut opts: ParamVisOpts) -> (f32, f32) {
                opts.offset.0 += opts.module_padding.0;

                let offset = self.0.visualize(dt, opts.clone());
                $(let offset = self.$idx.visualize(dt, opts.update_cursor(offset));)*
                offset
            }
        }
    }
}

tuple_impls!([M1][], M1, []);
tuple_impls!([M1, M2][1], M2, [M1]);
tuple_impls!([M1, M2, M3] [1, 2], M3, [M2, M1]);
tuple_impls!([M1, M2, M3, M4] [1, 2, 3], M4, [M3, M2, M1]);
tuple_impls!([M1, M2, M3, M4, M5] [1, 2, 3, 4], M5, [M4, M3, M2, M1]);
tuple_impls!([M1, M2, M3, M4, M5, M6] [1, 2, 3, 4, 5], M6, [M5, M4, M3, M2, M1]);
tuple_impls!([M1, M2, M3, M4, M5, M6, M7] [1, 2, 3, 4, 5, 6], M7, [M6, M5, M4, M3, M2, M1]);
