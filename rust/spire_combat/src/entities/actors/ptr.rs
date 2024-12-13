use super::*;

pub struct Ptr<T>(Rc<UnsafeCell<T>>);

impl<T> Ptr<T> {
	pub fn new(value: T) -> Self { Self(Rc::new(UnsafeCell::new(value))) }
}

impl<T> Clone for Ptr<T> {
	fn clone(&self) -> Self { Self(self.0.clone()) }
}

impl<T> Deref for Ptr<T> {
	type Target = T;
	fn deref(&self) -> &Self::Target { unsafe { &*self.0.get() } }
}

impl<T> DerefMut for Ptr<T> {
	fn deref_mut(&mut self) -> &mut Self::Target { unsafe { &mut *self.0.get() } }
}

impl<T> PartialEq for Ptr<T> {
	fn eq(&self, other: &Self) -> bool { Rc::ptr_eq(&self.0, &other.0) }
}
