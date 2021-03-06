use crate::ds::bitset::*;
pub use crate::graph::weighted::*;

/// f: (v, par, w)
pub fn dfs<G: WGraph>(g: &G, s: usize, mut f: impl FnMut(usize, usize, G::W))
where
    G::W: Copy + Default,
{
    let mut visited = new_bitset(g.len());
    visited.set_bit(s);
    dfs_impl(g, s, !0, G::W::default(), &mut visited, &mut f);
}

fn dfs_impl<G: WGraph>(
    g: &G,
    v: usize,
    par: usize,
    w: G::W,
    visited: &mut [u32],
    f: &mut impl FnMut(usize, usize, G::W),
) where G::W: Copy {
    f(v, par, w);
    g.adj_w(v, |to, w| {
        if visited.set_bit(to) {
            dfs_impl(g, to, v, w, visited, f);
        }
    });
}
