---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: src/graph/io.rs
    title: src/graph/io.rs
  - icon: ':heavy_check_mark:'
    path: src/io.rs
    title: src/io.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/io/graph.rs\n"
  code: 'pub use crate::graph::io::*;

    '
  dependsOn:
  - src/graph/io.rs
  - src/io.rs
  isVerificationFile: false
  path: src/io/graph.rs
  requiredBy: []
  timestamp: '2021-02-15 17:55:41+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/io/graph.rs
layout: document
redirect_from:
- /library/src/io/graph.rs
- /library/src/io/graph.rs.html
title: src/io/graph.rs
---