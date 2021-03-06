use std::cell::RefCell;
use std::hash::Hash;
use lib2::fxhash::*;
// (otherwise) use std::collections::HashMap;

#[must_use]
pub struct Memo<F, Arg, Ret>(F, RefCell<HashMap<Arg, Ret>>);

impl<F, Arg, Ret> Memo<F, Arg, Ret>
where
    F: Fn(&dyn Fn(Arg) -> Ret, Arg) -> Ret,
    Arg: Clone + Eq + Hash,
    Ret: Clone,
{
    pub fn call(&self, arg: Arg) -> Ret {
        if let Some(ret) = self.1.borrow().get(&arg) {
            return ret.clone();
        }
        let ret = self.0(&|arg| self.call(arg), arg.clone());
        self.1.borrow_mut().insert(arg, ret.clone());
        ret
    }
}

pub fn memoize<Arg: Eq + Hash, Ret, F>(f: F) -> Memo<F, Arg, Ret>
where
    F: Fn(&dyn Fn(Arg) -> Ret, Arg) -> Ret,
{
    Memo(f, RefCell::new(HashMap::default()))
}
