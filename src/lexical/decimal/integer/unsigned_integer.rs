use crate::lexical::lexical_form;

use super::{
	Decimal, DecimalBuf, Integer, IntegerBuf, NonNegativeInteger, NonNegativeIntegerBuf, Overflow,
	Sign,
};
use std::borrow::{Borrow, ToOwned};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

lexical_form! {
	/// Unsigned integer number.
	///
	/// See: <http://www.w3.org/2001/XMLSchema#unsignedInt>
	ty: UnsignedInteger,

	/// Owned unsigned integer number.
	///
	/// See: <http://www.w3.org/2001/XMLSchema#unsignedInt>
	buffer: UnsignedIntegerBuf,

	/// Creates a new unsigned integer from a string.
	///
	/// The input string must be a [valid XSD unsignedInt](http://www.w3.org/2001/XMLSchema#unsignedInt).
	/// an [`InvalidUnsignedInteger`] error is returned.
	new,

	/// Creates a new unsigned integer from a string without checking it.
	///
	/// # Safety
	///
	/// The input string must be a [valid XSD unsignedInt](http://www.w3.org/2001/XMLSchema#unsignedInt).
	new_unchecked,

	value: crate::UnsignedInteger,
	error: InvalidUnsignedInteger,
	as_ref: as_unsigned_integer,
	parent_forms: {
		as_non_negative_integer: NonNegativeInteger, NonNegativeIntegerBuf,
		as_integer: Integer, IntegerBuf,
		as_decimal: Decimal, DecimalBuf
	}
}

impl UnsignedInteger {
	/// Returns `true` if `self` is positive
	/// and `false` if the number is zero.
	pub fn is_positive(&self) -> bool {
		for c in &self.0 {
			match c {
				b'+' | b'0' => (),
				_ => return true,
			}
		}

		false
	}

	/// Returns `true` if `self` is zero
	/// and `false` otherwise.
	pub fn is_zero(&self) -> bool {
		for c in &self.0 {
			if !matches!(c, b'+' | b'0') {
				return false;
			}
		}

		true
	}

	pub fn sign(&self) -> Sign {
		for c in &self.0 {
			match c {
				b'+' | b'0' => (),
				_ => return Sign::Positive,
			}
		}

		Sign::Zero
	}

	/// Returns the canonical form of `self` (without leading zeros).
	pub fn canonical(&self) -> &Self {
		let mut last_zero = 0;
		for (i, c) in self.0.iter().enumerate() {
			match c {
				b'+' => (),
				b'0' => last_zero = i,
				_ => return unsafe { Self::new_unchecked(&self.0[i..]) },
			}
		}

		unsafe { Self::new_unchecked(&self.0[last_zero..]) }
	}

	#[inline(always)]
	fn as_canonical_str(&self) -> &str {
		self.canonical().as_str()
	}

	#[inline(always)]
	pub fn value(&self) -> crate::UnsignedInteger {
		use num_bigint::BigUint;
		use num_traits::Num;
		crate::UnsignedInteger::new(BigUint::from_str_radix(self.as_str(), 10).unwrap())
	}
}

impl PartialEq for UnsignedInteger {
	fn eq(&self, other: &Self) -> bool {
		self.as_canonical_str() == other.as_canonical_str()
	}
}

impl Eq for UnsignedInteger {}

impl Hash for UnsignedInteger {
	fn hash<H: Hasher>(&self, h: &mut H) {
		self.as_canonical_str().hash(h)
	}
}

impl Ord for UnsignedInteger {
	fn cmp(&self, other: &Self) -> Ordering {
		let a = &self.canonical().0;
		let b = &other.canonical().0;

		match a.len().cmp(&b.len()) {
			Ordering::Equal => a.cmp(b),
			other => other,
		}
	}
}

impl PartialOrd for UnsignedInteger {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl UnsignedIntegerBuf {
	pub fn zero() -> Self {
		unsafe { Self::new_unchecked("0".to_string()) }
	}

	pub fn one() -> Self {
		unsafe { Self::new_unchecked("1".to_string()) }
	}
}

impl Default for UnsignedIntegerBuf {
	fn default() -> Self {
		Self::zero()
	}
}

impl PartialOrd for UnsignedIntegerBuf {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for UnsignedIntegerBuf {
	fn cmp(&self, other: &Self) -> Ordering {
		self.as_integer().cmp(other.as_integer())
	}
}

macro_rules! number_conversion {
	{ $($ty:ty),* } => {
		$(
			impl From<$ty> for UnsignedIntegerBuf {
				fn from(i: $ty) -> Self {
					unsafe { UnsignedIntegerBuf::new_unchecked(i.to_string()) }
				}
			}

			impl<'a> TryFrom<&'a UnsignedInteger> for $ty {
				type Error = Overflow;

				fn try_from(i: &'a UnsignedInteger) -> Result<Self, Overflow> {
					i.as_str().parse().map_err(|_| Overflow)
				}
			}

			impl TryFrom<UnsignedIntegerBuf> for $ty {
				type Error = Overflow;

				fn try_from(i: UnsignedIntegerBuf) -> Result<Self, Overflow> {
					i.as_str().parse().map_err(|_| Overflow)
				}
			}
		)*
	};
}

number_conversion! {
	u8,
	i8,
	u16,
	i16,
	u32,
	i32,
	u64,
	i64,
	u128,
	i128,
	usize,
	isize
}

fn check_bytes(s: &[u8]) -> bool {
	check(s.iter().copied())
}

fn check<C: Iterator<Item = u8>>(mut chars: C) -> bool {
	enum State {
		Initial,
		NonEmptyInteger,
		Integer,
		Zero,
	}

	let mut state = State::Initial;

	loop {
		state = match state {
			State::Initial => match chars.next() {
				Some(b'+') => State::NonEmptyInteger,
				Some(b'-') => State::Zero,
				Some(b'0'..=b'9') => State::Integer,
				_ => break false,
			},
			State::NonEmptyInteger => match chars.next() {
				Some(b'0'..=b'9') => State::Integer,
				_ => break false,
			},
			State::Integer => match chars.next() {
				Some(b'0'..=b'9') => State::Integer,
				Some(_) => break false,
				None => break true,
			},
			State::Zero => match chars.next() {
				Some(b'0') => State::Zero,
				_ => break false,
			},
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_01() {
		UnsignedInteger::new("0").unwrap();
	}

	#[test]
	fn parse_02() {
		UnsignedInteger::new("+42").unwrap();
	}

	#[test]
	#[should_panic]
	fn parse_03() {
		UnsignedInteger::new("-42").unwrap();
	}

	#[test]
	fn canonical_01() {
		assert_eq!(
			UnsignedInteger::new("01").unwrap().canonical().as_str(),
			"1"
		)
	}

	#[test]
	fn canonical_02() {
		assert_eq!(
			UnsignedInteger::new("00").unwrap().canonical().as_str(),
			"0"
		)
	}

	#[test]
	fn canonical_03() {
		assert_eq!(
			UnsignedInteger::new("+00000").unwrap().canonical().as_str(),
			"0"
		)
	}

	#[test]
	fn eq_01() {
		assert_eq!(
			UnsignedInteger::new("+001").unwrap(),
			UnsignedInteger::new("1").unwrap()
		)
	}

	#[test]
	fn cmp_01() {
		assert!(UnsignedInteger::new("123").unwrap() < UnsignedInteger::new("456").unwrap())
	}

	#[test]
	fn cmp_02() {
		assert!(UnsignedInteger::new("1230").unwrap() > UnsignedInteger::new("456").unwrap())
	}

	#[test]
	fn cmp_03() {
		assert_eq!(
			UnsignedInteger::new("+123456")
				.unwrap()
				.cmp(UnsignedInteger::new("0000123456").unwrap()),
			Ordering::Equal
		)
	}
}
