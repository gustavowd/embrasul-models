use super::*;

pub fn model2() -> EModel {
    let mut ret = EModel {
        start_addr: 66,
        end_addr: 123,
        model_number: 2,
        qtd: 58,
        update: false,
        data: Vec::new(),
    };
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "FreqA", offset: 0, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "UrmsA", offset: 2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "UrmsB", offset: 4, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "UrmsC", offset: 6, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "IrmsA", offset: 8, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "IrmsB", offset: 10, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "IrmsC", offset: 12, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "PotAtivA", offset: 14, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "PotAtivB", offset: 16, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "PotAtivC", offset: 18, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "PotAtivT", offset: 20, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "PotReatA", offset: 22, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "PotReatB", offset: 24, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "PotReatC", offset: 26, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "PotReatT", offset: 28, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "PotAparA", offset: 30, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "PotAparB", offset: 32, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "PotAparC", offset: 34, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "PotAparT", offset: 36, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "FatPotA", offset: 38, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "FatPotB", offset: 40, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "FatPotC", offset: 42, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "FatPotT", offset: 44, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "DhtUA", offset: 46, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "DhtUB", offset: 48, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "DhtUC", offset: 50, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "DhtIA", offset: 52, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "DhtIB", offset: 54, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "DhtIC", offset: 56, length: 2, write_access: false, value: f32::NAN } ));
    
    ret
}