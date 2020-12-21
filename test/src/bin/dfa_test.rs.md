---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: src/bit.rs
    title: src/bit.rs
  - icon: ':question:'
    path: src/cast.rs
    title: src/cast.rs
  - icon: ':heavy_check_mark:'
    path: src/dfa.rs
    title: src/dfa.rs
  - icon: ':heavy_check_mark:'
    path: src/fxhash.rs
    title: src/fxhash.rs
  - icon: ':question:'
    path: src/int.rs
    title: src/int.rs
  - icon: ':question:'
    path: src/int/zo.rs
    title: src/int/zo.rs
  - icon: ':question:'
    path: src/io.rs
    title: src/io.rs
  - icon: ':question:'
    path: src/mint.rs
    title: src/mint.rs
  - icon: ':question:'
    path: src/rand/seed.rs
    title: src/rand/seed.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=0570
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "// verify-helper: PROBLEM http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=0570\n\
    \nuse lib::dfa::*;\nuse lib::io::*;\nuse lib::mint::*;\n\n#[derive(Ord, PartialOrd,\
    \ Eq, PartialEq, Copy, Clone, Hash)]\nenum ZigZagState {\n\tInitial,\n\tFirst(u8),\n\
    \tIncreasing(u8),\n\tDecreasing(u8),\n}\n\nstruct ZigZag;\n\nimpl Dfa for ZigZag\
    \ {\n\ttype Alphabet = u8;\n\ttype State = Option<ZigZagState>;\n\tfn init(&self)\
    \ -> Self::State {\n\t\tSome(ZigZagState::Initial)\n\t}\n\tfn next(&self, s: Self::State,\
    \ a: Self::Alphabet, _: usize) -> Self::State {\n\t\tuse ZigZagState::*;\n\t\t\
    if let Some(s) = s {\n\t\t\tmatch s {\n\t\t\t\tInitial if a == b'0' => Some(Initial),\n\
    \t\t\t\tInitial => Some(First(a)),\n\t\t\t\tFirst(d) if d < a => Some(Increasing(a)),\n\
    \t\t\t\tFirst(d) if d > a => Some(Decreasing(a)),\n\t\t\t\tIncreasing(d) if d\
    \ > a => Some(Decreasing(a)),\n\t\t\t\tDecreasing(d) if d < a => Some(Increasing(a)),\n\
    \t\t\t\t_ => None,\n\t\t\t}\n\t\t} else {\n\t\t\tNone\n\t\t}\n\t}\n\tfn accept(&self,\
    \ s: Self::State) -> bool {\n\t\ts.is_some()\n\t}\n\tfn unsuccessful(&self, s:\
    \ Self::State) -> bool {\n\t\ts.is_none()\n\t}\n}\n\n#[derive(Default, Clone,\
    \ Copy, PartialEq, Eq)]\nstruct Modx;\n\nimpl Mod for Modx {\n\tconst P: u32 =\
    \ 10000;\n\tconst PHI: u32 = 4000;\n}\n\nfn main() {\n\tlet mut io = IO::new();\n\
    \tlet [a0, b]: [&[u8]; 2] = io.scan();\n\tlet m = io.scan();\n\tlet mut a = vec![b'0';\
    \ b.len()];\n\ta[b.len() - a0.len()..].copy_from_slice(&a0);\n\n\tlet dfa = And(ZigZag,\
    \ And(MultipleOf(m), And(Leq(&b), Not(Lt(&a)))));\n\tlet alphabet = \"0123456789\"\
    .as_bytes();\n\tlet ans: Mint<Modx> = dfa.count(a.len(), alphabet);\n\tprintln!(\"\
    {}\", ans);\n}\n"
  dependsOn:
  - src/bit.rs
  - src/cast.rs
  - src/dfa.rs
  - src/fxhash.rs
  - src/int.rs
  - src/int/zo.rs
  - src/io.rs
  - src/mint.rs
  - src/rand/seed.rs
  isVerificationFile: true
  path: test/src/bin/dfa_test.rs
  requiredBy: []
  timestamp: '2020-12-21 16:33:52+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: test/src/bin/dfa_test.rs
layout: document
redirect_from:
- /verify/test/src/bin/dfa_test.rs
- /verify/test/src/bin/dfa_test.rs.html
title: test/src/bin/dfa_test.rs
---