pub mod either;
pub mod pow;
pub mod prod;

pub trait Itertools: Iterator {
	fn collect_vec(self) -> Vec<Self::Item>
	where
		Self: Sized,
	{
		self.collect()
	}
}

impl<I: Iterator> Itertools for I {}

#[macro_export]
macro_rules! iprod {
	($head:expr) => {
		$head.into_iter()
	};
	($head:expr, $($tail:expr),*) => (
		$head.into_iter().flat_map(|e| {
			std::iter::repeat(e).zip(iprod!($($tail),*))
		})
	);
}
