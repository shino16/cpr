---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: src/ds/uf.rs
    title: src/ds/uf.rs
  - icon: ':heavy_check_mark:'
    path: src/io.rs
    title: src/io.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_1_A
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "// verify-helper: PROBLEM http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_1_A\n\
    \nuse lib::ds::uf::*;\nuse lib::io::*;\n\nfn main() {\n    let mut io = IO::new();\n\
    \    let (n, q) = io.scan();\n    let mut uf = UnionFind::new(n);\n    for _ in\
    \ 0_usize..q {\n        let (com, x, y): (u8, _, _) = io.scan();\n        if com\
    \ == b'0' {\n            uf.unite(x, y);\n        } else {\n            io.println(uf.is_same(x,\
    \ y) as u32);\n        }\n    }\n}\n"
  dependsOn:
  - src/ds/uf.rs
  - src/io.rs
  isVerificationFile: true
  path: test/src/bin/union_find_test.rs
  requiredBy: []
  timestamp: '2020-11-16 01:31:27+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: test/src/bin/union_find_test.rs
layout: document
redirect_from:
- /verify/test/src/bin/union_find_test.rs
- /verify/test/src/bin/union_find_test.rs.html
title: test/src/bin/union_find_test.rs
---