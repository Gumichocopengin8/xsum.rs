// CONSTANTS DEFINING THE FLOATING POINT FORMAT
pub const XSUM_MANTISSA_BITS: i64 = 52; // Bits in fp mantissa, excludes implict 1
pub const XSUM_EXP_BITS: i64 = 11; // Bits in fp exponent
pub const XSUM_MANTISSA_MASK: i64 = (1i64 << XSUM_MANTISSA_BITS) - 1; // Mask for mantissa bits
pub const XSUM_EXP_MASK: i64 = (1 << XSUM_EXP_BITS) - 1; // Mask for exponent
pub const XSUM_EXP_BIAS: i64 = (1 << (XSUM_EXP_BITS - 1)) - 1; // Bias added to signed exponent
pub const XSUM_SIGN_BIT: i64 = XSUM_MANTISSA_BITS + XSUM_EXP_BITS; // Position of sign bit
pub const XSUM_SIGN_MASK: u64 = 1u64 << XSUM_SIGN_BIT; // Mask for sign bit

// CONSTANTS DEFINING THE SMALL ACCUMULATOR FORMAT
pub const XSUM_SCHUNK_BITS: i64 = 64; // Bits in chunk of the small accumulator
pub const XSUM_LOW_EXP_BITS: i64 = 5; // # of low bits of exponent, in one chunk
pub const XSUM_LOW_EXP_MASK: i64 = (1 << XSUM_LOW_EXP_BITS) - 1; // Mask for low-order exponent bits
pub const XSUM_HIGH_EXP_BITS: i64 = XSUM_EXP_BITS - XSUM_LOW_EXP_BITS; // # of high exponent bits for index
pub const XSUM_SCHUNKS: i64 = (1 << XSUM_HIGH_EXP_BITS) + 3; // # of chunks in small accumulator
pub const XSUM_LOW_MANTISSA_BITS: i64 = 1 << XSUM_LOW_EXP_BITS; // Bits in low part of mantissa
pub const XSUM_LOW_MANTISSA_MASK: i64 = (1i64 << XSUM_LOW_MANTISSA_BITS) - 1; // Mask for low bits
pub const XSUM_SMALL_CARRY_BITS: i64 = (XSUM_SCHUNK_BITS - 1) - XSUM_MANTISSA_BITS; // Bits sums can carry into
pub const XSUM_SMALL_CARRY_TERMS: i64 = (1 << XSUM_SMALL_CARRY_BITS) - 1; // # terms can add before need prop.
