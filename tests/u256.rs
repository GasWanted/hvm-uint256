use hvm::hvm::{U256Val, Numb, TY_U256};

#[test]
fn test_u256_add_basic() {
  let a = U256Val::from_u32(100);
  let b = U256Val::from_u32(200);
  let c = U256Val::add(&a, &b);
  assert_eq!(c.limbs[0], 300);
  assert_eq!(c.limbs[1], 0);
}

#[test]
fn test_u256_add_carry() {
  let a = U256Val { limbs: [u32::MAX, 0, 0, 0, 0, 0, 0, 0] };
  let b = U256Val::from_u32(1);
  let c = U256Val::add(&a, &b);
  assert_eq!(c.limbs[0], 0);
  assert_eq!(c.limbs[1], 1);
}

#[test]
fn test_u256_add_overflow_wraps() {
  let c = U256Val::add(&U256Val::MAX, &U256Val::ONE);
  assert_eq!(c, U256Val::ZERO);
}

#[test]
fn test_u256_sub_basic() {
  let a = U256Val::from_u32(300);
  let b = U256Val::from_u32(100);
  let c = U256Val::sub(&a, &b);
  assert_eq!(c.limbs[0], 200);
}

#[test]
fn test_u256_sub_underflow_wraps() {
  let c = U256Val::sub(&U256Val::ZERO, &U256Val::ONE);
  assert_eq!(c, U256Val::MAX);
}

#[test]
fn test_u256_mul_basic() {
  let a = U256Val::from_u32(1000);
  let b = U256Val::from_u32(2000);
  let c = U256Val::mul(&a, &b);
  assert_eq!(c.limbs[0], 2_000_000);
}

#[test]
fn test_u256_mul_cross_limb() {
  let a = U256Val { limbs: [0, 1, 0, 0, 0, 0, 0, 0] }; // 2^32
  let b = U256Val { limbs: [0, 1, 0, 0, 0, 0, 0, 0] }; // 2^32
  let c = U256Val::mul(&a, &b);
  assert_eq!(c.limbs[0], 0);
  assert_eq!(c.limbs[1], 0);
  assert_eq!(c.limbs[2], 1); // 2^64
}

#[test]
fn test_u256_div_basic() {
  let a = U256Val::from_u32(100);
  let b = U256Val::from_u32(3);
  let c = U256Val::div(&a, &b);
  assert_eq!(c.limbs[0], 33);
}

#[test]
fn test_u256_div_by_zero() {
  let a = U256Val::from_u32(100);
  let c = U256Val::div(&a, &U256Val::ZERO);
  assert_eq!(c, U256Val::ZERO);
}

#[test]
fn test_u256_rem_basic() {
  let a = U256Val::from_u32(100);
  let b = U256Val::from_u32(3);
  let c = U256Val::rem(&a, &b);
  assert_eq!(c.limbs[0], 1);
}

#[test]
fn test_u256_comparisons() {
  let a = U256Val::from_u32(100);
  let b = U256Val::from_u32(200);
  assert!(U256Val::lt(&a, &b));
  assert!(U256Val::gt(&b, &a));
  assert!(!U256Val::eq(&a, &b));
  assert!(U256Val::eq(&a, &a));
}

#[test]
fn test_u256_comparisons_high_limbs() {
  let a = U256Val { limbs: [0, 0, 0, 0, 0, 0, 0, 1] };
  let b = U256Val { limbs: [u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX, u32::MAX, 0] };
  assert!(U256Val::gt(&a, &b));
}

#[test]
fn test_u256_bitwise() {
  let a = U256Val::from_u32(0xFF00);
  let b = U256Val::from_u32(0x0FF0);
  assert_eq!(U256Val::and(&a, &b).limbs[0], 0x0F00);
  assert_eq!(U256Val::or(&a, &b).limbs[0], 0xFFF0);
  assert_eq!(U256Val::xor(&a, &b).limbs[0], 0xF0F0);
}

#[test]
fn test_u256_shl() {
  let a = U256Val::from_u32(1);
  let b = U256Val::shl(&a, 32);
  assert_eq!(b.limbs[0], 0);
  assert_eq!(b.limbs[1], 1);

  let c = U256Val::shl(&a, 255);
  assert_eq!(c.limbs[7], 0x80000000);

  let d = U256Val::shl(&a, 256);
  assert_eq!(d, U256Val::ZERO);
}

#[test]
fn test_u256_shr() {
  let a = U256Val { limbs: [0, 1, 0, 0, 0, 0, 0, 0] }; // 2^32
  let b = U256Val::shr(&a, 32);
  assert_eq!(b.limbs[0], 1);
  assert_eq!(b.limbs[1], 0);
}

#[test]
fn test_u256_numb_roundtrip() {
  let idx: u32 = 42;
  let numb = Numb::new_u256(idx);
  assert_eq!(numb.get_typ(), TY_U256);
  assert_eq!(numb.get_u256(), idx);
}

#[test]
fn test_u256_div_rem_consistency() {
  let a = U256Val::from_u32(12345);
  let b = U256Val::from_u32(67);
  let (q, r) = U256Val::div_rem(&a, &b);
  // a == q * b + r
  let reconstructed = U256Val::add(&U256Val::mul(&q, &b), &r);
  assert_eq!(reconstructed, a);
}

// Uniswap V2 constant product formula test
#[test]
fn test_u256_univ2_math() {
  let reserve0 = U256Val::from_u32(1_000_000);
  let reserve1 = U256Val::from_u32(2_000_000);
  let amount_in = U256Val::from_u32(1000);
  let fee_factor = U256Val::from_u32(9970); // 10000 - 30
  let ten_k = U256Val::from_u32(10000);

  let amount_with_fee = U256Val::mul(&amount_in, &fee_factor);
  let numerator = U256Val::mul(&amount_with_fee, &reserve1);
  let denom_part = U256Val::mul(&reserve0, &ten_k);
  let denominator = U256Val::add(&denom_part, &amount_with_fee);
  let output = U256Val::div(&numerator, &denominator);

  assert!(output.limbs[0] > 1990 && output.limbs[0] < 2000,
    "UniV2 output was {} but expected ~1993", output.limbs[0]);
}
