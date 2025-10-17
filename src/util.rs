use num_traits::Float;

pub fn normalize_float_arr<F: Float + std::iter::Sum, const N: usize>(arr: &mut [F; N]) {
    let norm = find_norm_arr(arr);
    if norm == F::zero() {
        arr.fill(F::zero());
        return;
    }
    for e in arr {
        *e = *e / norm;
    }
}

fn find_norm_arr<F: Float + std::iter::Sum>(arr: &[F]) -> F {
    arr.iter().map(|&x| x * x).sum::<F>().sqrt()
}
