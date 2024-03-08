use super::*;

pub fn model6() -> EModel {
    let mut ret = EModel {
        start_addr: 0,
        end_addr: 0,
        model_number: 6,
        qtd: 4,
        update: false,
        data: Vec::new(),
    };
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "Energia Ativa", offset: 0, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "Energia Reativa", offset: 2, length: 2, write_access: false, value: f32::NAN } ));

    ret
}