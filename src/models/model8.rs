use super::*;

pub fn model8() -> EModel {
    let mut ret = EModel {
        start_addr: 125,
        end_addr: 156,
        model_number: 8,
        qtd: 32,
        update: false,
        data: Vec::new(),
    };
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "EnergGerA", offset: 0, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "EnergGerB", offset: 2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "EnergGerC", offset: 4, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "EnergGerT", offset: 6, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "ConsumoA", offset: 8, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "ConsumoB", offset: 10, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "ConsumoC", offset: 12, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "ConsumoT", offset: 14, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "EnergindA", offset: 16, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "EnergindB", offset: 18, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "EnergindC", offset: 20, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "EnergindT", offset: 22, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "EnergcapA", offset: 24, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "EnergcapB", offset: 26, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "EnergcapC", offset: 28, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "EnergcapT", offset: 30, length: 2, write_access: false, value: f32::NAN } ));
    
    ret
}
