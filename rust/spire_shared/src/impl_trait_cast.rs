#[macro_export]
macro_rules! impl_trait_cast {
	($trait_ty:ident) => {
		impl dyn $trait_ty {
			pub fn cast<P: 'static + $trait_ty>(&self) -> Option<&P> {
				(self as &dyn std::any::Any).downcast_ref()
			}

			pub fn cast_mut<P: 'static + $trait_ty>(&mut self) -> Option<&mut P> {
				(self as &mut dyn std::any::Any).downcast_mut()
			}
		}
	};
}
