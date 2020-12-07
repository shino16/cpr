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
  code: "use crate::ds::uvec::*;\nuse std::ops::{Index, IndexMut};\n\n#[derive(Clone)]\n\
    pub struct UVec2D<T> {\n\tpub h: usize,\n\tpub w: usize,\n\tpub inner: UVec<T>,\n\
    }\n\nimpl<T: Clone> UVec2D<T> {\n\tpub fn fill(h: usize, w: usize, v: T) -> Self\
    \ {\n\t\tSelf { h, w, inner: UVec(vec![v; h * w]) }\n\t}\n\tpub fn resize_from(h:\
    \ usize, w: usize, inner: UVec<T>) -> Self {\n\t\tdebug_assert_eq!(inner.len(),\
    \ h * w);\n\t\tSelf { h, w, inner }\n\t}\n}\n\nimpl<T> Index<(usize, usize)> for\
    \ UVec2D<T> {\n\ttype Output = T;\n\tfn index(&self, (r, c): (usize, usize)) ->\
    \ &Self::Output {\n\t\t&self.inner[r * self.w + c]\n\t}\n}\n\nimpl<T> IndexMut<(usize,\
    \ usize)> for UVec2D<T> {\n\tfn index_mut(&mut self, (r, c): (usize, usize)) ->\
    \ &mut Self::Output {\n\t\t&mut self.inner[r * self.w + c]\n\t}\n}\n\nimpl<T>\
    \ Index<usize> for UVec2D<T> {\n\ttype Output = [T];\n\tfn index(&self, r: usize)\
    \ -> &Self::Output {\n\t\t&self.inner[r * self.w..(r + 1) * self.w]\n\t}\n}\n\n\
    impl<T> IndexMut<usize> for UVec2D<T> {\n\tfn index_mut(&mut self, r: usize) ->\
    \ &mut Self::Output {\n\t\t&mut self.inner[r * self.w..(r + 1) * self.w]\n\t}\n\
    }\n"
  dependsOn: []
  isVerificationFile: false
  path: src/ds/uvec2d.rs
  requiredBy: []
  timestamp: '2020-11-27 14:24:44+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/ds/uvec2d.rs
layout: document
redirect_from:
- /library/src/ds/uvec2d.rs
- /library/src/ds/uvec2d.rs.html
title: src/ds/uvec2d.rs
---