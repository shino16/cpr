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
  code: "use std::mem::ManuallyDrop;\n\npub fn transmute_vec<T, U>(v: Vec<T>) -> Vec<U>\
    \ {\n\tlet mut v = ManuallyDrop::new(v);\n\tunsafe { Vec::from_raw_parts(v.as_mut_ptr()\
    \ as *mut _, v.len(), v.capacity()) }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/vec.rs
  requiredBy: []
  timestamp: '2020-11-27 14:24:44+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/vec.rs
layout: document
redirect_from:
- /library/src/vec.rs
- /library/src/vec.rs.html
title: src/vec.rs
---