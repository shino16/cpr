---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/dfa_test.rs
    title: test/src/bin/dfa_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_test.rs
    title: test/src/bin/ntt_test.rs
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "pub trait CastTo<T> {\n    fn cast_to(self) -> T;\n}\npub trait CastFrom<T>\
    \ {\n    fn cast_from(src: T) -> Self;\n}\n\nimpl<T, U: CastTo<T>> CastFrom<U>\
    \ for T {\n    fn cast_from(src: U) -> Self {\n        U::cast_to(src)\n    }\n\
    }\n\nmacro_rules! impl_prim {\n    ($($ts:ty),*) => {\n        impl_asint!({ $($ts),*\
    \ } => { $($ts),* });\n        pub trait CastInt where $(Self: CastTo<$ts>),*,\
    \ $(Self: CastFrom<$ts>),* {}\n        $( impl CastInt for $ts {} )*\n    }\n\
    }\n\nmacro_rules! impl_asint {\n    ({ $t:ty } => { $($us:ty),* }) => { $(\n \
    \       impl CastTo<$us> for $t {\n            fn cast_to(self) -> $us {\n   \
    \             self as $us\n            }\n        }\n    )* };\n    ({ $t:ty,\
    \ $($ts:ty),* } => { $($us:ty),* }) => {\n        impl_asint!({ $t } => { $($us),*\
    \ });\n        impl_asint!({ $($ts),* } => { $($us),* });\n    };\n}\n\nimpl_prim!(i32,\
    \ i64, i128, isize, u32, u64, u128, usize);\n\npub trait As: Sized {\n    fn as_<T:\
    \ CastFrom<Self>>(self) -> T {\n        T::cast_from(self)\n    }\n}\n\nimpl<T>\
    \ As for T {}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/as_int.rs
  requiredBy: []
  timestamp: '2020-11-17 21:49:18+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/cargo_test.rs
  - test/src/bin/ntt_test.rs
  - test/src/bin/dfa_test.rs
documentation_of: src/as_int.rs
layout: document
redirect_from:
- /library/src/as_int.rs
- /library/src/as_int.rs.html
title: src/as_int.rs
---