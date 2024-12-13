#[macro_export]
macro_rules! impl_into_boxed {
    ($trait_ty: ident for {
	    $($ty: ident),* $(,)?
    }) => {
	    $(
	        impl From<$ty> for Box<dyn $trait_ty> {
	            fn from(value: $ty) -> Self {
	                Box::new(value)
	            }
	        }
	    )*
    };
}
