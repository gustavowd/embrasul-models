use super::*;

pub fn model9() -> EModel {
    let mut ret = EModel {
        start_addr: 138,
        end_addr: 142,
        model_number: 9,
        qtd: 4,
        update: false,
        data: Vec::new(),
    };
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "ConsumoPontaHojeAteAgora", offset: 0, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "ConsumoForaPontaHojeAteAgora", offset: 2, length: 2, write_access: false, value: f32::NAN } ));
    
    ret
}
