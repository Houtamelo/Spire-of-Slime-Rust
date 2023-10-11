use bounded_integer::BoundedU32;

pub trait FromU32 {
	fn from_u32(value: u32) -> Self;
}

impl FromU32 for BoundedU32<   20, 300> { fn from_u32(value: u32) -> Self { return <BoundedU32<   20, 300>>::new_saturating(value); } }
impl FromU32 for BoundedU32<    0, 500> { fn from_u32(value: u32) -> Self { return <BoundedU32<    0, 500>>::new_saturating(value); } }
impl FromU32 for BoundedU32<    0, 200> { fn from_u32(value: u32) -> Self { return <BoundedU32<    0, 200>>::new_saturating(value); } }
impl FromU32 for BoundedU32<    0, 100> { fn from_u32(value: u32) -> Self { return <BoundedU32<    0, 100>>::new_saturating(value); } }


pub trait ToBounded {
	fn bind_p20_p300 (&self) -> BoundedU32<   20, 300>;
	fn bind_0_p500   (&self) -> BoundedU32<    0, 500>;
	fn bind_0_p200   (&self) -> BoundedU32<    0, 200>;
	fn bind_0_p100   (&self) -> BoundedU32<    0, 100>;
}

impl ToBounded for u32 {
	fn bind_p20_p300 (&self) -> BoundedU32<   20, 300> { return BoundedU32::<   20, 300>::new_saturating(*self); }
	fn bind_0_p500   (&self) -> BoundedU32<    0, 500> { return BoundedU32::<    0, 500>::new_saturating(*self); }
	fn bind_0_p200   (&self) -> BoundedU32<    0, 200> { return BoundedU32::<    0, 200>::new_saturating(*self); }
	fn bind_0_p100   (&self) -> BoundedU32<    0, 100> { return BoundedU32::<    0, 100>::new_saturating(*self); }
}

impl ToBounded for usize {
	fn bind_p20_p300 (&self) -> BoundedU32<   20, 300> { return BoundedU32::<   20, 300>::new_saturating(*self as u32); }
	fn bind_0_p500   (&self) -> BoundedU32<    0, 500> { return BoundedU32::<    0, 500>::new_saturating(*self as u32); }
	fn bind_0_p200   (&self) -> BoundedU32<    0, 200> { return BoundedU32::<    0, 200>::new_saturating(*self as u32); }
	fn bind_0_p100   (&self) -> BoundedU32<    0, 100> { return BoundedU32::<    0, 100>::new_saturating(*self as u32); }
}

impl ToBounded for i32 {
	fn bind_p20_p300 (&self) -> BoundedU32<   20, 300> { return BoundedU32::<   20, 300>::new_saturating(*self as u32); }
	fn bind_0_p500   (&self) -> BoundedU32<    0, 500> { return BoundedU32::<    0, 500>::new_saturating(*self as u32); }
	fn bind_0_p200   (&self) -> BoundedU32<    0, 200> { return BoundedU32::<    0, 200>::new_saturating(*self as u32); }
	fn bind_0_p100   (&self) -> BoundedU32<    0, 100> { return BoundedU32::<    0, 100>::new_saturating(*self as u32); }
}

impl ToBounded for isize {
	fn bind_p20_p300 (&self) -> BoundedU32<   20, 300> { return BoundedU32::<   20, 300>::new_saturating(*self as u32); }
	fn bind_0_p500   (&self) -> BoundedU32<    0, 500> { return BoundedU32::<    0, 500>::new_saturating(*self as u32); }
	fn bind_0_p200   (&self) -> BoundedU32<    0, 200> { return BoundedU32::<    0, 200>::new_saturating(*self as u32); }
	fn bind_0_p100   (&self) -> BoundedU32<    0, 100> { return BoundedU32::<    0, 100>::new_saturating(*self as u32); }
}