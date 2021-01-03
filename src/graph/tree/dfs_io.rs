pub use super::*;
use crate::ds::bitset::*;

pub fn dfs_io<G: Graph, FI: FnMut(usize, usize), FO: FnMut(usize, usize)>(
	g: &G,
	s: usize,
	mut fi: FI,
	mut fo: FO,
) {
	let mut togo = vec![(s, !0)];
	while let Some((v, par)) = togo.pop() {
		if v.get_bit(31) {
			fo(!v, par);
		} else {
			fi(v, par);
			togo.push((!v, par));
			g.adj(v, |w| {
				if w != par {
					togo.push((w, v));
				}
			});
		}
	}
}
