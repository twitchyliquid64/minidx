mod dim;
pub use dim::{Const, ConstDim, Dim};
mod axes;
pub use axes::{Axes, Axes2, Axes3, Axes4, Axes5, Axes6, Axis, HasAxes};

mod broadcasts;
pub use broadcasts::{
    BroadcastShapeTo, BroadcastStridesTo, ReduceShape, ReduceShapeTo, ReduceStridesTo,
};

mod permutes;

mod replace_dim;

mod shape;
pub use shape::Array;
pub use shape::{ConstShape, HasShape, Shape};
pub use shape::{Rank0, Rank1, Rank2, Rank3, Rank4, Rank5, Rank6};

mod slice;
pub use slice::SliceShape;

/// Marker for shapes that can be converted using their concrete types.
pub trait RealizeShapeTo<Dst: Shape>: Shape {
    fn realized(&self) -> Option<Dst>;
}

impl<Src: Shape<Concrete = Dst::Concrete>, Dst: Shape> RealizeShapeTo<Dst> for Src {
    #[inline(always)]
    fn realized(&self) -> Option<Dst> {
        Dst::from_concrete(&self.concrete())
    }
}
