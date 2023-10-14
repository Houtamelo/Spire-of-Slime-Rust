#[macro_export]
macro_rules! hash_set {
    	[$($x:expr),+] => {
		{
			let mut set = ::std::collections::HashSet::new();
			$(set.insert($x);)+
			set
		}
	};
}