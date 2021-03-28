pub trait Pairable<P>: IntoIterator {
	fn pairwise(&self) -> std::iter::Zip<std::slice::Iter<P>, std::slice::Iter<P>>;
}
impl<T> Pairable<T> for Vec<T> {
	fn pairwise(&self) -> std::iter::Zip<std::slice::Iter<T>, std::slice::Iter<T>> {
		self.iter().zip(self[1..].iter())
	}
}