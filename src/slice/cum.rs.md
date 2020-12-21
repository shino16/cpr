---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "pub use crate::int::alg::*;\n\npub trait Cum {\n\ttype Item: Clone;\n\tfn\
    \ cuml<M: Monoid<Item = Self::Item>>(&self, m: M) -> Vec<Self::Item>;\n\tfn cumr<M:\
    \ Monoid<Item = Self::Item>>(&self, m: M) -> Vec<Self::Item>;\n\tfn cuml_sum(&self)\
    \ -> Vec<Self::Item>\n\twhere\n\t\tSelf::Item: Num,\n\t{\n\t\tself.cuml(Addition::new())\n\
    \t}\n\tfn cumr_sum(&self) -> Vec<Self::Item>\n\twhere\n\t\tSelf::Item: Num,\n\t\
    {\n\t\tself.cumr(Addition::new())\n\t}\n}\n\nimpl<T: Clone> Cum for [T] {\n\t\
    type Item = T;\n\tfn cuml<M: Monoid<Item = Self::Item>>(&self, m: M) -> Vec<Self::Item>\
    \ {\n\t\tlet mut res = Vec::with_capacity(self.len() + 1);\n\t\tlet mut tl = m.unit();\n\
    \t\tres.push(tl.clone());\n\t\tfor e in self {\n\t\t\ttl = m.op(tl, e.clone());\n\
    \t\t\tres.push(tl.clone());\n\t\t}\n\t\tres\n\t}\n\n\tfn cumr<M: Monoid<Item =\
    \ Self::Item>>(&self, m: M) -> Vec<Self::Item> {\n\t\tlet mut res = Vec::with_capacity(self.len()\
    \ + 1);\n\t\tlet mut tl = m.unit();\n\t\tres.push(tl.clone());\n\t\tfor e in self.into_iter().rev()\
    \ {\n\t\t\ttl = m.op(e.clone(), tl);\n\t\t\tres.push(tl.clone());\n\t\t}\n\t\t\
    res.reverse();\n\t\tres\n\t}\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/slice/cum.rs
  requiredBy: []
  timestamp: '2020-12-21 16:32:06+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/slice/cum.rs
layout: document
redirect_from:
- /library/src/slice/cum.rs
- /library/src/slice/cum.rs.html
title: src/slice/cum.rs
---