---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/cargo_test.rs
    title: test/src/bin/cargo_test.rs
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "#[cfg(test)]\nmod tests {\n\tmod fp {\n\t\tuse crate::fp::*;\n\t\t#[test]\n\
    \t\tfn test_pow() {\n\t\t\tuse crate::rng::*;\n\t\t\tlet mut rng = Rng32::new();\n\
    \t\t\tassert_eq!(F17::from(2).pow(3), F17::from(8));\n\t\t\tfor _ in 0..100 {\n\
    \t\t\t\tlet base: F17 = rng.gen().into();\n\t\t\t\tlet k = rng.gen() % 100;\n\t\
    \t\t\tlet p = (0..k).map(|_| base).product::<F17>();\n\t\t\t\tassert_eq!(p, base.pow(k));\n\
    \t\t\t}\n\t\t}\n\t\t#[test]\n\t\tfn test_inv() {\n\t\t\tuse crate::rng::*;\n\t\
    \t\tlet mut rng = Rng32::new();\n\t\t\tfor _ in 0..100 {\n\t\t\t\tlet a: F17 =\
    \ rng.gen().into();\n\t\t\t\tlet b = a.inv();\n\t\t\t\tassert!(a * b == F17::ONE,\
    \ \"{} {}\", a, b);\n\t\t\t}\n\t\t}\n\t}\n\n\tmod fp_naive {\n\t\tuse crate::mint::*;\n\
    \t\t#[test]\n\t\tfn test_mul() {\n\t\t\tuse crate::rng::*;\n\t\t\tlet mut rng\
    \ = Rng32::new();\n\t\t\tfor _ in 0..100 {\n\t\t\t\tlet a = rng.gen() as u64;\n\
    \t\t\t\tlet b = rng.gen() as u64;\n\t\t\t\tassert_eq!(Mint17::from(a) * b, Mint17::from(a\
    \ * b));\n\t\t\t}\n\t\t}\n\t\t#[test]\n\t\tfn test_pow() {\n\t\t\tuse crate::rng::*;\n\
    \t\t\tlet mut rng = Rng32::new();\n\t\t\tfor _ in 0..100 {\n\t\t\t\tlet base:\
    \ Mint17 = rng.gen().into();\n\t\t\t\tlet k = rng.gen() % 100;\n\t\t\t\tlet p\
    \ = (0..k).map(|_| base).product::<Mint17>();\n\t\t\t\tassert_eq!(p, base.pow(k.into()));\n\
    \t\t\t}\n\t\t}\n\t\t#[test]\n\t\tfn test_inv() {\n\t\t\tuse crate::rng::*;\n\t\
    \t\tlet mut rng = Rng32::new();\n\t\t\tfor _ in 0..100 {\n\t\t\t\tlet a: Mint17\
    \ = rng.gen().into();\n\t\t\t\tlet b = a.inv();\n\t\t\t\tassert!(a * b == Mint17::ONE,\
    \ \"{} * {} = {}\", a, b, a * b);\n\t\t\t}\n\t\t}\n\t}\n\n\tmod iter {\n\t\tuse\
    \ crate::iter::prod::*;\n\t\tuse crate::iter::*;\n\t\t#[test]\n\t\tfn test() {\n\
    \t\t\tlet lhs = (0..3).prod(b\"ab\".to_vec()).collect_vec();\n\t\t\tlet rhs =\
    \ vec![\n\t\t\t\t(0, b'a'),\n\t\t\t\t(0, b'b'),\n\t\t\t\t(1, b'a'),\n\t\t\t\t\
    (1, b'b'),\n\t\t\t\t(2, b'a'),\n\t\t\t\t(2, b'b'),\n\t\t\t];\n\t\t\tassert_eq!(lhs,\
    \ rhs);\n\t\t}\n\t}\n\n\tmod num {\n\t\tuse crate::num::*;\n\t\t#[test]\n\t\t\
    fn types() {\n\t\t\tassert_eq!(<i32 as Int>::Signed::ZERO, 0_i32);\n\t\t\tassert_eq!(<i32\
    \ as Int>::Unsigned::ZERO, 0_u32);\n\t\t\tassert_eq!(<u32 as Int>::Signed::ZERO,\
    \ 0_i32);\n\t\t\tassert_eq!(<u32 as Int>::Unsigned::ZERO, 0_u32);\n\t\t}\n\t}\n\
    \n\tmod make_vec {\n\t\tuse crate::make_vec::*;\n\t\t#[test]\n\t\tfn test() {\n\
    \t\t\tlet v = make_vec((3, (5, 8)), \"foo\");\n\t\t\tassert_eq!(v, vec![vec![vec![\"\
    foo\"; 8]; 5]; 3]);\n\t\t}\n\t}\n\n\tmod math {\n\t\tmod gcd {\n\t\t\tuse crate::math::gcd::*;\n\
    \t\t\t#[test]\n\t\t\tfn test_gcd() {\n\t\t\t\tassert_eq!(gcd(0, 0), 0);\n\t\t\t\
    \tfor a in 0..100 {\n\t\t\t\t\tfor b in 1..100 {\n\t\t\t\t\t\tlet g = gcd(a, b);\n\
    \t\t\t\t\t\tfor c in g + 1..g {\n\t\t\t\t\t\t\tassert!(a % c != 0 || b % c !=\
    \ 0);\n\t\t\t\t\t\t}\n\t\t\t\t\t\tassert_eq!(a % g, 0);\n\t\t\t\t\t\tassert_eq!(b\
    \ % g, 0);\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t}\n\t\t}\n\t}\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/tests.rs
  requiredBy: []
  timestamp: '2020-11-27 14:24:44+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/cargo_test.rs
documentation_of: src/tests.rs
layout: document
redirect_from:
- /library/src/tests.rs
- /library/src/tests.rs.html
title: src/tests.rs
---