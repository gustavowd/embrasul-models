use std::io::Write;
use crate::types::*;

pub mod model1;
pub mod model2;
pub mod model3;
pub mod model4;
pub mod model5;


#[derive(Debug, Clone)]
pub struct EModel {
    pub start_addr: u16,
    pub end_addr: u16,
    pub model_number: u16,
    pub qtd: u16,
    pub update: bool,
    pub data: Vec<EDataTypes>,
}

#[derive(Debug, Clone)]
pub struct EModels {
    pub models: Vec<EModel>,
}

// Declare the struct
pub trait EmbrasulModels {
    // This new function acts as a constructor
    fn new (model_number: u16) -> Self;
    fn update_data(&mut self, point: &str, value: &EDataTypes);
    fn update_data_by_index(&mut self, index: usize, value: &EDataTypes);
    fn get_data(&self, point: &str) -> EDataTypes;
    fn get_data_index(&self, point: &str) -> usize;
    fn get_string(&self, point: &str) -> Option<String>;
    fn get_string_by_index(&self, idx: usize) -> Option<String>;
    fn get_f32(&self, point: &str) -> Option<f32>;
    fn get_f32_by_index(&self, idx: usize) -> Option<f32>;
    fn get_u16(&self, point: &str) -> Option<u16>;
    fn get_u16_by_index(&self, idx: usize) -> Option<u16>;
    fn get_u32(&self, point: &str) -> Option<u32>;
    fn get_u32_by_index(&self, idx: usize) -> Option<u32>;
    fn get_u64(&self, point: &str) -> Option<u64>;
    fn get_u64_by_index(&self, idx: usize) -> Option<u64>;
    fn get_u128(&self, point: &str) -> Option<u128>;
    fn get_u128_by_index(&self, idx: usize) -> Option<u128>;
    fn get_i16(&self, point: &str) -> Option<i16>;
    fn get_i16_by_index(&self, idx: usize) -> Option<i16>;
    fn get_i32(&self, point: &str) -> Option<i32>;
    fn get_i32_by_index(&self, idx: usize) -> Option<i32>;
    fn get_i64(&self, point: &str) -> Option<i64>;
    fn get_i64_by_index(&self, idx: usize) -> Option<i64>;
    fn print(&self);
}


impl EModels {
    pub fn new () -> EModels {
        EModels { models: Vec::new() }
    }

    pub fn get_model_index(&self, model_number: u16) -> Option<usize> {
        let mut idx = 0;
        for model in self.models.iter() {
            if model_number == model.model_number {
                return Some(idx);
            }
            idx += 1;
        }
        None
    }

    pub fn compute_addr (&mut self) {
        let mut idx = 0;
        let mut end_addr = 0;
        for model in self.models.iter_mut() {
            if idx == 0 {
                model.start_addr = 2;
            }else{
                model.start_addr = end_addr;
            }
            idx += 1;
            model.end_addr = model.start_addr + model.qtd + 2;
            end_addr = model.end_addr;
        }
    }
}

fn model_end() -> EModel {
    EModel {
        start_addr: 0,
        model_number: 0xFFFF,
        qtd: 0,
        end_addr: 0,
        update: false,
        data: Vec::new(),
    }
}

impl EmbrasulModels for EModel {
    fn new (model_number: u16) -> EModel {
        match model_number {
            1 => model1::model1(),
            2 => model2::model2(),
            3 => model3::model3(),
            4 => model4::model4(),
            5 => model5::model5(),
            _ => model_end(),
        }
    }

    fn update_data(&mut self, point: &str, value: &EDataTypes) {
        for data_tmp in self.data.iter_mut() {
            match data_tmp {
                EDataTypes::EmbrasulString(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()){
                        match value {
                            EDataTypes::EmbrasulString(update_value) =>  data.value = update_value.value.clone(),
                            _ => {},
                        };
                    }
                },
                EDataTypes::EmbrasulF32(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()){
                        match value {
                            EDataTypes::EmbrasulF32(update_value) =>  data.value = update_value.value,
                            _ => {},
                        };
                    }
                },
                EDataTypes::EmbrasulU16(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()){
                        match value {
                            EDataTypes::EmbrasulU16(update_value) =>  data.value = update_value.value,
                            _ => {},
                        };
                    }
                },
                EDataTypes::EmbrasulU32(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()){
                        match value {
                            EDataTypes::EmbrasulU32(update_value) =>  data.value = update_value.value,
                            _ => {},
                        };
                    }
                },
                EDataTypes::EmbrasulU64(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()){
                        match value {
                            EDataTypes::EmbrasulU64(update_value) =>  data.value = update_value.value,
                            _ => {},
                        };
                    }
                },
                EDataTypes::EmbrasulU128(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()){
                        match value {
                            EDataTypes::EmbrasulU128(update_value) =>  data.value = update_value.value,
                            _ => {},
                        };
                    }
                },
                EDataTypes::EmbrasulI16(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()){
                        match value {
                            EDataTypes::EmbrasulI16(update_value) =>  data.value = update_value.value,
                            _ => {},
                        };
                    }
                },
                EDataTypes::EmbrasulI32(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()){
                        match value {
                            EDataTypes::EmbrasulI32(update_value) =>  data.value = update_value.value,
                            _ => {},
                        };
                    }
                },
                EDataTypes::EmbrasulI64(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()){
                        match value {
                            EDataTypes::EmbrasulI64(update_value) =>  data.value = update_value.value,
                            _ => {},
                        };
                    }
                },
            }
        }
    }

    fn update_data_by_index(&mut self, index: usize, value: &EDataTypes) {
        match &mut self.data[index] {
            EDataTypes::EmbrasulString(data) => {
                match value {
                    EDataTypes::EmbrasulString(update_value) =>  data.value = update_value.value.clone(),
                    _ => {},
                };
            },
            EDataTypes::EmbrasulF32(data) => {
                match value {
                    EDataTypes::EmbrasulF32(update_value) =>  data.value = update_value.value,
                    _ => {},
                };
            },
            EDataTypes::EmbrasulU16(data) => {
                match value {
                    EDataTypes::EmbrasulU16(update_value) =>  data.value = update_value.value,
                    _ => {},
                };
            },
            EDataTypes::EmbrasulU32(data) => {
                match value {
                    EDataTypes::EmbrasulU32(update_value) =>  data.value = update_value.value,
                    _ => {},
                };
            },
            EDataTypes::EmbrasulU64(data) => {
                match value {
                    EDataTypes::EmbrasulU64(update_value) =>  data.value = update_value.value,
                    _ => {},
                };
            },
            EDataTypes::EmbrasulU128(data) => {
                match value {
                    EDataTypes::EmbrasulU128(update_value) =>  data.value = update_value.value,
                    _ => {},
                };
            },
            EDataTypes::EmbrasulI16(data) => {
                match value {
                    EDataTypes::EmbrasulI16(update_value) =>  data.value = update_value.value,
                    _ => {},
                };
            },
            EDataTypes::EmbrasulI32(data) => {
                match value {
                    EDataTypes::EmbrasulI32(update_value) =>  data.value = update_value.value,
                    _ => {},
                };
            },
            EDataTypes::EmbrasulI64(data) => {
                match value {
                    EDataTypes::EmbrasulI64(update_value) =>  data.value = update_value.value,
                    _ => {},
                };
            },
        }
    }

    fn get_data(&self, point: &str) -> EDataTypes {
        for data_tmp in self.data.iter() {
            match data_tmp {
                EDataTypes::EmbrasulString(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return data_tmp.clone();
                    }
                },
                EDataTypes::EmbrasulU16(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return data_tmp.clone();
                    }
                },
                EDataTypes::EmbrasulU32(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return data_tmp.clone();
                    }
                },
                EDataTypes::EmbrasulU64(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return data_tmp.clone();
                    }
                },
                EDataTypes::EmbrasulU128(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return data_tmp.clone();
                    }
                },
                EDataTypes::EmbrasulI16(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return data_tmp.clone();
                    }
                },
                EDataTypes::EmbrasulI32(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return data_tmp.clone();
                    }
                },
                EDataTypes::EmbrasulI64(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return data_tmp.clone();
                    }
                },
                EDataTypes::EmbrasulF32(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return data_tmp.clone();
                    }
                }
            };
        }
        return EDataTypes::EmbrasulU16(Point { name: "", offset: 0, length: 1, write_access: false, value: 0 } )
    }

    fn get_data_index(&self, point: &str) -> usize {
        let mut idx = 0;
        for data_tmp in self.data.iter() {
            match data_tmp {
                EDataTypes::EmbrasulString(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return idx;
                    }
                },
                EDataTypes::EmbrasulU16(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return idx;
                    }
                },
                EDataTypes::EmbrasulU32(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return idx;
                    }
                },
                EDataTypes::EmbrasulU64(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return idx;
                    }
                },
                EDataTypes::EmbrasulU128(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return idx;
                    }
                },
                EDataTypes::EmbrasulI16(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return idx;
                    }
                },
                EDataTypes::EmbrasulI32(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return idx;
                    }
                },
                EDataTypes::EmbrasulI64(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return idx;
                    }
                },
                EDataTypes::EmbrasulF32(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return idx;
                    }
                }
            };
            idx += 1;
        }
        return idx;
    }

    fn get_f32(&self, point: &str) -> Option<f32> {
        for data_tmp in self.data.iter() {
            match data_tmp {
                EDataTypes::EmbrasulF32(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return Some(data.value);
                    }
                },
                _ => {},
            }
        }
        return None
    }

    fn get_f32_by_index(&self, idx: usize) -> Option<f32> {
        match self.data[idx] {
            EDataTypes::EmbrasulF32(data) => {
                return Some(data.value);
            },
            _ => return None,
        }
    }

    fn get_string(&self, point: &str) -> Option<String> {
        for data_tmp in self.data.iter() {
            match data_tmp {
                EDataTypes::EmbrasulString(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return Some(data.value.clone());
                    }
                },
                _ => {},
            }
        }
        return None
    }

    fn get_string_by_index(&self, idx: usize) -> Option<String> {
        match &self.data[idx] {
            EDataTypes::EmbrasulString(data) => {
                return Some(data.value.clone());
            },
            _ => return None,
        }
    }

    fn get_u16(&self, point: &str) -> Option<u16> {
        for data_tmp in self.data.iter() {
            match data_tmp {
                EDataTypes::EmbrasulU16(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return Some(data.value);
                    }
                },
                _ => {},
            }
        }
        return None
    }

    fn get_u16_by_index(&self, idx: usize) -> Option<u16> {
        match self.data[idx] {
            EDataTypes::EmbrasulU16(data) => {
                return Some(data.value);
            },
            _ => return None,
        }
    }

    fn get_u32(&self, point: &str) -> Option<u32> {
        for data_tmp in self.data.iter() {
            match data_tmp {
                EDataTypes::EmbrasulU32(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return Some(data.value);
                    }
                },
                _ => {},
            }
        }
        return None
    }

    fn get_u32_by_index(&self, idx: usize) -> Option<u32> {
        match self.data[idx] {
            EDataTypes::EmbrasulU32(data) => {
                return Some(data.value);
            },
            _ => return None,
        }
    }

    fn get_u64(&self, point: &str) -> Option<u64> {
        for data_tmp in self.data.iter() {
            match data_tmp {
                EDataTypes::EmbrasulU64(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return Some(data.value);
                    }
                },
                _ => {},
            }
        }
        return None
    }

    fn get_u64_by_index(&self, idx: usize) -> Option<u64> {
        match self.data[idx] {
            EDataTypes::EmbrasulU64(data) => {
                return Some(data.value);
            },
            _ => return None,
        }
    }

    fn get_u128(&self, point: &str) -> Option<u128> {
        for data_tmp in self.data.iter() {
            match data_tmp {
                EDataTypes::EmbrasulU128(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return Some(data.value);
                    }
                },
                _ => {},
            }
        }
        return None
    }

    fn get_u128_by_index(&self, idx: usize) -> Option<u128> {
        match self.data[idx] {
            EDataTypes::EmbrasulU128(data) => {
                return Some(data.value);
            },
            _ => return None,
        }
    }

    fn get_i16(&self, point: &str) -> Option<i16> {
        for data_tmp in self.data.iter() {
            match data_tmp {
                EDataTypes::EmbrasulI16(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return Some(data.value);
                    }
                },
                _ => {},
            }
        }
        return None
    }

    fn get_i16_by_index(&self, idx: usize) -> Option<i16> {
        match self.data[idx] {
            EDataTypes::EmbrasulI16(data) => {
                return Some(data.value);
            },
            _ => return None,
        }
    }

    fn get_i32(&self, point: &str) -> Option<i32> {
        for data_tmp in self.data.iter() {
            match data_tmp {
                EDataTypes::EmbrasulI32(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return Some(data.value);
                    }
                },
                _ => {},
            }
        }
        None
    }

    fn get_i32_by_index(&self, idx: usize) -> Option<i32> {
        match self.data[idx] {
            EDataTypes::EmbrasulI32(data) => {
                Some(data.value)
            },
            _ => None,
        }
    }

    fn get_i64(&self, point: &str) -> Option<i64> {
        for data_tmp in self.data.iter() {
            if let EDataTypes::EmbrasulI64(data) = data_tmp {
                if data.name.contains(point) && (data.name.len() == point.len()) {
                    return Some(data.value);
                }
            }
        }
        None
    }

    fn get_i64_by_index(&self, idx: usize) -> Option<i64> {
        match self.data[idx] {
            EDataTypes::EmbrasulI64(data) => {
                Some(data.value)
            },
            _ => None,
        }
    }

    fn print(&self) {
        println!("Model {}:", self.model_number);
        for data in self.data.iter() {
            match data {
                EDataTypes::EmbrasulF32(data) => println!("{}: {}", data.name, data.value),
                EDataTypes::EmbrasulU16(data) => println!("{}: {}", data.name, data.value),
                EDataTypes::EmbrasulU32(data) => println!("{}: {}", data.name, data.value),
                EDataTypes::EmbrasulU64(data) => println!("{}: {}", data.name, data.value),
                EDataTypes::EmbrasulU128(data) => println!("{}: {}", data.name, data.value),
                EDataTypes::EmbrasulI16(data) => println!("{}: {}", data.name, data.value),
                EDataTypes::EmbrasulI32(data) => println!("{}: {}", data.name, data.value),
                EDataTypes::EmbrasulI64(data) => println!("{}: {}", data.name, data.value),
                EDataTypes::EmbrasulString(data) => println!("{}: {}", data.name, data.value.clone()),
            }
        }
        println!(" ");
    }
}


pub fn srt_to_vec_u8(src: &str, mut dst: &mut [u8]){
    dst.write_all(src.as_bytes()).unwrap();
}


impl From<EModel> for Vec<u16> {
    fn from(from: EModel) -> Self {
        let mut registers: Vec<u16> = vec![0; 2];
        registers[0] = from.model_number;
        registers[1] = from.qtd;

        for data in from.data.iter() {
            match data {
                EDataTypes::EmbrasulF32(data) => registers.extend(f32::encode(data.value)),
                EDataTypes::EmbrasulU16(data) => registers.extend(u16::encode(data.value)),
                EDataTypes::EmbrasulU32(data) => registers.extend(u32::encode(data.value)),
                EDataTypes::EmbrasulU64(data) => registers.extend(u64::encode(data.value)),
                EDataTypes::EmbrasulU128(data) => registers.extend(u128::encode(data.value)),
                EDataTypes::EmbrasulI16(data) => registers.extend(i16::encode(data.value)),
                EDataTypes::EmbrasulI32(data) => registers.extend(i32::encode(data.value)),
                EDataTypes::EmbrasulI64(data) => registers.extend(i64::encode(data.value)),
                EDataTypes::EmbrasulString(data) => registers.extend(Point::<String>::encode(data.clone())),
            }
        }
        registers
    }
}

impl From<(Vec<u16>, u16, u16, &EModel)> for EModel {
    fn from(from: (Vec<u16>, u16, u16, &EModel)) -> Self {
        let mut model1 = from.3.clone();
        let mut offset = from.1;
        let mut qtd = from.2;

        while qtd > 0 {
            for data in model1.data.iter_mut() {
                match data {
                    EDataTypes::EmbrasulString(data) => {
                        if offset == data.offset {
                            let slice = from.0[data.offset as usize..(data.offset + data.length) as usize].to_vec();
                            data.value = Point::<String>::decode(slice).value;
                            offset += data.length;
                            qtd -= data.length;
                        }
                    },
                    EDataTypes::EmbrasulU16(data) => {
                        if offset == data.offset {
                            let slice = from.0[data.offset as usize..(data.offset + data.length) as usize].to_vec();
                            data.value = u16::decode(slice);
                            offset += data.length;
                            qtd -= data.length;
                        }
                    },
                    EDataTypes::EmbrasulU32(data) => {
                        if offset == data.offset {
                            let slice = from.0[data.offset as usize..(data.offset + data.length) as usize].to_vec();
                            data.value = u32::decode(slice);
                            offset += data.length;
                            qtd -= data.length;
                        }
                    },
                    EDataTypes::EmbrasulU64(data) => {
                        if offset == data.offset {
                            let slice = from.0[data.offset as usize..(data.offset + data.length) as usize].to_vec();
                            data.value = u64::decode(slice);
                            offset += data.length;
                            qtd -= data.length;
                        }
                    },
                    EDataTypes::EmbrasulU128(data) => {
                        if offset == data.offset {
                            let slice = from.0[data.offset as usize..(data.offset + data.length) as usize].to_vec();
                            data.value = u128::decode(slice);
                            offset += data.length;
                            qtd -= data.length;
                        }
                    },
                    EDataTypes::EmbrasulI16(data) => {
                        if offset == data.offset {
                            let slice = from.0[data.offset as usize..(data.offset + data.length) as usize].to_vec();
                            data.value = i16::decode(slice);
                            offset += data.length;
                            qtd -= data.length;
                        }
                    },
                    EDataTypes::EmbrasulI32(data) => {
                        if offset == data.offset {
                            let slice = from.0[data.offset as usize..(data.offset + data.length) as usize].to_vec();
                            data.value = i32::decode(slice);
                            offset += data.length;
                            qtd -= data.length;
                        }
                    },
                    EDataTypes::EmbrasulI64(data) => {
                        if offset == data.offset {
                            let slice = from.0[data.offset as usize..(data.offset + data.length) as usize].to_vec();
                            data.value = i64::decode(slice);
                            offset += data.length;
                            qtd -= data.length;
                        }
                    },
                    EDataTypes::EmbrasulF32(data) => {
                        if offset == data.offset {
                            let slice = from.0[data.offset as usize..(data.offset + data.length) as usize].to_vec();
                            data.value = f32::decode(slice);
                            offset += data.length;
                            qtd -= data.length;
                        }
                    },
                }
            }
        }
        model1
    }
}
