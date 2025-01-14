mod dim;
pub use dim::{Dim, Const, ConstDim};
mod axes;
pub use axes::{Axes, Axes2, Axes3, Axes4, Axes5, Axes6, Axis, HasAxes};

mod broadcasts;
pub use broadcasts::{
    BroadcastShapeTo, BroadcastStridesTo, ReduceShape, ReduceShapeTo, ReduceStridesTo,
};

mod shape;
pub use shape::Array;
pub use shape::{ConstShape, HasShape, Shape};
pub use shape::{Rank0, Rank1, Rank2, Rank3, Rank4, Rank5, Rank6};

mod slice;
pub use slice::SliceShape;
