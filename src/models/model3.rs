use super::*;

pub fn model3() -> EModel {
    let mut ret = EModel {
        start_addr: 10,
        end_addr: 21,
        model_number: 3,
        qtd: 12,
        update: false,
        data: Vec::new(),
    };
    ret.data.push(EDataTypes::EmbrasulU16(Point { name: "SU.PulsAtiva", offset: 0, length: 1, write_access: false, value: 0 } ));
    ret.data.push(EDataTypes::EmbrasulU16(Point { name: "SU.PulsReat", offset: 1, length: 1, write_access: false, value: 0 } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "SU.EnAtiva", offset: 2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "SU.EnReat", offset: 4, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulU16(Point { name: "SU.FP", offset: 6, length: 1, write_access: false, value: 0 } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "SU.EAAnt", offset: 7, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "SU.ERAnt", offset: 9, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulU16(Point { name: "SU.FPAnt", offset: 11, length: 1, write_access: false, value: 0 } ));

    ret
}