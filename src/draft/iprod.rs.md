---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "macro_rules! iprod {\n    ($head:expr) => {\n        $head\n    };\n    ($head:expr,\
    \ $($tail:expr),*) => (\n        $head.flat_map(|e| {\n            std::iter::repeat(e).zip(iprod!($($tail),*))\n\
    \        })\n    );\n}"
  dependsOn: []
  isVerificationFile: false
  path: src/draft/iprod.rs
  requiredBy: []
  timestamp: '2020-11-16 22:35:05+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/draft/iprod.rs
layout: document
redirect_from:
- /library/src/draft/iprod.rs
- /library/src/draft/iprod.rs.html
title: src/draft/iprod.rs
---