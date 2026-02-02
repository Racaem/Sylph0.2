// 类型定义和操作

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Rem};
use num_bigint::BigInt;
use num_traits::cast::ToPrimitive;

// 整数类型枚举
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum IntegerType {
    I8,
    I16,
    I32,
    I64,
    I128,
    BigInt,
}

// 整数值枚举
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum IntegerValue {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    BigInt(BigInt), // 使用BigInt存储任意精度整数
}

// 字符串值
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct StringValue {
    value: String,
}

impl StringValue {
    pub fn new(value: String) -> Self {
        StringValue { value }
    }
    
    pub fn as_str(&self) -> &str {
        &self.value
    }
    
    pub fn len(&self) -> usize {
        self.value.len()
    }
}

impl fmt::Display for StringValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

// 统一值类型
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Value {
    Integer(IntegerValue),
    String(StringValue),
}

// 为 Value 实现 PartialOrd
impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => a.partial_cmp(b),
            (Value::String(a), Value::String(b)) => a.value.partial_cmp(&b.value),
            _ => None, // 不同类型之间不比较
        }
    }
}

// 为 Value 实现 Ord
impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => a.cmp(b),
            (Value::String(a), Value::String(b)) => a.value.cmp(&b.value),
            _ => panic!("Cannot compare different types"),
        }
    }
}

// 为 Value 实现 fmt::Display
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Integer(v) => write!(f, "{}", v),
            Value::String(v) => write!(f, "{}", v),
        }
    }
}

// 实现 PartialOrd 用于比较
impl PartialOrd for IntegerValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// 实现 Ord 用于排序
impl Ord for IntegerValue {
    fn cmp(&self, other: &Self) -> Ordering {
        // 将两个值转换为 i128 进行比较，如果是 BigInt 则特殊处理
        match (self, other) {
            (IntegerValue::I8(a), IntegerValue::I8(b)) => a.cmp(b),
            (IntegerValue::I16(a), IntegerValue::I16(b)) => a.cmp(b),
            (IntegerValue::I32(a), IntegerValue::I32(b)) => a.cmp(b),
            (IntegerValue::I64(a), IntegerValue::I64(b)) => a.cmp(b),
            (IntegerValue::I128(a), IntegerValue::I128(b)) => a.cmp(b),
            (IntegerValue::BigInt(a), IntegerValue::BigInt(b)) => a.cmp(b),
            _ => {
                // 混合类型比较，转换为较大的类型
                let a = self.to_i128().unwrap_or_else(|_| panic!("Cannot compare mixed integer types"));
                let b = other.to_i128().unwrap_or_else(|_| panic!("Cannot compare mixed integer types"));
                a.cmp(&b)
            }
        }
    }
}

// 实现 fmt::Display 用于打印
impl fmt::Display for IntegerValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IntegerValue::I8(v) => write!(f, "{}", v),
            IntegerValue::I16(v) => write!(f, "{}", v),
            IntegerValue::I32(v) => write!(f, "{}", v),
            IntegerValue::I64(v) => write!(f, "{}", v),
            IntegerValue::I128(v) => write!(f, "{}", v),
            IntegerValue::BigInt(v) => write!(f, "{}", v),
        }
    }
}

// 整数类型的常量和操作
trait IntegerTypeInfo {
    const MIN: &'static str;
    const MAX: &'static str;
    const TYPE: IntegerType;
}

// 实现各种整数类型的常量
impl IntegerTypeInfo for i8 {
    const MIN: &'static str = "-128";
    const MAX: &'static str = "127";
    const TYPE: IntegerType = IntegerType::I8;
}

impl IntegerTypeInfo for i16 {
    const MIN: &'static str = "-32768";
    const MAX: &'static str = "32767";
    const TYPE: IntegerType = IntegerType::I16;
}

impl IntegerTypeInfo for i32 {
    const MIN: &'static str = "-2147483648";
    const MAX: &'static str = "2147483647";
    const TYPE: IntegerType = IntegerType::I32;
}

impl IntegerTypeInfo for i64 {
    const MIN: &'static str = "-9223372036854775808";
    const MAX: &'static str = "9223372036854775807";
    const TYPE: IntegerType = IntegerType::I64;
}

impl IntegerTypeInfo for i128 {
    const MIN: &'static str = "-170141183460469231731687303715884105728";
    const MAX: &'static str = "170141183460469231731687303715884105727";
    const TYPE: IntegerType = IntegerType::I128;
}

// 实现 IntegerValue 的方法
impl IntegerValue {
    // 从字符串创建 IntegerValue
    pub fn from_string(s: &str, int_type: IntegerType) -> Result<Self, String> {
        match int_type {
            IntegerType::I8 => {
                s.parse::<i8>()
                    .map(IntegerValue::I8)
                    .map_err(|_| format!("Value {} out of range for i8", s))
            }
            IntegerType::I16 => {
                s.parse::<i16>()
                    .map(IntegerValue::I16)
                    .map_err(|_| format!("Value {} out of range for i16", s))
            }
            IntegerType::I32 => {
                s.parse::<i32>()
                    .map(IntegerValue::I32)
                    .map_err(|_| format!("Value {} out of range for i32", s))
            }
            IntegerType::I64 => {
                s.parse::<i64>()
                    .map(IntegerValue::I64)
                    .map_err(|_| format!("Value {} out of range for i64", s))
            }
            IntegerType::I128 => {
                s.parse::<i128>()
                    .map(IntegerValue::I128)
                    .map_err(|_| format!("Value {} out of range for i128", s))
            }
            IntegerType::BigInt => {
                // 对于 BigInt，使用 BigInt::parse_bytes 来解析
                match BigInt::parse_bytes(s.as_bytes(), 10) {
                    Some(value) => Ok(IntegerValue::BigInt(value)),
                    None => Err(format!("Invalid bigint value: {}", s)),
                }
            }
        }
    }

    // 获取类型
    pub fn get_type(&self) -> IntegerType {
        match self {
            IntegerValue::I8(_) => IntegerType::I8,
            IntegerValue::I16(_) => IntegerType::I16,
            IntegerValue::I32(_) => IntegerType::I32,
            IntegerValue::I64(_) => IntegerType::I64,
            IntegerValue::I128(_) => IntegerType::I128,
            IntegerValue::BigInt(_) => IntegerType::BigInt,
        }
    }

    // 转换为 i8
    pub fn to_i8(&self) -> Result<i8, String> {
        match self {
            IntegerValue::I8(v) => Ok(*v),
            IntegerValue::I16(v) => {
                if *v >= i8::MIN as i16 && *v <= i8::MAX as i16 {
                    Ok(*v as i8)
                } else {
                    Err(format!("Value {} out of range for i8", v))
                }
            }
            IntegerValue::I32(v) => {
                if *v >= i8::MIN as i32 && *v <= i8::MAX as i32 {
                    Ok(*v as i8)
                } else {
                    Err(format!("Value {} out of range for i8", v))
                }
            }
            IntegerValue::I64(v) => {
                if *v >= i8::MIN as i64 && *v <= i8::MAX as i64 {
                    Ok(*v as i8)
                } else {
                    Err(format!("Value {} out of range for i8", v))
                }
            }
            IntegerValue::I128(v) => {
                if *v >= i8::MIN as i128 && *v <= i8::MAX as i128 {
                    Ok(*v as i8)
                } else {
                    Err(format!("Value {} out of range for i8", v))
                }
            }
            IntegerValue::BigInt(v) => {
                if let Some(value) = v.to_i8() {
                    Ok(value)
                } else {
                    Err(format!("Value {} out of range for i8", v))
                }
            }
        }
    }

    // 转换为 i16
    pub fn to_i16(&self) -> Result<i16, String> {
        match self {
            IntegerValue::I8(v) => Ok(*v as i16),
            IntegerValue::I16(v) => Ok(*v),
            IntegerValue::I32(v) => {
                if *v >= i16::MIN as i32 && *v <= i16::MAX as i32 {
                    Ok(*v as i16)
                } else {
                    Err(format!("Value {} out of range for i16", v))
                }
            }
            IntegerValue::I64(v) => {
                if *v >= i16::MIN as i64 && *v <= i16::MAX as i64 {
                    Ok(*v as i16)
                } else {
                    Err(format!("Value {} out of range for i16", v))
                }
            }
            IntegerValue::I128(v) => {
                if *v >= i16::MIN as i128 && *v <= i16::MAX as i128 {
                    Ok(*v as i16)
                } else {
                    Err(format!("Value {} out of range for i16", v))
                }
            }
            IntegerValue::BigInt(v) => {
                if let Some(value) = v.to_i16() {
                    Ok(value)
                } else {
                    Err(format!("Value {} out of range for i16", v))
                }
            }
        }
    }

    // 转换为 i32
    pub fn to_i32(&self) -> Result<i32, String> {
        match self {
            IntegerValue::I8(v) => Ok(*v as i32),
            IntegerValue::I16(v) => Ok(*v as i32),
            IntegerValue::I32(v) => Ok(*v),
            IntegerValue::I64(v) => {
                if *v >= i32::MIN as i64 && *v <= i32::MAX as i64 {
                    Ok(*v as i32)
                } else {
                    Err(format!("Value {} out of range for i32", v))
                }
            }
            IntegerValue::I128(v) => {
                if *v >= i32::MIN as i128 && *v <= i32::MAX as i128 {
                    Ok(*v as i32)
                } else {
                    Err(format!("Value {} out of range for i32", v))
                }
            }
            IntegerValue::BigInt(v) => {
                if let Some(value) = v.to_i32() {
                    Ok(value)
                } else {
                    Err(format!("Value {} out of range for i32", v))
                }
            }
        }
    }

    // 转换为 i64
    pub fn to_i64(&self) -> Result<i64, String> {
        match self {
            IntegerValue::I8(v) => Ok(*v as i64),
            IntegerValue::I16(v) => Ok(*v as i64),
            IntegerValue::I32(v) => Ok(*v as i64),
            IntegerValue::I64(v) => Ok(*v),
            IntegerValue::I128(v) => {
                if *v >= i64::MIN as i128 && *v <= i64::MAX as i128 {
                    Ok(*v as i64)
                } else {
                    Err(format!("Value {} out of range for i64", v))
                }
            }
            IntegerValue::BigInt(v) => {
                if let Some(value) = v.to_i64() {
                    Ok(value)
                } else {
                    Err(format!("Value {} out of range for i64", v))
                }
            }
        }
    }

    // 转换为 i128
    pub fn to_i128(&self) -> Result<i128, String> {
        match self {
            IntegerValue::I8(v) => Ok(*v as i128),
            IntegerValue::I16(v) => Ok(*v as i128),
            IntegerValue::I32(v) => Ok(*v as i128),
            IntegerValue::I64(v) => Ok(*v as i128),
            IntegerValue::I128(v) => Ok(*v),
            IntegerValue::BigInt(v) => {
                if let Some(value) = v.to_i128() {
                    Ok(value)
                } else {
                    Err(format!("Value {} out of range for i128", v))
                }
            }
        }
    }

    // 转换为 BigInt
    pub fn to_bigint(&self) -> IntegerValue {
        match self {
            IntegerValue::I8(v) => IntegerValue::BigInt(BigInt::from(*v)),
            IntegerValue::I16(v) => IntegerValue::BigInt(BigInt::from(*v)),
            IntegerValue::I32(v) => IntegerValue::BigInt(BigInt::from(*v)),
            IntegerValue::I64(v) => IntegerValue::BigInt(BigInt::from(*v)),
            IntegerValue::I128(v) => IntegerValue::BigInt(BigInt::from(*v)),
            IntegerValue::BigInt(v) => IntegerValue::BigInt(v.clone()),
        }
    }

    // 自动类型提升：返回两个值中较大的类型
    pub fn promote_type(a: &IntegerValue, b: &IntegerValue) -> IntegerType {
        let type_order = [
            IntegerType::I8,
            IntegerType::I16,
            IntegerType::I32,
            IntegerType::I64,
            IntegerType::I128,
            IntegerType::BigInt,
        ];

        let a_type = a.get_type();
        let b_type = b.get_type();

        let a_idx = type_order.iter().position(|t| *t == a_type).unwrap();
        let b_idx = type_order.iter().position(|t| *t == b_type).unwrap();

        if a_idx > b_idx {
            a_type
        } else {
            b_type
        }
    }

    // 转换为指定类型
    pub fn cast_to(&self, target_type: &IntegerType) -> Result<IntegerValue, String> {
        match target_type {
            IntegerType::I8 => self.to_i8().map(IntegerValue::I8),
            IntegerType::I16 => self.to_i16().map(IntegerValue::I16),
            IntegerType::I32 => self.to_i32().map(IntegerValue::I32),
            IntegerType::I64 => self.to_i64().map(IntegerValue::I64),
            IntegerType::I128 => self.to_i128().map(IntegerValue::I128),
            IntegerType::BigInt => Ok(self.to_bigint()),
        }
    }
}

// 实现加法操作
impl Add for IntegerValue {
    type Output = Result<IntegerValue, String>;

    fn add(self, rhs: Self) -> Self::Output {
        let target_type = IntegerValue::promote_type(&self, &rhs);
        let a = self.cast_to(&target_type)?;
        let b = rhs.cast_to(&target_type)?;

        match (a, b) {
            (IntegerValue::I8(a), IntegerValue::I8(b)) => {
                a.checked_add(b)
                    .map(IntegerValue::I8)
                    .ok_or_else(|| format!("Addition overflow for i8: {} + {}", a, b))
            }
            (IntegerValue::I16(a), IntegerValue::I16(b)) => {
                a.checked_add(b)
                    .map(IntegerValue::I16)
                    .ok_or_else(|| format!("Addition overflow for i16: {} + {}", a, b))
            }
            (IntegerValue::I32(a), IntegerValue::I32(b)) => {
                a.checked_add(b)
                    .map(IntegerValue::I32)
                    .ok_or_else(|| format!("Addition overflow for i32: {} + {}", a, b))
            }
            (IntegerValue::I64(a), IntegerValue::I64(b)) => {
                a.checked_add(b)
                    .map(IntegerValue::I64)
                    .ok_or_else(|| format!("Addition overflow for i64: {} + {}", a, b))
            }
            (IntegerValue::I128(a), IntegerValue::I128(b)) => {
                a.checked_add(b)
                    .map(IntegerValue::I128)
                    .ok_or_else(|| format!("Addition overflow for i128: {} + {}", a, b))
            }
            (IntegerValue::BigInt(a), IntegerValue::BigInt(b)) => {
                let result = a + b;
                Ok(IntegerValue::BigInt(result))
            }
            _ => Err("Type mismatch in addition".to_string()),
        }
    }
}

// 实现减法操作
impl Sub for IntegerValue {
    type Output = Result<IntegerValue, String>;

    fn sub(self, rhs: Self) -> Self::Output {
        let target_type = IntegerValue::promote_type(&self, &rhs);
        let a = self.cast_to(&target_type)?;
        let b = rhs.cast_to(&target_type)?;

        match (a, b) {
            (IntegerValue::I8(a), IntegerValue::I8(b)) => {
                a.checked_sub(b)
                    .map(IntegerValue::I8)
                    .ok_or_else(|| format!("Subtraction overflow for i8: {} - {}", a, b))
            }
            (IntegerValue::I16(a), IntegerValue::I16(b)) => {
                a.checked_sub(b)
                    .map(IntegerValue::I16)
                    .ok_or_else(|| format!("Subtraction overflow for i16: {} - {}", a, b))
            }
            (IntegerValue::I32(a), IntegerValue::I32(b)) => {
                a.checked_sub(b)
                    .map(IntegerValue::I32)
                    .ok_or_else(|| format!("Subtraction overflow for i32: {} - {}", a, b))
            }
            (IntegerValue::I64(a), IntegerValue::I64(b)) => {
                a.checked_sub(b)
                    .map(IntegerValue::I64)
                    .ok_or_else(|| format!("Subtraction overflow for i64: {} - {}", a, b))
            }
            (IntegerValue::I128(a), IntegerValue::I128(b)) => {
                a.checked_sub(b)
                    .map(IntegerValue::I128)
                    .ok_or_else(|| format!("Subtraction overflow for i128: {} - {}", a, b))
            }
            (IntegerValue::BigInt(a), IntegerValue::BigInt(b)) => {
                let result = a - b;
                Ok(IntegerValue::BigInt(result))
            }
            _ => Err("Type mismatch in subtraction".to_string()),
        }
    }
}

// 实现乘法操作
impl Mul for IntegerValue {
    type Output = Result<IntegerValue, String>;

    fn mul(self, rhs: Self) -> Self::Output {
        let target_type = IntegerValue::promote_type(&self, &rhs);
        let a = self.cast_to(&target_type)?;
        let b = rhs.cast_to(&target_type)?;

        match (a, b) {
            (IntegerValue::I8(a), IntegerValue::I8(b)) => {
                if let Some(result) = a.checked_mul(b) {
                    Ok(IntegerValue::I8(result))
                } else {
                    // 溢出，提升到 BigInt
                    let big_a = BigInt::from(a);
                    let big_b = BigInt::from(b);
                    let result = big_a * big_b;
                    Ok(IntegerValue::BigInt(result))
                }
            }
            (IntegerValue::I16(a), IntegerValue::I16(b)) => {
                if let Some(result) = a.checked_mul(b) {
                    Ok(IntegerValue::I16(result))
                } else {
                    // 溢出，提升到 BigInt
                    let big_a = BigInt::from(a);
                    let big_b = BigInt::from(b);
                    let result = big_a * big_b;
                    Ok(IntegerValue::BigInt(result))
                }
            }
            (IntegerValue::I32(a), IntegerValue::I32(b)) => {
                if let Some(result) = a.checked_mul(b) {
                    Ok(IntegerValue::I32(result))
                } else {
                    // 溢出，提升到 BigInt
                    let big_a = BigInt::from(a);
                    let big_b = BigInt::from(b);
                    let result = big_a * big_b;
                    Ok(IntegerValue::BigInt(result))
                }
            }
            (IntegerValue::I64(a), IntegerValue::I64(b)) => {
                if let Some(result) = a.checked_mul(b) {
                    Ok(IntegerValue::I64(result))
                } else {
                    // 溢出，提升到 BigInt
                    let big_a = BigInt::from(a);
                    let big_b = BigInt::from(b);
                    let result = big_a * big_b;
                    Ok(IntegerValue::BigInt(result))
                }
            }
            (IntegerValue::I128(a), IntegerValue::I128(b)) => {
                if let Some(result) = a.checked_mul(b) {
                    Ok(IntegerValue::I128(result))
                } else {
                    // 溢出，提升到 BigInt
                    let big_a = BigInt::from(a);
                    let big_b = BigInt::from(b);
                    let result = big_a * big_b;
                    Ok(IntegerValue::BigInt(result))
                }
            }
            (IntegerValue::BigInt(a), IntegerValue::BigInt(b)) => {
                let result = a * b;
                Ok(IntegerValue::BigInt(result))
            }
            _ => Err("Type mismatch in multiplication".to_string()),
        }
    }
}

// 实现除法操作
impl Div for IntegerValue {
    type Output = Result<IntegerValue, String>;

    fn div(self, rhs: Self) -> Self::Output {
        let target_type = IntegerValue::promote_type(&self, &rhs);
        let a = self.cast_to(&target_type)?;
        let b = rhs.cast_to(&target_type)?;

        match (a, b) {
            (IntegerValue::I8(a), IntegerValue::I8(b)) => {
                if b == 0 {
                    return Err("Division by zero".to_string());
                }
                a.checked_div(b)
                    .map(IntegerValue::I8)
                    .ok_or_else(|| format!("Division overflow for i8: {} / {}", a, b))
            }
            (IntegerValue::I16(a), IntegerValue::I16(b)) => {
                if b == 0 {
                    return Err("Division by zero".to_string());
                }
                a.checked_div(b)
                    .map(IntegerValue::I16)
                    .ok_or_else(|| format!("Division overflow for i16: {} / {}", a, b))
            }
            (IntegerValue::I32(a), IntegerValue::I32(b)) => {
                if b == 0 {
                    return Err("Division by zero".to_string());
                }
                a.checked_div(b)
                    .map(IntegerValue::I32)
                    .ok_or_else(|| format!("Division overflow for i32: {} / {}", a, b))
            }
            (IntegerValue::I64(a), IntegerValue::I64(b)) => {
                if b == 0 {
                    return Err("Division by zero".to_string());
                }
                a.checked_div(b)
                    .map(IntegerValue::I64)
                    .ok_or_else(|| format!("Division overflow for i64: {} / {}", a, b))
            }
            (IntegerValue::I128(a), IntegerValue::I128(b)) => {
                if b == 0 {
                    return Err("Division by zero".to_string());
                }
                a.checked_div(b)
                    .map(IntegerValue::I128)
                    .ok_or_else(|| format!("Division overflow for i128: {} / {}", a, b))
            }
            (IntegerValue::BigInt(a), IntegerValue::BigInt(b)) => {
                if b == BigInt::from(0) {
                    return Err("Division by zero".to_string());
                }
                let result = a / b;
                Ok(IntegerValue::BigInt(result))
            }
            _ => Err("Type mismatch in division".to_string()),
        }
    }
}

// 实现取模操作
impl Rem for IntegerValue {
    type Output = Result<IntegerValue, String>;

    fn rem(self, rhs: Self) -> Self::Output {
        let target_type = IntegerValue::promote_type(&self, &rhs);
        let a = self.cast_to(&target_type)?;
        let b = rhs.cast_to(&target_type)?;

        match (a, b) {
            (IntegerValue::I8(a), IntegerValue::I8(b)) => {
                if b == 0 {
                    return Err("Modulo by zero".to_string());
                }
                Ok(IntegerValue::I8(a % b))
            }
            (IntegerValue::I16(a), IntegerValue::I16(b)) => {
                if b == 0 {
                    return Err("Modulo by zero".to_string());
                }
                Ok(IntegerValue::I16(a % b))
            }
            (IntegerValue::I32(a), IntegerValue::I32(b)) => {
                if b == 0 {
                    return Err("Modulo by zero".to_string());
                }
                Ok(IntegerValue::I32(a % b))
            }
            (IntegerValue::I64(a), IntegerValue::I64(b)) => {
                if b == 0 {
                    return Err("Modulo by zero".to_string());
                }
                Ok(IntegerValue::I64(a % b))
            }
            (IntegerValue::I128(a), IntegerValue::I128(b)) => {
                if b == 0 {
                    return Err("Modulo by zero".to_string());
                }
                Ok(IntegerValue::I128(a % b))
            }
            (IntegerValue::BigInt(a), IntegerValue::BigInt(b)) => {
                if b == BigInt::from(0) {
                    return Err("Modulo by zero".to_string());
                }
                let result = a % b;
                Ok(IntegerValue::BigInt(result))
            }
            _ => Err("Type mismatch in modulo operation".to_string()),
        }
    }
}

// 测试函数
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_value_creation() {
        // 测试创建各种整数类型
        assert!(IntegerValue::from_string("10", IntegerType::I8).is_ok());
        assert!(IntegerValue::from_string("1000", IntegerType::I16).is_ok());
        assert!(IntegerValue::from_string("100000", IntegerType::I32).is_ok());
        assert!(IntegerValue::from_string("1000000000", IntegerType::I64).is_ok());
        assert!(IntegerValue::from_string("1000000000000000000", IntegerType::I128).is_ok());
        assert!(IntegerValue::from_string("1000000000000000000000000000000", IntegerType::BigInt).is_ok());

        // 测试边界值
        assert!(IntegerValue::from_string(i8::MIN.to_string().as_str(), IntegerType::I8).is_ok());
        assert!(IntegerValue::from_string(i8::MAX.to_string().as_str(), IntegerType::I8).is_ok());
        assert!(IntegerValue::from_string("128", IntegerType::I8).is_err());
        assert!(IntegerValue::from_string("-129", IntegerType::I8).is_err());
    }

    #[test]
    fn test_integer_value_conversion() {
        // 测试类型转换
        let i8_val = IntegerValue::from_string("10", IntegerType::I8).unwrap();
        assert_eq!(i8_val.to_i16().unwrap(), 10);
        assert_eq!(i8_val.to_i32().unwrap(), 10);
        assert_eq!(i8_val.to_i64().unwrap(), 10);
        assert_eq!(i8_val.to_i128().unwrap(), 10);

        let i16_val = IntegerValue::from_string("1000", IntegerType::I16).unwrap();
        assert_eq!(i16_val.to_i32().unwrap(), 1000);
        assert_eq!(i16_val.to_i64().unwrap(), 1000);

        // 测试溢出转换
        let i16_max = IntegerValue::from_string(i16::MAX.to_string().as_str(), IntegerType::I16).unwrap();
        assert!(i16_max.to_i8().is_err());
    }

    #[test]
    fn test_integer_operations() {
        // 测试加法
        let a = IntegerValue::from_string("10", IntegerType::I8).unwrap();
        let b = IntegerValue::from_string("20", IntegerType::I8).unwrap();
        let result = a + b;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_i8().unwrap(), 30);

        // 测试减法
        let a = IntegerValue::from_string("30", IntegerType::I8).unwrap();
        let b = IntegerValue::from_string("10", IntegerType::I8).unwrap();
        let result = a - b;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_i8().unwrap(), 20);

        // 测试乘法
        let a = IntegerValue::from_string("5", IntegerType::I8).unwrap();
        let b = IntegerValue::from_string("6", IntegerType::I8).unwrap();
        let result = a * b;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_i8().unwrap(), 30);

        // 测试除法
        let a = IntegerValue::from_string("30", IntegerType::I8).unwrap();
        let b = IntegerValue::from_string("5", IntegerType::I8).unwrap();
        let result = a / b;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_i8().unwrap(), 6);

        // 测试取模
        let a = IntegerValue::from_string("31", IntegerType::I8).unwrap();
        let b = IntegerValue::from_string("5", IntegerType::I8).unwrap();
        let result = a % b;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_i8().unwrap(), 1);

        // 测试溢出
        let a = IntegerValue::from_string(i8::MAX.to_string().as_str(), IntegerType::I8).unwrap();
        let b = IntegerValue::from_string("1", IntegerType::I8).unwrap();
        let result = a + b;
        assert!(result.is_err());
    }

    #[test]
    fn test_bigint_operations() {
        // 测试 BigInt 操作
        let a = IntegerValue::from_string("1000000000000000000", IntegerType::BigInt).unwrap();
        let b = IntegerValue::from_string("2000000000000000000", IntegerType::BigInt).unwrap();
        let result = a + b;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_bigint().to_string(), "3000000000000000000");
    }

    #[test]
    fn test_mixed_type_operations() {
        // 测试混合类型操作
        let a = IntegerValue::from_string("10", IntegerType::I8).unwrap();
        let b = IntegerValue::from_string("20", IntegerType::I16).unwrap();
        let result = a + b;
        assert!(result.is_ok());
        let result_unwrap = result.unwrap();
        assert_eq!(result_unwrap.get_type(), IntegerType::I16);
        assert_eq!(result_unwrap.to_i16().unwrap(), 30);
    }
}
