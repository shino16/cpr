---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: src/bit.rs
    title: src/bit.rs
  - icon: ':question:'
    path: src/cast.rs
    title: src/cast.rs
  - icon: ':x:'
    path: src/fp.rs
    title: src/fp.rs
  - icon: ':question:'
    path: src/int.rs
    title: src/int.rs
  - icon: ':question:'
    path: src/int/zo.rs
    title: src/int/zo.rs
  - icon: ':question:'
    path: src/io.rs
    title: src/io.rs
  - icon: ':x:'
    path: src/iter.rs
    title: src/iter.rs
  - icon: ':x:'
    path: src/iter/prod.rs
    title: src/iter/prod.rs
  - icon: ':x:'
    path: src/make_vec.rs
    title: src/make_vec.rs
  - icon: ':x:'
    path: src/math/gcd.rs
    title: src/math/gcd.rs
  - icon: ':question:'
    path: src/mint.rs
    title: src/mint.rs
  - icon: ':question:'
    path: src/rand/seed.rs
    title: src/rand/seed.rs
  - icon: ':x:'
    path: src/rand/xoshiro256plus.rs
    title: src/rand/xoshiro256plus.rs
  - icon: ':x:'
    path: src/tests.rs
    title: src/tests.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    PROBLEM: http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_1_A
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "// verify-helper: PROBLEM http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_1_A\n\
    \n#[allow(unused_imports)]\nuse lib::tests::*;\n\nuse std::io::{self, Write, Result};\n\
    use std::process::Command;\n\nfn main() -> Result<()> {\n\tlet output = Command::new(\"\
    cargo\")\n\t\t.arg(\"test\")\n\t\t.arg(\"--\")\n\t\t.arg(\"--test-threads=1\"\
    )\n\t\t.output()?;\n\n\tif !output.status.success() {\n\t\tprintln!(\"`cargo test`\
    \ failed\");\n\t\tprintln!(\"--- captured stdout ---\");\n\t\tio::stdout().write_all(&output.stdout)?;\n\
    \t\tprintln!(\"--- captured stderr ---\");\n\t\tio::stdout().write_all(&output.stderr)?;\n\
    \t}\n\n\tprintln!(\"Hello World\");\n\n\tOk(())\n}\n"
  dependsOn:
  - src/bit.rs
  - src/cast.rs
  - src/fp.rs
  - src/int.rs
  - src/int/zo.rs
  - src/io.rs
  - src/iter.rs
  - src/iter/prod.rs
  - src/make_vec.rs
  - src/math/gcd.rs
  - src/mint.rs
  - src/rand/seed.rs
  - src/rand/xoshiro256plus.rs
  - src/tests.rs
  isVerificationFile: true
  path: test/src/bin/cargo_test.rs
  requiredBy: []
  timestamp: '2020-12-21 16:33:52+09:00'
  verificationStatus: TEST_WRONG_ANSWER
  verifiedWith: []
documentation_of: test/src/bin/cargo_test.rs
layout: document
redirect_from:
- /verify/test/src/bin/cargo_test.rs
- /verify/test/src/bin/cargo_test.rs.html
title: test/src/bin/cargo_test.rs
---