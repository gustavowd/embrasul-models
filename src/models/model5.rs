use super::*;

pub fn model5() -> EModel {
    let mut ret = EModel {
        start_addr: 65001,
        end_addr: 65002,
        model_number: 5,
        qtd: 2,
        update: false,
        data: Vec::new(),
    };
    ret.data.push(EDataTypes::EmbrasulU16(Point { name: "ID", offset: 0, length: 1, write_access: false, value: 0 } ));
    ret.data.push(EDataTypes::EmbrasulU16(Point { name: "Versao", offset: 1, length: 1, write_access: false, value: 0 } ));

    ret
}