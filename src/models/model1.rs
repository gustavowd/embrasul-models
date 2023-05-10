use super::*;

pub fn model1() -> EModel {
    let mut ret = EModel {
        start_addr: 1,
        end_addr: 66,
        model_number: 1,
        qtd: 66,
        update: false,
        data: Vec::new(),
    };
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "Version", offset: 0, length: 2, write_access: false, value: f32::NAN } ));
    
    ret
}