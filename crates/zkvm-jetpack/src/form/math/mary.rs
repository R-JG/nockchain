use crate::form::mary::{MarySlice, MarySliceMut};
use crate::form::Belt;

#[inline(always)]
pub fn mary_weld(a: MarySlice, b: MarySlice, res: MarySliceMut) {
    assert_eq!(a.step, b.step);
    assert_eq!(res.len, a.len + b.len);
    let a_len = a.len as usize;
    let res_len = res.len as usize;
    let step = res.step as usize;
    res.dat[0..a_len * step].copy_from_slice(a.dat);
    res.dat[a_len * step..res_len * step].copy_from_slice(b.dat);
}

#[inline(always)]
pub fn mary_transpose(fpolys: MarySlice, offset: usize, res: &mut MarySliceMut) {
    let step = fpolys.step as usize;
    let len = fpolys.len as usize;

    let num_cols = step / offset;
    let num_rows = len;

    for i in 0..num_cols {
        for j in 0..num_rows {
            for k in 0..offset {
                res.dat[offset * (i * num_rows + j) + k] =
                    fpolys.dat[offset * (j * num_cols + i) + k];
            }
        }
    }
}

#[inline(always)]
pub fn snag_as_bpoly(a: MarySlice, i: usize) -> &[Belt] {
    let step = a.step as usize;
    to_belts(&a.dat[step * i..(step * (i + 1))])
}

#[inline(always)]
pub fn to_belts(sli: &[u64]) -> &[Belt] {
    unsafe {
        let ptr = sli.as_ptr() as *const Belt;
        std::slice::from_raw_parts(ptr, sli.len())
    }
}
