// verification-helper: PROBLEM http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_A

use lib::ds::segtree::*;
use lib::io::*;

fn main() {
    let mut io = IO::new();
    let (n, q): (usize, usize) = io.scan();

    let inf = (1_u32 << 31) - 1;

    let mut st = SegmentTree::new(n, MonoidImpl(|| inf, |a, b| a.min(b)));

    for _ in 0..q {
        let [c, x, y]: [usize; 3] = io.scan();
        if c == 0 {
            st.with(x, |v| *v = y as u32);
        } else {
            io.println(st.ask(x, y + 1));
        }
    }
}
