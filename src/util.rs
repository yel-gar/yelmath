use num_traits::Float;

pub fn normalize_float_arr<F: Float, const N: usize>(arr: &mut [F; N]) {
    let max = find_max_abs_arr(arr);
    for e in arr {
        *e = *e / max;
    }
}

fn find_max_abs_arr<F: Float>(arr: &[F]) -> F {
    let mut max = F::min_value();
    arr.iter().map(|v| v.abs()).for_each(|v| {
        if v > max {
            max = v;
        }
    });
    max
}
