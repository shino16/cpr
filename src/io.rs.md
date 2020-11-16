---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: test/src/bin/ntt_test.rs
    title: test/src/bin/ntt_test.rs
  - icon: ':heavy_check_mark:'
    path: test/src/bin/union_find_test.rs
    title: test/src/bin/union_find_test.rs
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.0/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 67, in bundle\n    assert 'bundle' in self.config\nAssertionError\n"
  code: "use std::io::{stdout, BufWriter, Read, StdoutLock, Write};\n\npub struct\
    \ IO {\n    iter: std::str::SplitAsciiWhitespace<'static>,\n    buf: BufWriter<StdoutLock<'static>>,\n\
    }\n\nimpl IO {\n    pub fn new() -> Self {\n        let mut input = String::new();\n\
    \        std::io::stdin().read_to_string(&mut input).unwrap();\n        let input\
    \ = Box::leak(input.into_boxed_str());\n        let out = Box::new(stdout());\n\
    \        IO {\n            iter: input.split_ascii_whitespace(),\n           \
    \ buf: BufWriter::new(Box::leak(out).lock()),\n        }\n    }\n    fn scan_str(&mut\
    \ self) -> &'static str {\n        self.iter.next().unwrap()\n    }\n    fn scan_raw(&mut\
    \ self) -> &'static [u8] {\n        self.scan_str().as_bytes()\n    }\n    pub\
    \ fn scan<T: Scan>(&mut self) -> T {\n        T::scan(self)\n    }\n    pub fn\
    \ scan_vec<T: Scan>(&mut self, n: usize) -> Vec<T> {\n        (0..n).map(|_| self.scan()).collect()\n\
    \    }\n    pub fn scan_graph(&mut self) -> (usize, usize, Vec<Vec<usize>>) {\n\
    \        let n = self.scan();\n        let m = self.scan();\n        let mut graph\
    \ = vec![Vec::new(); n];\n        for _ in 0..m {\n            let u: usize =\
    \ self.scan();\n            let v: usize = self.scan();\n            graph[u].push(v);\n\
    \            graph[v].push(u);\n        }\n        (n, m, graph)\n    }\n    pub\
    \ fn scan_digraph(&mut self) -> (usize, usize, Vec<Vec<usize>>) {\n        let\
    \ n = self.scan();\n        let m = self.scan();\n        let mut graph = vec![Vec::new();\
    \ n];\n        for _ in 0..m {\n            let u: usize = self.scan();\n    \
    \        let v: usize = self.scan();\n            graph[u].push(v);\n        }\n\
    \        (n, m, graph)\n    }\n    pub fn scan_tree(&mut self) -> (usize, Vec<Vec<usize>>)\
    \ {\n        let n = self.scan();\n        let mut graph = vec![Vec::new(); n];\n\
    \        for _ in 0..n - 1 {\n            let u: usize = self.scan();\n      \
    \      let v: usize = self.scan();\n            graph[u].push(v);\n          \
    \  graph[v].push(u);\n        }\n        (n, graph)\n    }\n}\n\nimpl IO {\n \
    \   pub fn print<T: Print>(&mut self, x: T) {\n        T::print(self, x);\n  \
    \  }\n    pub fn println<T: Print>(&mut self, x: T) {\n        self.print(x);\n\
    \        self.print(\"\\n\");\n    }\n    pub fn iterln<T: Print, I: IntoIterator<Item\
    \ = T>>(&mut self, iter: I, delim: &str) {\n        let mut iter = iter.into_iter();\n\
    \        if let Some(v) = iter.next() {\n            self.print(v);\n        \
    \    for v in iter {\n                self.print(delim);\n                self.print(v);\n\
    \            }\n        }\n        self.print(\"\\n\");\n    }\n    pub fn flush(&mut\
    \ self) {\n        self.buf.flush().unwrap();\n    }\n}\n\npub trait Scan {\n\
    \    fn scan(io: &mut IO) -> Self;\n}\n\nmacro_rules! impl_parse_int {\n    ($($t:tt),*)\
    \ => { $(\n        impl Scan for $t {\n            fn scan(s: &mut IO) -> Self\
    \ {\n                let mut res = 0;\n                for d in s.scan_raw() {\n\
    \                    res *= 10;\n                    res += (*d - b'0') as $t;\n\
    \                }\n                res\n            }\n        }\n    )* };\n\
    }\n\nimpl_parse_int!(i32, i64, isize, u32, u64, usize);\n\nimpl Scan for u8 {\n\
    \    fn scan(s: &mut IO) -> Self {\n        let bytes = s.scan_raw();\n      \
    \  debug_assert_eq!(bytes.len(), 1);\n        bytes[0]\n    }\n}\n\nimpl Scan\
    \ for &[u8] {\n    fn scan(s: &mut IO) -> Self {\n        s.scan_raw()\n    }\n\
    }\n\nimpl<T: Scan, U: Scan> Scan for (T, U) {\n    fn scan(s: &mut IO) -> Self\
    \ {\n        (T::scan(s), U::scan(s))\n    }\n}\n\nimpl<T: Scan, U: Scan, V: Scan>\
    \ Scan for (T, U, V) {\n    fn scan(s: &mut IO) -> Self {\n        (T::scan(s),\
    \ U::scan(s), V::scan(s))\n    }\n}\n\nimpl<T: Scan> Scan for [T; 2] {\n    fn\
    \ scan(s: &mut IO) -> Self {\n        [s.scan(), s.scan()]\n    }\n}\n\nimpl<T:\
    \ Scan> Scan for [T; 3] {\n    fn scan(s: &mut IO) -> Self {\n        [s.scan(),\
    \ s.scan(), s.scan()]\n    }\n}\n\nimpl<T: Scan> Scan for [T; 4] {\n    fn scan(s:\
    \ &mut IO) -> Self {\n        [s.scan(), s.scan(), s.scan(), s.scan()]\n    }\n\
    }\n\npub trait Print {\n    fn print(w: &mut IO, x: Self);\n}\n\nmacro_rules!\
    \ impl_print_int {\n    ($($t:ty),*) => { $(\n        impl Print for $t {\n  \
    \          fn print(w: &mut IO, x: Self) {\n                w.buf.write_all(x.to_string().as_bytes()).unwrap();\n\
    \            }\n        }\n    )* };\n}\n\nimpl_print_int!(i32, i64, isize, u32,\
    \ u64, usize);\n\nimpl Print for u8 {\n    fn print(w: &mut IO, x: Self) {\n \
    \       w.buf.write_all(&[x]).unwrap();\n    }\n}\n\nimpl Print for &[u8] {\n\
    \    fn print(w: &mut IO, x: Self) {\n        w.buf.write_all(x).unwrap();\n \
    \   }\n}\n\nimpl Print for &str {\n    fn print(w: &mut IO, x: Self) {\n     \
    \   w.print(x.as_bytes());\n    }\n}\n\nimpl<T: Print, U: Print> Print for (T,\
    \ U) {\n    fn print(w: &mut IO, (x, y): Self) {\n        w.print(x);\n      \
    \  w.print(\" \");\n        w.print(y);\n    }\n}\n\nimpl<T: Print, U: Print,\
    \ V: Print> Print for (T, U, V) {\n    fn print(w: &mut IO, (x, y, z): Self) {\n\
    \        w.print(x);\n        w.print(\" \");\n        w.print(y);\n        w.print(\"\
    \ \");\n        w.print(z);\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/io.rs
  requiredBy: []
  timestamp: '2020-11-03 22:42:19+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - test/src/bin/ntt_test.rs
  - test/src/bin/union_find_test.rs
documentation_of: src/io.rs
layout: document
redirect_from:
- /library/src/io.rs
- /library/src/io.rs.html
title: src/io.rs
---