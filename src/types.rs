// The ideia for the PointType trait and the implementation for decode and encode functions
// were copied from a different crate, available at:
// https://github.com/lukaskirner/tokio-Embrasul/blob/main/src/point.rs

use crate::utils::*;

pub trait PointType<T> {
    fn decode(data: Vec<u16>) -> T;
    fn encode(data: T) -> Vec<u16>;
}

#[derive(Debug, Clone, Copy)]
pub struct Point<T: PointType<T>> {
    pub name: &'static str,
    pub offset: u16,
    pub length: u16,
    pub write_access: bool,
    pub value: T,
}

impl PointType<String> for String {
    fn decode(data: Vec<u16>) -> String {
        let bytes: Vec<u8> = to_be_bytes(data);
        let fbytes: Vec<u8> = bytes.iter().filter(|b| **b != 0).copied().collect();
        String::from_utf8(fbytes).unwrap()
    }

    fn encode(data: String) -> Vec<u16> {
        to_u16_vector(data.as_bytes())
    }
}

impl PointType<Point<String>> for Point<String> {
    fn decode(data: Vec<u16>) -> Point<String> {
        let bytes: Vec<u8> = to_be_bytes(data);
        let fbytes: Vec<u8> = bytes.iter().filter(|b| **b != 0).copied().collect();
        let data: Point<String> = Point { name: "", offset: 0, length: 0, write_access: false, value: String::from_utf8(fbytes).unwrap() };
        data
    }

    fn encode(data: Point<String>) -> Vec<u16> {
        let mut regs = String::encode(data.value);
        for _i in 0..(data.length-(regs.len() as u16)){
            regs.push(0);
        }
        regs
    }
}

impl PointType<i16> for i16 {
    fn decode(data: Vec<u16>) -> i16 {
        data[0] as i16
    }

    fn encode(data: i16) -> Vec<u16> {
        vec![data as u16]
    }
}

impl PointType<i32> for i32 {
    fn decode(data: Vec<u16>) -> i32 {
        let bytes = to_be_bytes(data).try_into().unwrap();
        i32::from_be_bytes(bytes)
    }

    fn encode(data: i32) -> Vec<u16> {
        to_u16_vector(&data.to_be_bytes())
    }
}

impl PointType<i64> for i64 {
    fn decode(data: Vec<u16>) -> i64 {
        let bytes = to_be_bytes(data).try_into().unwrap();
        i64::from_be_bytes(bytes)
    }

    fn encode(data: i64) -> Vec<u16> {
        to_u16_vector(&data.to_be_bytes())
    }
}

impl PointType<u16> for u16 {
    fn decode(data: Vec<u16>) -> u16 {
        data[0]
    }

    fn encode(data: u16) -> Vec<u16> {
        vec![data]
    }
}

impl PointType<u32> for u32 {
    fn decode(data: Vec<u16>) -> u32 {
        let bytes = to_be_bytes(data).try_into().unwrap();
        u32::from_be_bytes(bytes)
    }

    fn encode(data: u32) -> Vec<u16> {
        to_u16_vector(&data.to_be_bytes())
    }
}

impl PointType<u64> for u64 {
    fn decode(data: Vec<u16>) -> u64 {
        let bytes = to_be_bytes(data).try_into().unwrap();
        u64::from_be_bytes(bytes)
    }

    fn encode(data: u64) -> Vec<u16> {
        to_u16_vector(&data.to_be_bytes())
    }
}

impl PointType<u128> for u128 {
    fn decode(data: Vec<u16>) -> u128 {
        let bytes = to_be_bytes(data).try_into().unwrap();
        u128::from_be_bytes(bytes)
    }

    fn encode(data: u128) -> Vec<u16> {
        to_u16_vector(&data.to_be_bytes())
    }
}

#[allow(dead_code)]
#[cfg(feature = "md")]
impl PointType<f32> for f32 {
    fn decode(data: Vec<u16>) -> f32 {
        let mut bytes: [u8; 4] = to_be_bytes(data).try_into().unwrap();
        let tmp: [u8; 2] = [bytes[2], bytes[3]];
        bytes[3] = bytes[1];
        bytes[2] = bytes[0];
        bytes[1] = tmp[1];
        bytes[0] = tmp[0];
        f32::from_be_bytes(bytes)
    }

    fn encode(data: f32) -> Vec<u16> {
        let mut bytes = data.to_be_bytes();
        let tmp: [u8; 2] = [bytes[2], bytes[3]];
        bytes[3] = bytes[1];
        bytes[2] = bytes[0];
        bytes[1] = tmp[1];
        bytes[0] = tmp[0];
        to_u16_vector(&bytes)
    }
}

#[allow(dead_code)]
#[cfg(feature = "gde")]
impl PointType<f32> for f32 {
    fn decode(data: Vec<u16>) -> f32 {
        let bytes: [u8; 4] = to_be_bytes(data).try_into().unwrap();
        f32::from_be_bytes(bytes)
    }

    fn encode(data: f32) -> Vec<u16> {
        to_u16_vector(&data.to_be_bytes())
    }
}

#[derive(Debug, Clone)]
pub enum EDataTypes {
    EmbrasulString(Point<String>),
    EmbrasulU16(Point<u16>),
    EmbrasulU32(Point<u32>),
    EmbrasulU64(Point<u64>),
    EmbrasulU128(Point<u128>),
    EmbrasulI16(Point<i16>),
    EmbrasulI32(Point<i32>),
    EmbrasulI64(Point<i64>),
    EmbrasulF32(Point<f32>),
}

pub trait EmbrasulTypes {
    // This new function acts as a constructor
    fn new_string (data: &str) -> Self;
    fn new_u16 (data: u16) -> Self;
    fn new_u32 (data: u32) -> Self;
    fn new_u64 (data: u64) -> Self;
    fn new_u128 (data: u128) -> Self;
    fn new_i16 (data: i16) -> Self;
    fn new_i32 (data: i32) -> Self;
    fn new_i64 (data: i64) -> Self;
    fn new_f32 (data: f32) -> Self;
}

impl EmbrasulTypes for EDataTypes {
    fn new_string (data: &str) -> EDataTypes {
        EDataTypes::EmbrasulString(Point { name: "", offset: 0, length: 0, write_access: false, value: String::from(data) } )
    }
    fn new_u16 (data: u16) -> EDataTypes {
        EDataTypes::EmbrasulU16(Point { name: "", offset: 0, length: 0, write_access: false, value: data } )
    }
    fn new_u32 (data: u32) -> EDataTypes {
        EDataTypes::EmbrasulU32(Point { name: "", offset: 0, length: 0, write_access: false, value: data } )
    }
    fn new_u64 (data: u64) -> EDataTypes {
        EDataTypes::EmbrasulU64(Point { name: "", offset: 0, length: 0, write_access: false, value: data } )
    }
    fn new_u128 (data: u128) -> EDataTypes {
        EDataTypes::EmbrasulU128(Point { name: "", offset: 0, length: 0, write_access: false, value: data } )
    }
    fn new_i16 (data: i16) -> EDataTypes {
        EDataTypes::EmbrasulI16(Point { name: "", offset: 0, length: 0, write_access: false, value: data } )
    }
    fn new_i32 (data: i32) -> EDataTypes {
        EDataTypes::EmbrasulI32(Point { name: "", offset: 0, length: 0, write_access: false, value: data } )
    }
    fn new_i64 (data: i64) -> EDataTypes {
        EDataTypes::EmbrasulI64(Point { name: "", offset: 0, length: 0, write_access: false, value: data } )
    }
    fn new_f32 (data: f32) -> EDataTypes {
        EDataTypes::EmbrasulF32(Point { name: "", offset: 0, length: 0, write_access: false, value: data } )
    }
}
