use super::*;

/// return (min f, argmin f)
pub fn convex_min<T: PartialOrd>(
    mut l: Float,
    mut r: Float,
    e: Float,
    mut f: impl FnMut(Float) -> T,
) -> (T, Float) {
    const PHI: Float = 1.618_033_988_749_895;
    let k = ((r - l) / e).log(PHI) as u32 + 2;

    let mut ml = (PHI * l + r) / (1.0 + PHI);
    let mut mr = (l + PHI * r) / (1.0 + PHI);
    let mut yml = f(ml);
    let mut ymr = f(mr);

    for _ in 0..k {
        if yml < ymr {
            l = ml;
            ml = mr;
            yml = ymr;
            mr = (ml + PHI * r) / (1.0 + PHI);
            ymr = f(mr);
        } else {
            r = mr;
            mr = ml;
            ymr = yml;
            mr = (PHI * l + mr) / (1.0 + PHI);
            yml = f(ml);
        }
    }
    (yml, ml)
}
