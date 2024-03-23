use super::*;

pub fn model7() -> EModel {
    let mut ret = EModel {
        start_addr: 50,
        end_addr: 55,
        model_number: 7,
        qtd: 6,
        update: false,
        data: Vec::new(),
    };
    ret.data.push(EDataTypes::EmbrasulU16(Point { name: "Gerador", offset: 0, length: 1, write_access: false, value: 0 } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "SU.EnAtivaGer", offset: 1, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "SU.EnReatGer", offset: 3, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulU16(Point { name: "SU.FPGer", offset: 5, length: 1, write_access: false, value: 0 } ));

    ret
}