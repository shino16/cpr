---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/ds/fenwick.rs
    title: src/ds/fenwick.rs
  - icon: ':warning:'
    path: src/slice/cum.rs
    title: src/slice/cum.rs
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/alg/arith.rs\n"
  code: "pub use super::*;\npub use crate::int::*;\nuse std::marker::PhantomData;\n\
    \n#[derive(Default)]\npub struct Addition<N>(PhantomData<N>);\n\nimpl<N> Addition<N>\
    \ {\n\tpub fn new() -> Self {\n\t\tSelf(PhantomData)\n\t}\n}\n\nimpl<N: Num> Alg\
    \ for Addition<N> {\n\ttype Item = N;\n}\n\nimpl<N: Num> Monoid for Addition<N>\
    \ {\n\tfn unit(&self) -> Self::Item {\n\t\tN::ZERO\n\t}\n\tfn op(&self, x: Self::Item,\
    \ y: Self::Item) -> Self::Item {\n\t\tx.wrapping_add(y)\n\t}\n}\n\nimpl<N: Num>\
    \ Group for Addition<N> {\n\tfn inv(&self, x: Self::Item) -> Self::Item {\n\t\t\
    x.wrapping_neg()\n\t}\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/alg/arith.rs
  requiredBy:
  - src/slice/cum.rs
  - src/ds/fenwick.rs
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/alg/arith.rs
layout: document
redirect_from:
- /library/src/alg/arith.rs
- /library/src/alg/arith.rs.html
title: src/alg/arith.rs
---