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
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/graph/tree/reroot.rs\n"
  code: "use std::cell::RefCell;\npub use super::dfs_io::*;\npub use crate::alg::*;\n\
    \npub fn rerooting<G: Graph, A: Group>(g: &G, s: usize, alg: A) -> Vec<A::Item>\
    \ {\n\tlet state = RefCell::new(vec![alg.unit(); g.len()]);\n\tlet mut res = vec![alg.unit();\
    \ g.len()];\n\tdfs_io(g, s, |_, _| {}, |v, par| {\n\t\tlet mut state = state.borrow_mut();\n\
    \t\tg.adj(v, |w| {\n\t\t\tif w != par {\n\t\t\t\talg.op_to(state[w], &mut state[v]);\n\
    \t\t\t}\n\t\t});\n\t});\n\tres[s] = state.borrow()[s];\n\tlet f_in = |v: usize,\
    \ par: usize| {\n\t\tlet mut state = state.borrow_mut();\n\t\tif par != !0 {\n\
    \t\t\talg.op_inv_to(state[v], &mut state[par]);\n\t\t\talg.op_to(state[par], &mut\
    \ state[v]);\n\t\t\tres[v] = state[v];\n\t\t}\n\t};\n\tlet f_out = |v: usize,\
    \ par: usize| {\n\t\tlet mut state = state.borrow_mut();\n\t\tif par != !0 {\n\
    \t\t\talg.op_inv_to(state[par], &mut state[v]);\n\t\t\talg.op_to(state[v], &mut\
    \ state[par]);\n\t\t}\n\t};\n\tdfs_io(g, s, f_in, f_out);\n\tres\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: src/graph/tree/reroot.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/graph/tree/reroot.rs
layout: document
redirect_from:
- /library/src/graph/tree/reroot.rs
- /library/src/graph/tree/reroot.rs.html
title: src/graph/tree/reroot.rs
---