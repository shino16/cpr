pub trait Slice {
    type Item;
    fn fill(&mut self, value: Self::Item)
    where
        Self::Item: Clone;
    /// min { i | !pred(arr[i]) }
    fn partition_point<F: FnMut(&Self::Item) -> bool>(&self, pred: F) -> usize;
    fn lower_bound(&self, v: &Self::Item) -> usize
    where
        Self::Item: Ord,
    {
        self.partition_point(|x| x < v)
    }
    fn upper_bound(&self, v: &Self::Item) -> usize
    where
        Self::Item: Ord,
    {
        self.partition_point(|x| x <= v)
    }
}

impl<T> Slice for [T] {
    type Item = T;
    fn fill(&mut self, value: Self::Item)
    where
        Self::Item: Clone,
    {
        self.iter_mut().for_each(|e| e.clone_from(&value));
    }
    fn partition_point<F: FnMut(&Self::Item) -> bool>(&self, mut pred: F) -> usize {
        let (mut lb, mut ub) = (0, self.len()); // pred(self[ub]) == false
        while lb != ub {
            let mid = (lb + ub) / 2;
            if pred(&self[mid]) {
                lb = mid + 1;
            } else {
                ub = mid;
            }
        }
        ub
    }
}