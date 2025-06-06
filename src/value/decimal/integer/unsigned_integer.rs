use std::{
	borrow::Borrow,
	fmt,
	ops::{Add, Div, Mul, Sub},
	str::FromStr,
};

use num_bigint::{BigInt, BigUint};
use num_traits::Zero;

use crate::{
	lexical::{self, LexicalFormOf},
	Datatype, NonNegativeInteger, NonNegativeIntegerDatatype, ParseXsd, UnsignedIntDatatype,
	UnsignedLongDatatype, UnsignedShortDatatype, XsdValue,
};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct UnsignedInteger(BigUint);

impl UnsignedInteger {
	/// Create a new unsigned integer from a `BigUint`.
	pub fn new(n: BigUint) -> Self {
		Self(n)
	}

	/// Create a new unsigned integer from a `BigUint`.
	///
	/// # Safety
	///
	/// The input number must be non-negative (which is always true for BigUint).
	pub unsafe fn new_unchecked(n: BigUint) -> Self {
		Self(n)
	}

	/// Creates an unsigned integer from its big endian bytes representation.
	pub fn from_bytes_be(bytes: &[u8]) -> Self {
		Self(BigUint::from_bytes_be(bytes))
	}

	/// Creates an unsigned integer from its little endian bytes representation.
	pub fn from_bytes_le(bytes: &[u8]) -> Self {
		Self(BigUint::from_bytes_le(bytes))
	}

	#[inline(always)]
	pub fn into_big_uint(self) -> BigUint {
		self.0
	}

	#[inline(always)]
	pub fn zero() -> Self {
		Self(BigUint::zero())
	}

	#[inline(always)]
	pub fn is_zero(&self) -> bool {
		self.0.is_zero()
	}

	pub fn unsigned_integer_type(&self) -> NonNegativeIntegerDatatype {
		if !self.is_zero() {
			if self.0 <= BigUint::from(u8::MAX) {
				UnsignedShortDatatype::UnsignedByte.into()
			} else if self.0 <= BigUint::from(u16::MAX) {
				UnsignedShortDatatype::UnsignedShort.into()
			} else if self.0 <= BigUint::from(u32::MAX) {
				UnsignedIntDatatype::UnsignedInt.into()
			} else if self.0 <= BigUint::from(u64::MAX) {
				UnsignedLongDatatype::UnsignedLong.into()
			} else {
				NonNegativeIntegerDatatype::PositiveInteger
			}
		} else {
			NonNegativeIntegerDatatype::NonNegativeInteger
		}
	}

	/// Returns a lexical representation of this unsigned integer.
	#[inline(always)]
	pub fn lexical_representation(&self) -> lexical::UnsignedIntegerBuf {
		unsafe {
			// This is safe because the `Display::fmt` method matches the
			// XSD lexical representation.
			lexical::UnsignedIntegerBuf::new_unchecked(format!("{}", self))
		}
	}

	pub fn to_bytes_be(&self) -> Vec<u8> {
		self.0.to_bytes_be()
	}

	pub fn to_bytes_le(&self) -> Vec<u8> {
		self.0.to_bytes_le()
	}
}

impl XsdValue for UnsignedInteger {
	fn datatype(&self) -> Datatype {
		self.unsigned_integer_type().into()
	}
}

impl ParseXsd for UnsignedInteger {
	type LexicalForm = lexical::UnsignedInteger;
}

impl LexicalFormOf<UnsignedInteger> for lexical::UnsignedInteger {
	type ValueError = std::convert::Infallible;

	fn try_as_value(&self) -> Result<UnsignedInteger, Self::ValueError> {
		Ok(self.value())
	}
}

impl From<UnsignedInteger> for BigUint {
	fn from(value: UnsignedInteger) -> Self {
		value.0
	}
}

impl<'a> From<&'a lexical::UnsignedInteger> for UnsignedInteger {
	#[inline(always)]
	fn from(value: &'a lexical::UnsignedInteger) -> Self {
		use num_traits::Num;
		Self(BigUint::from_str_radix(value.as_str(), 10).unwrap())
	}
}

impl From<lexical::UnsignedIntegerBuf> for UnsignedInteger {
	#[inline(always)]
	fn from(value: lexical::UnsignedIntegerBuf) -> Self {
		value.as_unsigned_integer().into()
	}
}

impl FromStr for UnsignedInteger {
	type Err = lexical::InvalidUnsignedInteger;

	#[inline(always)]
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let l = lexical::UnsignedInteger::new(s)?;
		Ok(l.into())
	}
}

impl fmt::Display for UnsignedInteger {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.fmt(f)
	}
}

impl AsRef<BigUint> for UnsignedInteger {
	#[inline(always)]
	fn as_ref(&self) -> &BigUint {
		&self.0
	}
}

impl Borrow<BigUint> for UnsignedInteger {
	#[inline(always)]
	fn borrow(&self) -> &BigUint {
		&self.0
	}
}

impl From<NonNegativeInteger> for UnsignedInteger {
	fn from(value: NonNegativeInteger) -> Self {
		let big_int: BigInt = value.into();
		Self(big_int.try_into().unwrap())
	}
}

impl From<UnsignedInteger> for NonNegativeInteger {
	fn from(value: UnsignedInteger) -> Self {
		unsafe {
			Self::new_unchecked(value.0.into())
		}
	}
}

macro_rules! from {
	{ $( $ty:ty ),* } => {
		$(
			impl From<$ty> for UnsignedInteger {
				fn from(value: $ty) -> Self {
					Self(value.into())
				}
			}
		)*
	};
}

from!(u8, u16, u32, u64, usize);

#[derive(Debug, thiserror::Error)]
#[error("unsigned integer out of supported bounds: {0}")]
pub struct UnsignedIntegerOutOfTargetBounds(pub UnsignedInteger);

macro_rules! try_into {
	{ $( $ty:ty ),* } => {
		$(
			impl TryFrom<UnsignedInteger> for $ty {
				type Error = UnsignedIntegerOutOfTargetBounds;

				fn try_from(value: UnsignedInteger) -> Result<Self, Self::Error> {
					let biguint = value.0.clone();
					biguint.try_into().map_err(|_| UnsignedIntegerOutOfTargetBounds(value))
				}
			}
		)*
	};
}

try_into!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

// Implement arithmetic operations for UnsignedInteger
impl Add for UnsignedInteger {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self(self.0 + rhs.0)
	}
}

impl Sub for UnsignedInteger {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		if self.0 < rhs.0 {
			panic!("attempt to subtract with overflow")
		}
		Self(self.0 - rhs.0)
	}
}

impl Mul for UnsignedInteger {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		Self(self.0 * rhs.0)
	}
}

impl Div for UnsignedInteger {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		if rhs.is_zero() {
			panic!("attempt to divide by zero")
		}
		Self(self.0 / rhs.0)
	}
}

// Implement other arithmetic operations for common integer types
macro_rules! impl_arithmetic_with_unsigned_types {
	{ $( $ty:ty ),* } => {
		$(
			impl Add<$ty> for UnsignedInteger {
				type Output = Self;

				fn add(self, rhs: $ty) -> Self::Output {
					Self(self.0 + rhs)
				}
			}

			impl Sub<$ty> for UnsignedInteger {
				type Output = Self;

				fn sub(self, rhs: $ty) -> Self::Output {
					if self.0 < BigUint::from(rhs) {
						panic!("attempt to subtract with overflow")
					}
					Self(self.0 - rhs)
				}
			}

			impl Mul<$ty> for UnsignedInteger {
				type Output = Self;

				fn mul(self, rhs: $ty) -> Self::Output {
					Self(self.0 * rhs)
				}
			}

			impl Div<$ty> for UnsignedInteger {
				type Output = Self;

				fn div(self, rhs: $ty) -> Self::Output {
					if rhs == 0 {
						panic!("attempt to divide by zero")
					}
					Self(self.0 / rhs)
				}
			}
		)*
	};
}

impl_arithmetic_with_unsigned_types!(u8, u16, u32, u64, usize);