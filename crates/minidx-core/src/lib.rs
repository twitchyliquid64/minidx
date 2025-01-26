mod dtypes;
pub use dtypes::*;
pub mod shapes;
pub use shapes::*;

mod iterate;
pub(crate) use iterate::*;

pub mod activation; // TODO: move to layers module.
pub mod bias1d; // TODO: move to layers module.
pub mod linear; // TODO: move to layers module.

pub type Error = ();

/// Some unit of computation that consumes `Input` and produces [Module::Output].
pub trait Module<X> {
    /// The type that this unit produces given `Input`.
    type Output;

    fn forward(&mut self, x: X) -> Result<Self::Output, Error>;
}

macro_rules! tuple_impls {
    ([$($name:ident),+] [$($idx:tt),+], $last:ident, [$($rev_tail:ident),*]) => {

        /*This macro expands like this for a 4-tuple:

        impl<
            Input: Tensor,

            // `$last:`
            D:

            // `$(Module::<$rev_tail ::Output>, $rev_tail: )+`
            Module<C ::Output>, C:
            Module<B ::Output>, B:
            Module<A ::Output>, A:

            Module<Input>
        > Module<Input> for (A, B, C, D) {
            type Output = D::Output;
            fn forward(&self, x: Input) -> Self::Output {
                let x = self.0.forward(x);
                let x = self.1.forward(x);
                let x = self.2.forward(x);
                let x = self.3.forward(x);
                x
            }
        }
        */
        impl<
            Input,
            $last:
            $(crate::Module::<$rev_tail ::Output>, $rev_tail: )*
            crate::Module<Input>
        > crate::Module<Input> for ($($name,)+) {
            type Output = $last ::Output;

            /// Calls forward sequentially on each module in the tuple.
            fn forward(&mut self, x: Input) -> Result<Self::Output, Error> {
                $(let x = self.$idx.forward(x)?;)+
                Ok(x)
            }
        }
    };
}

tuple_impls!([M1][0], M1, []);
tuple_impls!([M1, M2] [0, 1], M2, [M1]);
tuple_impls!([M1, M2, M3] [0, 1, 2], M3, [M2, M1]);
tuple_impls!([M1, M2, M3, M4] [0, 1, 2, 3], M4, [M3, M2, M1]);
tuple_impls!([M1, M2, M3, M4, M5] [0, 1, 2, 3, 4], M5, [M4, M3, M2, M1]);
tuple_impls!([M1, M2, M3, M4, M5, M6] [0, 1, 2, 3, 4, 5], M6, [M5, M4, M3, M2, M1]);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_forward() {
        let mut network = (
            linear::Dense::<f32, 2, 3>::default(),
            activation::Activation::Relu,
            bias1d::Bias1d::<f32, 3>::default(),
        );
        network.0.weights[0][0] = 2.5;
        network.2.bias.iter_mut().for_each(|x| *x += 1.0);

        let output = network.forward([1.0, 2.0]);

        assert_eq!(output, Ok([3.5, 1.0, 1.0]));
    }
}
