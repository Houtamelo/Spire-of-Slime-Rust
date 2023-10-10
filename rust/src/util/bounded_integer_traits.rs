use bounded_integer::BoundedIsize;

pub trait FromIsize { 
	fn from_isize(value: isize) -> Self;
}

impl FromIsize for BoundedIsize< -100, 300> { fn from_isize(value: isize) -> Self { return <BoundedIsize< -100, 300>>::new_saturating(value); } }
impl FromIsize for BoundedIsize< -100, 100> { fn from_isize(value: isize) -> Self { return <BoundedIsize< -100, 100>>::new_saturating(value); } }
impl FromIsize for BoundedIsize< -300, 300> { fn from_isize(value: isize) -> Self { return <BoundedIsize< -300, 300>>::new_saturating(value); } }
impl FromIsize for BoundedIsize<   20, 300> { fn from_isize(value: isize) -> Self { return <BoundedIsize<   20, 300>>::new_saturating(value); } }
impl FromIsize for BoundedIsize<    0, 500> { fn from_isize(value: isize) -> Self { return <BoundedIsize<    0, 500>>::new_saturating(value); } }
impl FromIsize for BoundedIsize<    0, 200> { fn from_isize(value: isize) -> Self { return <BoundedIsize<    0, 200>>::new_saturating(value); } }
impl FromIsize for BoundedIsize<    0, 100> { fn from_isize(value: isize) -> Self { return <BoundedIsize<    0, 100>>::new_saturating(value); } }


pub trait ToBounded { 
	fn bind_m100_p300(&self) -> BoundedIsize< -100, 300>;
	fn bind_m100_p100(&self) -> BoundedIsize< -100, 100>;
	fn bind_m300_p300(&self) -> BoundedIsize< -300, 300>;
	fn bind_p20_p300 (&self) -> BoundedIsize<   20, 300>;
	fn bind_0_p500   (&self) -> BoundedIsize<    0, 500>;
	fn bind_0_p200   (&self) -> BoundedIsize<    0, 200>;
	fn bind_0_p100   (&self) -> BoundedIsize<    0, 100>;
}

impl ToBounded for isize { 
	fn bind_m100_p300(&self) -> BoundedIsize< -100, 300> { return BoundedIsize::< -100, 300>::new_saturating(*self); }
	fn bind_m100_p100(&self) -> BoundedIsize< -100, 100> { return BoundedIsize::< -100, 100>::new_saturating(*self); }
	fn bind_m300_p300(&self) -> BoundedIsize< -300, 300> { return BoundedIsize::< -300, 300>::new_saturating(*self); }
	fn bind_p20_p300 (&self) -> BoundedIsize<   20, 300> { return BoundedIsize::<   20, 300>::new_saturating(*self); }
	fn bind_0_p500   (&self) -> BoundedIsize<    0, 500> { return BoundedIsize::<    0, 500>::new_saturating(*self); }
	fn bind_0_p200   (&self) -> BoundedIsize<    0, 200> { return BoundedIsize::<    0, 200>::new_saturating(*self); }
	fn bind_0_p100   (&self) -> BoundedIsize<    0, 100> { return BoundedIsize::<    0, 100>::new_saturating(*self); }
}

impl ToBounded for usize { 
	fn bind_m100_p300(&self) -> BoundedIsize< -100, 300> { return BoundedIsize::< -100, 300>::new_saturating(*self as isize); }
	fn bind_m100_p100(&self) -> BoundedIsize< -100, 100> { return BoundedIsize::< -100, 100>::new_saturating(*self as isize); }
	fn bind_m300_p300(&self) -> BoundedIsize< -300, 300> { return BoundedIsize::< -300, 300>::new_saturating(*self as isize); }
	fn bind_p20_p300 (&self) -> BoundedIsize<   20, 300> { return BoundedIsize::<   20, 300>::new_saturating(*self as isize); }
	fn bind_0_p500   (&self) -> BoundedIsize<    0, 500> { return BoundedIsize::<    0, 500>::new_saturating(*self as isize); }
	fn bind_0_p200   (&self) -> BoundedIsize<    0, 200> { return BoundedIsize::<    0, 200>::new_saturating(*self as isize); }
	fn bind_0_p100   (&self) -> BoundedIsize<    0, 100> { return BoundedIsize::<    0, 100>::new_saturating(*self as isize); }
}

impl ToBounded for i32 {
	fn bind_m100_p300(&self) -> BoundedIsize< -100, 300> { return BoundedIsize::< -100, 300>::new_saturating(*self as isize); }
	fn bind_m100_p100(&self) -> BoundedIsize< -100, 100> { return BoundedIsize::< -100, 100>::new_saturating(*self as isize); }
	fn bind_m300_p300(&self) -> BoundedIsize< -300, 300> { return BoundedIsize::< -300, 300>::new_saturating(*self as isize); }
	fn bind_p20_p300 (&self) -> BoundedIsize<   20, 300> { return BoundedIsize::<   20, 300>::new_saturating(*self as isize); }
	fn bind_0_p500   (&self) -> BoundedIsize<    0, 500> { return BoundedIsize::<    0, 500>::new_saturating(*self as isize); }
	fn bind_0_p200   (&self) -> BoundedIsize<    0, 200> { return BoundedIsize::<    0, 200>::new_saturating(*self as isize); }
	fn bind_0_p100   (&self) -> BoundedIsize<    0, 100> { return BoundedIsize::<    0, 100>::new_saturating(*self as isize); }
}