pub trait Pairable<P>: IntoIterator {
	fn pairwise(&self) -> std::iter::Zip<std::slice::Iter<P>, std::slice::Iter<P>>;
}
impl<T> Pairable<T> for Vec<T> {
	fn pairwise(&self) -> std::iter::Zip<std::slice::Iter<T>, std::slice::Iter<T>> {
		self.iter().zip(self[1..].iter())
	}
}

#[cfg(test)]
mod tests {
	use super::Pairable;

	#[test]
	fn test_pairable_on_single_item_for_loop() {
		let v = vec![1];
		for (_a, _b) in v.pairwise(){
			panic!("pairwise should not work on single item")
		}
	}

	#[test]
	fn test_pairable_as_iter_on_single_item() {
		
		let v = vec![1];
		let j:Vec<(&i32,&i32)> = v.pairwise().collect();
		assert_eq!(j.len(), 0);

		let v = vec![1,2];
		let j:Vec<(i32,i32)> = v.pairwise().map(|(&a,&b)| (a,b)).collect();
		assert_eq!(j.len(), 1);
		assert_eq!(j, vec![(1,2)]);

	}
}