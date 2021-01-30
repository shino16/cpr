---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':warning:'
    path: src/draft/linked_list/inner_mut.rs
    title: src/draft/linked_list/inner_mut.rs
  - icon: ':warning:'
    path: src/draft/linked_list/iter.rs
    title: src/draft/linked_list/iter.rs
  - icon: ':warning:'
    path: src/draft/linked_list/ptr.rs
    title: src/draft/linked_list/ptr.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes: {}
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n  File \"/opt/hostedtoolcache/Python/3.9.1/x64/lib/python3.9/site-packages/onlinejudge_verify/languages/user_defined.py\"\
    , line 68, in bundle\n    raise RuntimeError('bundler is not specified: {}'.format(path.as_posix()))\n\
    RuntimeError: bundler is not specified: src/draft/linked_list.rs\n"
  code: "use std::iter::FromIterator;\nuse std::marker::PhantomData;\nuse std::ops::{Deref,\
    \ DerefMut};\nuse std::ptr::NonNull;\n\npub mod inner_mut;\npub mod ptr;\n\n///\
    \ FIXME: double free\n/// https://github.com/shino16/cpr/runs/1796088138?check_suite_focus=true#step:8:64\n\
    #[derive(PartialEq, PartialOrd, Hash)]\npub struct LinkedList<T> {\n\tpub head:\
    \ NonNull<Node<T>>,\n\tpub tail: NonNull<Node<T>>,\n\tarenas: Vec<Vec<Node<T>>>,\n\
    \tarena_idx: usize,\n\tlen: usize,\n}\n\n#[derive(Debug, PartialEq, PartialOrd,\
    \ Clone, Copy, Hash, Default)]\npub struct Node<T> {\n\tprev: Option<NonNull<Node<T>>>,\n\
    \tnext_val: Option<(NonNull<Node<T>>, T)>,\n}\n\nimpl<T> Node<T> {\n\tfn new()\
    \ -> Self {\n\t\tSelf { prev: None, next_val: None }\n\t}\n}\n\npub struct Iter<'a,\
    \ T: 'a> {\n\thead: NonNull<Node<T>>,\n\tlen: usize,\n\t_marker: PhantomData<&'a\
    \ Node<T>>,\n}\n\npub struct IntoIter<T> {\n\tlist: LinkedList<T>,\n}\n\npub struct\
    \ CursorMut<'a, T: 'a> {\n\tat: NonNull<Node<T>>,\n\tlist: &'a mut LinkedList<T>,\n\
    }\n\nimpl<T> LinkedList<T> {\n\tpub fn new() -> Self {\n\t\tlet mut arenas = vec![vec![Node::new()]];\n\
    \t\tlet head = (&mut arenas[0][0]).into();\n\t\tSelf { head, tail: head, arenas,\
    \ arena_idx: 0, len: 0 }\n\t}\n\tpub fn with_capacity(cap: usize) -> Self {\n\t\
    \tlet mut arenas = vec![Vec::with_capacity(cap)];\n\t\tarenas[0].push(Node::new());\n\
    \t\tlet head = (&mut arenas[0][0]).into();\n\t\tSelf { head, tail: head, arenas,\
    \ arena_idx: 0, len: 0 }\n\t}\n\tpub fn len(&self) -> usize {\n\t\tself.len\n\t\
    }\n\tpub fn is_empty(&self) -> bool {\n\t\tself.head == self.tail\n\t}\n\tpub\
    \ fn clear(&mut self) {\n\t\tself.drop_impl();\n\t\tself.arena_idx = 0;\n\t}\n\
    \tfn new_node(&mut self, node: Node<T>) -> NonNull<Node<T>> {\n\t\tlet arena =\
    \ &self.arenas[self.arena_idx];\n\t\tif arena.len() == arena.capacity() {\n\t\t\
    \tself.arena_idx += 1;\n\t\t\tif self.arena_idx == self.arenas.len() {\n\t\t\t\
    \tlet new_arena = Vec::with_capacity(arena.capacity() * 2);\n\t\t\t\tself.arenas.push(new_arena);\n\
    \t\t\t}\n\t\t}\n\t\tlet arena = &mut self.arenas[self.arena_idx];\n\t\tarena.push(node);\n\
    \t\tarena.last_mut().unwrap().into()\n\t}\n\tpub fn begin_mut(&mut self) -> CursorMut<'_,\
    \ T> {\n\t\tCursorMut { at: self.head, list: self }\n\t}\n\tpub fn end_mut(&mut\
    \ self) -> CursorMut<'_, T> {\n\t\tCursorMut { at: self.tail, list: self }\n\t\
    }\n\tpub fn push_front(&mut self, val: T) {\n\t\tself.begin_mut().insert(val)\n\
    \t}\n\tpub fn push_back(&mut self, val: T) {\n\t\tself.end_mut().insert(val)\n\
    \t}\n\tpub fn pop_front(&mut self) -> Option<T> {\n\t\tself.begin_mut().remove()\n\
    \t}\n\tpub fn pop_back(&mut self) -> Option<T> {\n\t\tself.end_mut().prev()?.remove()\n\
    \t}\n\tpub fn iter(&self) -> Iter<'_, T> {\n\t\tIter { head: self.head, len: self.len,\
    \ _marker: PhantomData }\n\t}\n\tfn drop_impl(&mut self) {\n\t\tfor v in &mut\
    \ self.arenas[1..] {\n\t\t\tunsafe {\n\t\t\t\tv.set_len(0);\n\t\t\t}\n\t\t}\n\t\
    \tlet mut cursor = self.begin_mut();\n\t\twhile cursor.remove().is_some() {}\n\
    \t}\n}\n\nimpl<T> FromIterator<T> for LinkedList<T> {\n\tfn from_iter<I: IntoIterator<Item\
    \ = T>>(iter: I) -> Self {\n\t\tlet iter = iter.into_iter();\n\t\tlet mut res\
    \ = Self::with_capacity(iter.size_hint().0);\n\t\tfor val in iter {\n\t\t\tres.end_mut().insert(val);\n\
    \t\t}\n\t\tres\n\t}\n}\n\nimpl<T> IntoIterator for LinkedList<T> {\n\ttype Item\
    \ = T;\n\ttype IntoIter = IntoIter<T>;\n\tfn into_iter(self) -> Self::IntoIter\
    \ {\n\t\tIntoIter { list: self }\n\t}\n}\n\nimpl<T: Clone + std::fmt::Debug> Clone\
    \ for LinkedList<T> {\n\tfn clone(&self) -> Self {\n\t\tself.iter().cloned().collect()\n\
    \t}\n}\n\nimpl<T> Drop for LinkedList<T> {\n\tfn drop(&mut self) {\n\t\tself.drop_impl();\n\
    \t}\n}\n\nimpl<'a, T: 'a + std::fmt::Debug> Iterator for Iter<'a, T> {\n\ttype\
    \ Item = &'a T;\n\tfn next(&mut self) -> Option<Self::Item> {\n\t\tlet next_val\
    \ = unsafe { &*self.head.as_ptr() }.next_val.as_ref()?;\n\t\tlet res = &next_val.1;\n\
    \t\tassert!(next_val.0 != self.head);\n\t\tself.head = next_val.0;\n\t\tSome(res)\n\
    \t}\n\tfn size_hint(&self) -> (usize, Option<usize>) {\n\t\t(self.len, Some(self.len))\n\
    \t}\n}\n\nimpl<T> Iterator for IntoIter<T> {\n\ttype Item = T;\n\tfn next(&mut\
    \ self) -> Option<Self::Item> {\n\t\tself.list.pop_front()\n\t}\n}\n\nimpl<'a,\
    \ T: 'a> Deref for CursorMut<'a, T> {\n\ttype Target = T;\n\tfn deref(&self) ->\
    \ &Self::Target {\n\t\tunsafe { &self.at.as_ref().next_val.as_ref().unwrap().1\
    \ }\n\t}\n}\n\nimpl<'a, T: 'a> DerefMut for CursorMut<'a, T> {\n\tfn deref_mut(&mut\
    \ self) -> &mut Self::Target {\n\t\tunsafe { &mut self.at.as_mut().next_val.as_mut().unwrap().1\
    \ }\n\t}\n}\n\nimpl<'a, T: 'a> CursorMut<'a, T> {\n\tpub fn next(&mut self) ->\
    \ Option<&mut Self> {\n\t\tself.at = unsafe { self.at.as_ref() }.next_val.as_ref()?.0;\n\
    \t\tSome(self)\n\t}\n\tpub fn prev(&mut self) -> Option<&mut Self> {\n\t\tself.at\
    \ = unsafe { self.at.as_ref() }.prev?;\n\t\tSome(self)\n\t}\n\tpub fn advance(&mut\
    \ self, by: isize) -> Option<&mut Self> {\n\t\tif by >= 0 {\n\t\t\tfor _ in 0..by\
    \ {\n\t\t\t\tself.next()?;\n\t\t\t}\n\t\t} else {\n\t\t\tfor _ in by..0 {\n\t\t\
    \t\tself.prev()?;\n\t\t\t}\n\t\t}\n\t\tSome(self)\n\t}\n\tpub fn insert(&mut self,\
    \ val: T) {\n\t\tlet prev = unsafe { self.at.as_ref() }.prev;\n\t\tlet new_node\
    \ = self.list.new_node(Node { prev, next_val: Some((self.at, val)) });\n\t\tunsafe\
    \ { self.at.as_mut() }.prev = Some(new_node);\n\t\tif let Some(mut prev) = prev\
    \ {\n\t\t\tunsafe { prev.as_mut() }.next_val.as_mut().unwrap().0 = new_node;\n\
    \t\t} else {\n\t\t\tself.list.head = new_node;\n\t\t}\n\t\tself.at = new_node;\n\
    \t\tself.list.len += 1;\n\t\tunsafe {\n\t\t\tif let Some(prev) = self.at.as_ref().prev\
    \ {\n\t\t\t\tassert!(Some(prev) != prev.as_ref().next_val.as_ref().map(|t| t.0));\n\
    \t\t\t}\n\t\t\tassert!(Some(self.at) != self.at.as_ref().next_val.as_ref().map(|t|\
    \ t.0));\n\t\t\tif let Some((next, _)) = self.at.as_ref().next_val {\n\t\t\t\t\
    assert!(Some(next) != next.as_ref().next_val.as_ref().map(|t| t.0));\n\t\t\t}\n\
    \t\t}\n\t}\n\tpub fn remove(&mut self) -> Option<T> {\n\t\tif self.at == self.list.tail\
    \ {\n\t\t\treturn None;\n\t\t}\n\t\tunsafe {\n\t\t\tlet node = std::ptr::read(self.at.as_ptr());\n\
    \t\t\tlet (mut next, val) = node.next_val?;\n\t\t\tif let Some(mut prev) = node.prev\
    \ {\n\t\t\t\t*next.as_mut().prev.as_mut().unwrap() = prev;\n\t\t\t\tprev.as_mut().next_val.as_mut().unwrap().0\
    \ = next;\n\t\t\t} else {\n\t\t\t\tnext.as_mut().prev = None;\n\t\t\t\tself.list.head\
    \ = next;\n\t\t\t}\n\t\t\tself.at = next;\n\n\t\t\tif let Some(prev) = self.at.as_ref().prev\
    \ {\n\t\t\t\tassert!(Some(prev) != prev.as_ref().next_val.as_ref().map(|t| t.0));\n\
    \t\t\t}\n\t\t\tassert!(Some(self.at) != self.at.as_ref().next_val.as_ref().map(|t|\
    \ t.0));\n\t\t\tif let Some((next, _)) = self.at.as_ref().next_val {\n\t\t\t\t\
    assert!(Some(next) != next.as_ref().next_val.as_ref().map(|t| t.0));\n\t\t\t}\n\
    \t\t\tSome(val)\n\t\t}\n\t}\n}\n\nimpl<T: std::fmt::Debug> std::fmt::Debug for\
    \ LinkedList<T> {\n\tfn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result\
    \ {\n\t\tf.debug_list().entries(self.iter()).finish()\n\t}\n}\n\n#[cfg(test)]\n\
    mod test {\n\t#[test]\n\tfn test_linked_list() {\n\t\tuse crate::ds::linked_list::inner_mut::*;\n\
    \t\tuse std::cell::RefCell;\n\t\tuse std::sync::atomic::{AtomicU32, Ordering};\n\
    \n\t\tstatic DROP_CNT: AtomicU32 = AtomicU32::new(0);\n\t\t#[derive(PartialEq,\
    \ Eq, Clone, Debug)]\n\t\tstruct S(u32);\n\t\timpl Drop for S {\n\t\t\tfn drop(&mut\
    \ self) {\n\t\t\t\tDROP_CNT.fetch_add(1, Ordering::SeqCst);\n\t\t\t}\n\t\t}\n\n\
    \t\tlet mut v = Vec::new();\n\t\tlet mut l = LinkedList::new();\n\t\tlet mut l2\
    \ = l.clone();\n\t\tlet mut cur = l2.begin_mut();\n\t\tfor n in 0..10 {\n\t\t\t\
    v.push(Box::new(S(n)));\n\t\t\tl.push_back(Box::new(S(n)));\n\t\t\tcur.insert(Box::new(S(n)));\n\
    \t\t\tcur.next().unwrap();\n\t\t}\n\t\tassert_eq!(v, l.clone().into_iter().collect::<Vec<_>>());\n\
    \t\tassert_eq!(v, l2.into_iter().collect::<Vec<_>>());\n\n\t\tlet l = RefCell::new(l);\n\
    \t\tlet mut cur = LinkedList::begin_inner_mut(&l);\n\t\tcur.advance(7).unwrap().remove();\n\
    \t\tv.remove(7);\n\t\tassert_eq!(v, l.borrow().clone().into_iter().collect::<Vec<_>>());\n\
    \t\tcur.advance(-2).unwrap().insert(Box::new(S(100)));\n\t\tv.insert(5, Box::new(S(100)));\n\
    \t\tassert_eq!(v, l.borrow().clone().into_iter().collect::<Vec<_>>());\n\t\tlet\
    \ mut cur = LinkedList::end_inner_mut(&l);\n\t\tcur.advance(-8).unwrap().remove();\n\
    \t\tv.remove(2);\n\t\tassert_eq!(v, l.borrow().clone().into_iter().collect::<Vec<_>>());\n\
    \t\tcur.advance(-2).unwrap();\n\t\tassert!(cur.prev().is_none());\n\t\tstd::mem::drop((v,\
    \ l));\n\t\tassert_eq!(DROP_CNT.load(Ordering::SeqCst), 70);\n\t}\n\t#[test]\n\
    \tfn test_linked_list_ptr() {\n\t\tuse crate::ds::linked_list::ptr::*;\n\t\tuse\
    \ std::sync::atomic::{AtomicU32, Ordering};\n\n\t\tstatic DROP_CNT: AtomicU32\
    \ = AtomicU32::new(0);\n\t\t#[derive(PartialEq, Eq, Clone, Debug)]\n\t\tstruct\
    \ S(u32);\n\t\timpl Drop for S {\n\t\t\tfn drop(&mut self) {\n\t\t\t\tDROP_CNT.fetch_add(1,\
    \ Ordering::SeqCst);\n\t\t\t}\n\t\t}\n\n\t\tlet mut v = Vec::new();\n\t\tlet mut\
    \ l = LinkedList::new();\n\t\tlet mut l2 = l.clone();\n\t\tlet mut cur = l2.begin_mut();\n\
    \t\tfor n in 0..10 {\n\t\t\tv.push(Box::new(S(n)));\n\t\t\tl.push_back(Box::new(S(n)));\n\
    \t\t\tcur.insert(Box::new(S(n)));\n\t\t\tcur.next().unwrap();\n\t\t}\n\t\tassert_eq!(v,\
    \ l.clone().into_iter().collect::<Vec<_>>());\n\t\tassert_eq!(v, l2.into_iter().collect::<Vec<_>>());\n\
    \n\t\tlet mut cur = l.begin_ptr();\n\t\tunsafe { cur.advance(7).unwrap().remove();\
    \ }\n\t\tv.remove(7);\n\t\tassert_eq!(v, l.clone().into_iter().collect::<Vec<_>>());\n\
    \t\tunsafe { cur.advance(-2).unwrap().insert(Box::new(S(100))); }\n\t\tv.insert(5,\
    \ Box::new(S(100)));\n\t\tassert_eq!(v, l.clone().into_iter().collect::<Vec<_>>());\n\
    \t\tlet mut cur = l.end_ptr();\n\t\tunsafe { cur.advance(-8).unwrap().remove();\
    \ }\n\t\tv.remove(2);\n\t\tassert_eq!(v, l.clone().into_iter().collect::<Vec<_>>());\n\
    \t\tcur.advance(-2).unwrap();\n\t\tassert!(cur.prev().is_none());\n\t\tfor (v,\
    \ l) in v.iter().zip(l.iter()) {\n\t\t\tassert_eq!(v, l);\n\t\t}\n\t\tstd::mem::drop((v,\
    \ l));\n\t\tassert_eq!(DROP_CNT.load(Ordering::SeqCst), 70);\n\t}\n}"
  dependsOn: []
  isVerificationFile: false
  path: src/draft/linked_list.rs
  requiredBy:
  - src/draft/linked_list/ptr.rs
  - src/draft/linked_list/iter.rs
  - src/draft/linked_list/inner_mut.rs
  timestamp: '2021-01-30 17:33:56+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: src/draft/linked_list.rs
layout: document
redirect_from:
- /library/src/draft/linked_list.rs
- /library/src/draft/linked_list.rs.html
title: src/draft/linked_list.rs
---