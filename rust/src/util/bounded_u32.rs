#[derive(Debug, Clone, Copy, Hash)]
pub struct BoundU32<const MIN: u32, const MAX: u32> { inner_value: u32 }

macro_rules! bounded_u32 {
    ($MIN: literal, $MAX: literal) => {

	    impl BoundU32<$MIN, $MAX> {
		    pub fn new(inner_value: u32) -> Self {
			    return Self { inner_value: inner_value.clamp($MIN, $MAX) };
		    }

		    pub fn get(&self) -> u32 {
			    return self.inner_value;
		    }
	    }

	    impl std::ops::Add for BoundU32<$MIN, $MAX> {
		    type Output = Self;

		    fn add(self, other: Self) -> Self {
			    return Self::new(self.inner_value + other.inner_value);
		    }
	    }

	    impl std::ops::Add<u32> for BoundU32<$MIN, $MAX> {
		    type Output = Self;

		    fn add(self, other: u32) -> Self {
			    return Self::new(self.inner_value + other);
		    }
	    }

	    impl std::ops::Add<isize> for BoundU32<$MIN, $MAX> {
		    type Output = Self;

		    fn add(self, other: isize) -> Self {
			    let mut result = self.inner_value as isize + other;
			    if result < 0 {
				    result = 0;
			    }

			    return Self::new(result as u32);
		    }
	    }

	    impl std::ops::Add<i32> for BoundU32<$MIN, $MAX> {
		    type Output = Self;

		    fn add(self, other: i32) -> Self {
			    let mut result = self.inner_value as i32 + other;
			    if result < 0 {
				    result = 0;
			    }

			    return Self::new(result as u32);
		    }
	    }

	    impl std::ops::AddAssign for BoundU32<$MIN, $MAX> {
		    fn add_assign(&mut self, other: Self) {
			    self.inner_value = (self.inner_value + other.inner_value).clamp($MIN, $MAX);
		    }
	    }

	    impl std::ops::AddAssign<u32> for BoundU32<$MIN, $MAX> {
		    fn add_assign(&mut self, other: u32) {
			    self.inner_value = (self.inner_value + other).clamp($MIN, $MAX);
		    }
	    }

	    impl std::ops::AddAssign<isize> for BoundU32<$MIN, $MAX> {
		    fn add_assign(&mut self, other: isize) {
			    let mut result = self.inner_value as isize + other;
			    if result < 0 {
				    result = 0;
			    }

			    self.inner_value = (result as u32).clamp($MIN, $MAX);
		    }
	    }

	    impl std::ops::AddAssign<i32> for BoundU32<$MIN, $MAX> {
		    fn add_assign(&mut self, other: i32) {
			    let mut result = self.inner_value as i32 + other;
			    if result < 0 {
				    result = 0;
			    }

			    self.inner_value = (result as u32).clamp($MIN, $MAX);
		    }
	    }

	    impl std::ops::Sub for BoundU32<$MIN, $MAX> {
		    type Output = Self;

		    fn sub(self, other: Self) -> Self {
			    return Self::new(self.inner_value - other.inner_value);
		    }
	    }

	    impl std::ops::Sub<u32> for BoundU32<$MIN, $MAX> {
		    type Output = Self;

		    fn sub(self, other: u32) -> Self {
			    return Self::new(self.inner_value - other);
		    }
	    }

	    impl std::ops::Sub<isize> for BoundU32<$MIN, $MAX> {
		    type Output = Self;

		    fn sub(self, other: isize) -> Self {
			    let mut result = self.inner_value as isize - other;
			    if result < 0 {
				    result = 0;
			    }

			    return Self::new(result as u32);
		    }
	    }

	    impl std::ops::Sub<i32> for BoundU32<$MIN, $MAX> {
		    type Output = Self;

		    fn sub(self, other: i32) -> Self {
			    let mut result = self.inner_value as i32 - other;
			    if result < 0 {
				    result = 0;
			    }

			    return Self::new(result as u32);
		    }
	    }

	    impl std::ops::SubAssign for BoundU32<$MIN, $MAX> {
		    fn sub_assign(&mut self, other: Self) {
			    self.inner_value = (self.inner_value - other.inner_value).clamp($MIN, $MAX);
		    }
	    }

	    impl std::ops::SubAssign<u32> for BoundU32<$MIN, $MAX> {
		    fn sub_assign(&mut self, other: u32) {
			    self.inner_value = (self.inner_value - other).clamp($MIN, $MAX);
		    }
	    }

	    impl std::ops::SubAssign<isize> for BoundU32<$MIN, $MAX> {
		    fn sub_assign(&mut self, other: isize) {
			    let mut result = self.inner_value as isize - other;
			    if result < 0 {
				    result = 0;
			    }

			    self.inner_value = (result as u32).clamp($MIN, $MAX);
		    }
	    }

	    impl std::ops::SubAssign<i32> for BoundU32<$MIN, $MAX> {
		    fn sub_assign(&mut self, other: i32) {
			    let mut result = self.inner_value as i32 - other;
			    if result < 0 {
				    result = 0;
			    }

			    self.inner_value = (result as u32).clamp($MIN, $MAX);
		    }
	    }

	    impl std::ops::Div for BoundU32<$MIN, $MAX> {
		    type Output = Self;

		    fn div(self, other: Self) -> Self {
			    return Self::new(self.inner_value / other.inner_value);
		    }
	    }

	    impl std::ops::Div<u32> for BoundU32<$MIN, $MAX> {
		    type Output = Self;

		    fn div(self, other: u32) -> Self {
			    return Self::new(self.inner_value / other);
		    }
	    }

	    impl std::ops::Div<isize> for BoundU32<$MIN, $MAX> {
		    type Output = Self;

		    fn div(self, other: isize) -> Self {
			    let mut result = self.inner_value as isize / other;
			    if result < 0 {
				    result = 0;
			    }

			    return Self::new(result as u32);
		    }
	    }

	    impl std::ops::Div<i32> for BoundU32<$MIN, $MAX> {
		    type Output = Self;

		    fn div(self, other: i32) -> Self {
			    let mut result = self.inner_value as i32 / other;
			    if result < 0 {
				    result = 0;
			    }

			    return Self::new(result as u32);
		    }
	    }

	    impl std::ops::DivAssign for BoundU32<$MIN, $MAX> {
		    fn div_assign(&mut self, other: Self) {
			    self.inner_value = (self.inner_value / other.inner_value).clamp($MIN, $MAX);
		    }
	    }

	    impl std::ops::DivAssign<u32> for BoundU32<$MIN, $MAX> {
		    fn div_assign(&mut self, other: u32) {
			    self.inner_value = (self.inner_value / other).clamp($MIN, $MAX);
		    }
	    }

	    impl std::ops::DivAssign<isize> for BoundU32<$MIN, $MAX> {
		    fn div_assign(&mut self, other: isize) {
			    let mut result = self.inner_value as isize / other;
			    if result < 0 {
				    result = 0;
			    }

			    self.inner_value = (result as u32).clamp($MIN, $MAX);
		    }
	    }

	    impl std::ops::DivAssign<i32> for BoundU32<$MIN, $MAX> {
		    fn div_assign(&mut self, other: i32) {
			    let mut result = self.inner_value as i32 / other;
			    if result < 0 {
				    result = 0;
			    }

			    self.inner_value = (result as u32).clamp($MIN, $MAX);
		    }
	    }

	    impl std::ops::Mul for BoundU32<$MIN, $MAX> {
		    type Output = Self;

		    fn mul(self, other: Self) -> Self {
			    return Self::new(self.inner_value * other.inner_value);
		    }
	    }

	    impl std::ops::Mul<u32> for BoundU32<$MIN, $MAX> {
		    type Output = Self;

		    fn mul(self, other: u32) -> Self {
			    return Self::new(self.inner_value * other);
		    }
	    }

	    impl std::ops::Mul<isize> for BoundU32<$MIN, $MAX> {
		    type Output = Self;

		    fn mul(self, other: isize) -> Self {
			    let mut result = self.inner_value as isize * other;
			    if result < 0 {
				    result = 0;
			    }

			    return Self::new(result as u32);
		    }
	    }

	    impl std::ops::Mul<i32> for BoundU32<$MIN, $MAX> {
		    type Output = Self;

		    fn mul(self, other: i32) -> Self {
			    let mut result = self.inner_value as i32 * other;
			    if result < 0 {
				    result = 0;
			    }

			    return Self::new(result as u32);
		    }
	    }

	    impl std::ops::MulAssign for BoundU32<$MIN, $MAX> {
		    fn mul_assign(&mut self, other: Self) {
			    self.inner_value = (self.inner_value * other.inner_value).clamp($MIN, $MAX);
		    }
	    }

	    impl std::ops::MulAssign<u32> for BoundU32<$MIN, $MAX> {
		    fn mul_assign(&mut self, other: u32) {
			    self.inner_value = (self.inner_value * other).clamp($MIN, $MAX);
		    }
	    }

		impl std::ops::MulAssign<isize> for BoundU32<$MIN, $MAX> {
		    fn mul_assign(&mut self, other: isize) {
			    let mut result = self.inner_value as isize * other;
			    if result < 0 {
				    result = 0;
			    }

			    self.inner_value = (result as u32).clamp($MIN, $MAX);
		    }
	    }

	    impl std::ops::MulAssign<i32> for BoundU32<$MIN, $MAX> {
		    fn mul_assign(&mut self, other: i32) {
			    let mut result = self.inner_value as i32 * other;
			    if result < 0 {
				    result = 0;
			    }

			    self.inner_value = (result as u32).clamp($MIN, $MAX);
		    }
	    }

	    impl std::ops::Deref for BoundU32<$MIN, $MAX> {
		    type Target = u32;

		    fn deref(&self) -> &Self::Target {
			    return &self.inner_value;
		    }
	    }

	    impl std::ops::DerefMut for BoundU32<$MIN, $MAX> {
		    fn deref_mut(&mut self) -> &mut Self::Target {
			    return &mut self.inner_value;
		    }
	    }

	    impl PartialEq<Self> for BoundU32<$MIN, $MAX> {
		    fn eq(&self, other: &Self) -> bool {
			    return self.inner_value == other.inner_value;
		    }
	    }

	    impl PartialEq<u32> for BoundU32<$MIN, $MAX> {
		    fn eq(&self, other: &u32) -> bool {
			    return self.inner_value == *other;
		    }
	    }

	    impl PartialEq<isize> for BoundU32<$MIN, $MAX> {
		    fn eq(&self, other: &isize) -> bool {
			    if *other < 0 {
				    return false;
			    }

			    return self.inner_value == *other as u32;
		    }
	    }

	    impl PartialEq<i32> for BoundU32<$MIN, $MAX> {
		    fn eq(&self, other: &i32) -> bool {
			    if *other < 0 {
				    return false;
			    }

			    return self.inner_value == *other as u32;
		    }
	    }

	    impl PartialEq<BoundU32<$MIN, $MAX>> for u32 {
		    fn eq(&self, other: &BoundU32<$MIN, $MAX>) -> bool {
			    return *self == other.inner_value;
		    }
	    }

	    impl Eq for BoundU32<$MIN, $MAX> {}

	    impl PartialOrd<Self> for BoundU32<$MIN, $MAX> {
		    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
			    return self.inner_value.partial_cmp(&other.inner_value);
		    }
	    }

	    impl PartialOrd<u32> for BoundU32<$MIN, $MAX> {
		    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
			    return self.inner_value.partial_cmp(other);
		    }
	    }

	    impl PartialOrd<isize> for BoundU32<$MIN, $MAX> {
		    fn partial_cmp(&self, other: &isize) -> Option<std::cmp::Ordering> {
			    if *other < 0 {
				    return Some(std::cmp::Ordering::Less);
			    }

			    return self.inner_value.partial_cmp(&(*other as u32));
		    }
	    }

	    impl PartialOrd<i32> for BoundU32<$MIN, $MAX> {
		    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
			    if *other < 0 {
				    return Some(std::cmp::Ordering::Less);
			    }

			    return self.inner_value.partial_cmp(&(*other as u32));
		    }
	    }

	    impl From<u32> for BoundU32<$MIN, $MAX> {
		    fn from(value: u32) -> Self {
			    return Self::new(value);
		    }
	    }

	    impl From<usize> for BoundU32<$MIN, $MAX> {
		    fn from(value: usize) -> Self {
			    return Self::new(value as u32);
		    }
	    }

	    impl From<isize> for BoundU32<$MIN, $MAX> {
		    fn from(value: isize) -> Self {
			    if value < 0 {
				    return Self::new(0);
			    } else {
				    return Self::new(value as u32);
			    }
		    }
	    }

	    impl From<i32> for BoundU32<$MIN, $MAX> {
		    fn from(value: i32) -> Self {
			    if value < 0 {
				    return Self::new(0);
			    } else {
				    return Self::new(value as u32);
			    }
		    }
	    }
    };
}

bounded_u32!(20, 300);
bounded_u32!( 0, 500);
bounded_u32!( 0, 200);
bounded_u32!( 0, 100);
bounded_u32!( 1, 500);
bounded_u32!( 1, 4);
bounded_u32!( 1, 8);
bounded_u32!( 0, 4);
bounded_u32!( 0, 5);
bounded_u32!( 0, 6);

// Creating a new BoundU32 instance with a valid inner value should return the instance with the same inner value.
#[test]
fn test_new_instance_with_valid_inner_value() {
	let bound_u32 = BoundU32::<20, 300>::new(50);
	assert_eq!(bound_u32.get(), 50);
}

// Adding two BoundU32 instances should return a new BoundU32 instance with the sum of their inner values.
#[test]
fn test_add_two_instances() {
	let bound_u32_1 = BoundU32::<20, 300>::new(50);
	let bound_u32_2 = BoundU32::<20, 300>::new(100);
	let result = bound_u32_1 + bound_u32_2;
	assert_eq!(result.get(), 150);
}

// Adding a BoundU32 instance and a u32 value should return a new BoundU32 instance with the sum of their inner values.
#[test]
fn test_add_instance_and_u32_value() {
	let bound_u32 = BoundU32::<20, 300>::new(50);
	let result: BoundU32<20, 300> = bound_u32 + 100;
	assert_eq!(result.get(), 150);
}

// Creating a new BoundU32 instance with the minimum allowed inner value should return the instance with the same inner value.
#[test]
fn test_new_instance_with_minimum_inner_value() {
	let bound_u32 = BoundU32::<20, 300>::new(20);
	assert_eq!(bound_u32.get(), 20);
}

// Creating a new BoundU32 instance with the maximum allowed inner value should return the instance with the same inner value.
#[test]
fn test_new_instance_with_maximum_inner_value() {
	let bound_u32 = BoundU32::<20, 300>::new(300);
	assert_eq!(bound_u32.get(), 300);
}

// Adding two BoundU32 instances with the maximum allowed inner values should return a new BoundU32 instance with the sum of their inner values, clamped to the range [MIN, MAX].
#[test]
fn test_add_two_instances_with_maximum_inner_values() {
	let bound_u32_1 = BoundU32::<20, 300>::new(300);
	let bound_u32_2 = BoundU32::<20, 300>::new(200);
	let result = bound_u32_1 + bound_u32_2;
	assert_eq!(result.get(), 300);
}


// Creating a new BoundU32 instance with a valid inner value should return the instance with the same inner value.
#[test]
fn create_new_instance_with_valid_inner_value() {
	let bound_u32 = BoundU32::<20, 300>::new(50);
	assert_eq!(bound_u32.get(), 50);
}

// Adding two BoundU32 instances should return a new BoundU32 instance with the sum of their inner values.
#[test]
fn add_two_instances() {
	let bound_u32_1 = BoundU32::<20, 300>::new(50);
	let bound_u32_2 = BoundU32::<20, 300>::new(100);
	let result = bound_u32_1 + bound_u32_2;
	assert_eq!(result.get(), 150);
}

// Adding a BoundU32 instance and a u32 value should return a new BoundU32 instance with the sum of their inner values.
#[test]
fn add_instance_and_u32_value() {
	let bound_u32 = BoundU32::<20, 300>::new(50);
	let result: BoundU32<20, 300> = bound_u32 + 100;
	assert_eq!(result.get(), 150);
}

// Adding a BoundU32 instance and an isize value should return a new BoundU32 instance with the sum of their inner values, clamped to the range [0, MAX].
#[test]
fn add_instance_and_isize_value() {
	let bound_u32 = BoundU32::<20, 300>::new(50);
	let result = bound_u32 + 100isize;
	assert_eq!(result.get(), 150);
}

// Adding a BoundU32 instance and an i32 value should return a new BoundU32 instance with the sum of their inner values, clamped to the range [0, MAX].
#[test]
fn add_instance_and_i32_value() {
	let bound_u32 = BoundU32::<20, 300>::new(50);
	let result = bound_u32 + 100i32;
	assert_eq!(result.get(), 150);
}

// Creating a new BoundU32 instance with the minimum allowed inner value should return the instance with the same inner value.
#[test]
fn create_new_instance_with_minimum_inner_value() {
	let bound_u32 = BoundU32::<20, 300>::new(20);
	assert_eq!(bound_u32.get(), 20);
}

// Creating a new BoundU32 instance with the maximum allowed inner value should return the instance with the same inner value.
#[test]
fn create_new_instance_with_maximum_inner_value() {
	let bound_u32 = BoundU32::<20, 300>::new(300);
	assert_eq!(bound_u32.get(), 300);
}

// Adding two BoundU32 instances with the maximum allowed inner values should return a new BoundU32 instance with the sum of their inner values, clamped to the range [MIN, MAX].
#[test]
fn add_two_instances_with_maximum_inner_values() {
	let bound_u32_1 = BoundU32::<20, 300>::new(300);
	let bound_u32_2 = BoundU32::<20, 300>::new(200);
	let result = bound_u32_1 + bound_u32_2;
	assert_eq!(result.get(), 300);
}

// Adding a BoundU32 instance and a u32 value that would result in an overflow should return a new BoundU32 instance with the maximum allowed inner value.
#[test]
fn add_instance_and_overflowing_u32_value() {
	let bound_u32 = BoundU32::<20, 300>::new(250);
	let result = bound_u32 + 100u32;
	assert_eq!(result.get(), 300);
}

// Adding a BoundU32 instance and an isize value that would result in an overflow should return a new BoundU32 instance with the maximum allowed inner value.
#[test]
fn add_instance_and_overflowing_isize_value() {
	let bound_u32 = BoundU32::<20, 300>::new(250);
	let result = bound_u32 + 100000isize;
	assert_eq!(result.get(), 300);
}


