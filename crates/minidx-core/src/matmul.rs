use crate::{Dim, Dtype};

pub trait MatMulImpl
where
    Self: Dtype,
{
    #[allow(clippy::too_many_arguments)]
    fn matmul<M: Dim, K: Dim, N: Dim>(
        dims: (M, K, N),
        accum: bool,
        ap: *const Self,
        a_strides: [usize; 2],
        bp: *const Self,
        b_strides: [usize; 2],
        cp: *mut Self,
        c_strides: [usize; 2],
    );
}

#[allow(unused)]
#[allow(clippy::too_many_arguments)]
fn naive_gemm<F: Dtype, M: Dim, K: Dim, N: Dim>(
    (m, k, n): (M, K, N),
    accum: bool,
    ap: *const F,
    a_strides: [usize; 2],
    bp: *const F,
    b_strides: [usize; 2],
    cp: *mut F,
    c_strides: [usize; 2],
) {
    //println!("naive_gemm()");
    for i_m in 0..m.size() {
        for i_k in 0..k.size() {
            for i_n in 0..n.size() {
                unsafe {
                    let a = *ap.add(a_strides[0] * i_m + a_strides[1] * i_k);
                    let b = *bp.add(b_strides[0] * i_k + b_strides[1] * i_n);
                    let c = cp.add(c_strides[0] * i_m + c_strides[1] * i_n);

                    // println!(
                    //     "({},{},{}) {:?} * {:?} = {:?} ({}, {}, {})",
                    //     i_m,
                    //     i_k,
                    //     i_n,
                    //     a,
                    //     b,
                    //     a * b,
                    //     a_strides[0] * i_m + a_strides[1] * i_k,
                    //     b_strides[0] * i_k + b_strides[1] * i_n,
                    //     c_strides[0] * i_m + c_strides[1] * i_n
                    // );
                    if accum {
                        *c += a * b;
                    } else {
                        *c = a * b;
                    }
                }
            }
        }
    }
}

impl MatMulImpl for f32 {
    #[inline]
    fn matmul<M: Dim, K: Dim, N: Dim>(
        (m, k, n): (M, K, N),
        accum: bool,
        ap: *const f32,
        astr: [usize; 2],
        bp: *const f32,
        bstr: [usize; 2],
        cp: *mut f32,
        cstr: [usize; 2],
    ) {
        #[cfg(not(feature = "gemm"))]
        naive_gemm((m, k, n), accum, ap, astr, bp, bstr, cp, cstr);

        #[cfg(feature = "gemm")]
        unsafe {
            gemm::gemm(
                m.size(),
                n.size(),
                k.size(),
                cp,
                cstr[1] as isize,
                cstr[0] as isize,
                accum,
                ap,
                astr[1] as isize,
                astr[0] as isize,
                bp,
                bstr[1] as isize,
                bstr[0] as isize,
                if accum { 1.0 } else { 0.0 },
                1.0,
                false,
                false,
                false,
                gemm::Parallelism::Rayon((rayon::current_num_threads() / 2).max(2)),
            )
        }
    }
}

impl MatMulImpl for f64 {
    #[inline]
    fn matmul<M: Dim, K: Dim, N: Dim>(
        (m, k, n): (M, K, N),
        accum: bool,
        ap: *const f64,
        astr: [usize; 2],
        bp: *const f64,
        bstr: [usize; 2],
        cp: *mut f64,
        cstr: [usize; 2],
    ) {
        #[cfg(not(feature = "gemm"))]
        naive_gemm((m, k, n), accum, ap, astr, bp, bstr, cp, cstr);

        #[cfg(feature = "gemm")]
        unsafe {
            gemm::gemm(
                m.size(),
                n.size(),
                k.size(),
                cp,
                cstr[1] as isize,
                cstr[0] as isize,
                accum,
                ap,
                astr[1] as isize,
                astr[0] as isize,
                bp,
                bstr[1] as isize,
                bstr[0] as isize,
                if accum { 1.0 } else { 0.0 },
                1.0,
                false,
                false,
                false,
                gemm::Parallelism::Rayon((rayon::current_num_threads() / 2).max(2)),
            )
        }
    }
}
