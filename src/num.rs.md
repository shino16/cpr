---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: src/zo.rs
    title: src/zo.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: src/alg/arith.rs
    title: src/alg/arith.rs
  - icon: ':heavy_check_mark:'
    path: src/dfa.rs
    title: src/dfa.rs
  - icon: ':warning:'
    path: src/draft/fpacc64.rs
    title: src/draft/fpacc64.rs
  - icon: ':warning:'
    path: src/ds/fenwick.rs
    title: src/ds/fenwick.rs
  - icon: ':warning:'
    path: src/fp/num.rs
    title: src/fp/num.rs
  - icon: ':warning:'
    path: src/graph/dijkstra.rs
    title: src/graph/dijkstra.rs
  - icon: ':warning:'
    path: src/graph/euler_tour.rs
    title: src/graph/euler_tour.rs
  - icon: ':heavy_check_mark:'
    path: src/graph/max_flow/edmonds_karp.rs
    title: src/graph/max_flow/edmonds_karp.rs
  - icon: ':warning:'
    path: src/graph/max_flow/edmonds_karp/edge.rs
    title: src/graph/max_flow/edmonds_karp/edge.rs
  - icon: ':heavy_check_mark:'
    path: src/graph/max_flow/ford_fulkerson.rs
    title: src/graph/max_flow/ford_fulkerson.rs
  - icon: ':warning:'
    path: src/graph/max_flow/ford_fulkerson/edge.rs
    title: src/graph/max_flow/ford_fulkerson/edge.rs
  - icon: ':warning:'
    path: src/graph/max_flow/ford_fulkerson/edges.rs
    title: src/graph/max_flow/ford_fulkerson/edges.rs
  - icon: ':heavy_check_mark:'
    path: src/graph/max_flow/hlpp.rs
    title: src/graph/max_flow/hlpp.rs
  - icon: ':warning:'
    path: src/graph/max_flow/hlpp/edge.rs
    title: src/graph/max_flow/hlpp/edge.rs
  - icon: ':warning:'
    path: src/graph/max_flow/push_relabel.rs
    title: src/graph/max_flow/push_relabel.rs
  - icon: ':question:'
    path: src/int.rs
    title: src/int.rs
  - icon: ':warning:'
    path: src/int/arith.rs
    title: src/int/arith.rs
  - icon: ':warning:'
    path: src/int/bisect.rs
    title: src/int/bisect.rs
  - icon: ':x:'
    path: src/int/gcd.rs
    title: src/int/gcd.rs
  - icon: ':warning:'
    path: src/int/inv.rs
    title: src/int/inv.rs
  - icon: ':warning:'
    path: src/math/binom.rs
    title: src/math/binom.rs
  - icon: ':warning:'
    path: src/math/pow.rs
    title: src/math/pow.rs
  - icon: ':warning:'
    path: src/mint/num.rs
    title: src/mint/num.rs
  - icon: ':warning:'
    path: src/slice/cum.rs
    title: src/slice/cum.rs
  - icon: ':x:'
    path: src/tests.rs
    title: src/tests.rs
  _extendedVerifiedWith:
  - icon: ':x:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/dfa_test.rs
    title: test/src/bin/dfa_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/edmonds_karp_test.rs
    title: test/src/bin/edmonds_karp_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ford_fulkerson_test.rs
    title: test/src/bin/ford_fulkerson_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/hlpp_test.rs
    title: test/src/bin/hlpp_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/lazy_segtree_test.rs
    title: test/src/bin/lazy_segtree_test.rs
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':question:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/num.rs\n"
  code: "pub use crate::zo::ZeroOne;\nuse std::fmt::*;\nuse std::ops::*;\n\npub trait\
    \ Num:\n\tZeroOne\n\t+ Add<Output = Self>\n\t+ AddAssign\n\t+ Sub<Output = Self>\n\
    \t+ SubAssign\n\t+ Mul<Output = Self>\n\t+ MulAssign\n\t+ Div<Output = Self>\n\
    \t+ DivAssign\n\t+ Debug\n\t+ Display\n{\n\tfn wrapping_add(self, rhs: Self) ->\
    \ Self;\n\tfn wrapping_neg(self) -> Self;\n}\n\npub trait INum: Num + Neg<Output\
    \ = Self> {}\n\nmacro_rules! impl_num {\n\t($($t:ty),*) => { $(\n\t\timpl Num\
    \ for $t {\n\t\t\tfn wrapping_add(self, rhs: Self) -> Self {\n\t\t\t\tself.wrapping_add(rhs)\n\
    \t\t\t}\n\t\t\tfn wrapping_neg(self) -> Self {\n\t\t\t\tself.wrapping_neg()\n\t\
    \t\t}\n\t\t}\n\t)* };\n}\n\nimpl_num!(i32, i64, i128, isize, u32, u64, u128, usize);\n\
    \nimpl<T: Num + Neg<Output = Self>> INum for T {}\n"
  dependsOn:
  - src/zo.rs
  isVerificationFile: false
  path: src/num.rs
  requiredBy:
  - src/draft/fpacc64.rs
  - src/alg/arith.rs
  - src/int/inv.rs
  - src/int/bisect.rs
  - src/int/gcd.rs
  - src/int/arith.rs
  - src/mint/num.rs
  - src/fp/num.rs
  - src/slice/cum.rs
  - src/graph/dijkstra.rs
  - src/graph/max_flow/push_relabel.rs
  - src/graph/max_flow/ford_fulkerson.rs
  - src/graph/max_flow/edmonds_karp.rs
  - src/graph/max_flow/hlpp/edge.rs
  - src/graph/max_flow/hlpp.rs
  - src/graph/max_flow/edmonds_karp/edge.rs
  - src/graph/max_flow/ford_fulkerson/edge.rs
  - src/graph/max_flow/ford_fulkerson/edges.rs
  - src/graph/euler_tour.rs
  - src/math/binom.rs
  - src/math/pow.rs
  - src/ds/fenwick.rs
  - src/int.rs
  - src/dfa.rs
  - src/tests.rs
  timestamp: '2021-01-30 12:54:22+09:00'
  verificationStatus: LIBRARY_SOME_WA
  verifiedWith:
  - test/src/bin/lazy_segtree_test.rs
  - test/src/bin/hlpp_test.rs
  - test/src/bin/cargo_test.rs
  - test/src/bin/ford_fulkerson_test.rs
  - test/src/bin/edmonds_karp_test.rs
  - test/src/bin/dfa_test.rs
documentation_of: src/num.rs
layout: document
redirect_from:
- /library/src/num.rs
- /library/src/num.rs.html
title: src/num.rs
---