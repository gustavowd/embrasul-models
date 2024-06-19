use super::*;

pub fn model1() -> EModel {
    let mut ret = EModel {
        start_addr: 0,
        end_addr: 23,
        model_number: 1,
        qtd: 24,
        update: false,
        data: Vec::new(),
    };
    ret.data.push(EDataTypes::EmbrasulF32(Point { name: "Version", offset: 0, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(EDataTypes::EmbrasulU16(Point { name: "FuncRemota", offset: 2, length: 1, write_access: false, value: 0u16 } ));
    ret.data.push(EDataTypes::EmbrasulU16(Point { name: "Reservado", offset: 3, length: 1, write_access: false, value: 0u16 } ));
    ret.data.push(EDataTypes::EmbrasulU16(Point { name: "relacaoTPpri", offset: 4, length: 1, write_access: false, value: 0u16 } ));
    ret.data.push(EDataTypes::EmbrasulU16(Point { name: "relacaoTPsec", offset: 5, length: 1, write_access: false, value: 0u16 } ));
    ret.data.push(EDataTypes::EmbrasulU16(Point { name: "relacaoTPfat", offset: 6, length: 1, write_access: false, value: 0u16 } ));
    ret.data.push(EDataTypes::EmbrasulU16(Point { name: "relacaoTCpri", offset: 7, length: 1, write_access: false, value: 0u16 } ));
    ret.data.push(EDataTypes::EmbrasulU16(Point { name: "relacaoTCsec", offset: 8, length: 1, write_access: false, value: 0u16 } ));
    ret.data.push(EDataTypes::EmbrasulU16(Point { name: "relacaoTCfat", offset: 9, length: 1, write_access: false, value: 0u16 } ));
    ret.data.push(EDataTypes::EmbrasulU16(Point { name: "RTC_Ano", offset: 10, length: 1, write_access: false, value: 0u16 } ));
    ret.data.push(EDataTypes::EmbrasulU16(Point { name: "RTC_Mes", offset: 11, length: 1, write_access: false, value: 0u16 } ));
    ret.data.push(EDataTypes::EmbrasulU16(Point { name: "RTC_DiaDoAno", offset: 12, length: 1, write_access: false, value: 0u16 } ));
    ret.data.push(EDataTypes::EmbrasulU16(Point { name: "RTC_DiaDaSemana", offset: 13, length: 1, write_access: false, value: 0u16 } ));
    ret.data.push(EDataTypes::EmbrasulU16(Point { name: "RTC_DiaDoMes", offset: 14, length: 1, write_access: false, value: 0u16 } ));
    ret.data.push(EDataTypes::EmbrasulU16(Point { name: "RTC_Hora", offset: 15, length: 1, write_access: false, value: 0u16 } ));
    ret.data.push(EDataTypes::EmbrasulU16(Point { name: "RTC_Minuto", offset: 16, length: 1, write_access: false, value: 0u16 } ));
    ret.data.push(EDataTypes::EmbrasulU16(Point { name: "RTC_Segundo", offset: 17, length: 1, write_access: false, value: 0u16 } ));
    ret.data.push(EDataTypes::EmbrasulU16(Point { name: "hora_ponta_inicio1", offset: 18, length: 1, write_access: false, value: 0u16 } ));
    ret.data.push(EDataTypes::EmbrasulU16(Point { name: "min_ponta_inicio1", offset: 19, length: 1, write_access: false, value: 0u16 } ));
    ret.data.push(EDataTypes::EmbrasulU16(Point { name: "hora_fora_ponta_inicio1", offset: 20, length: 1, write_access: false, value: 0u16 } ));
    ret.data.push(EDataTypes::EmbrasulU16(Point { name: "min_fora_ponta_inicio1", offset: 21, length: 1, write_access: false, value: 0u16 } ));
    ret.data.push(EDataTypes::EmbrasulU16(Point { name: "hora_reserv_inicio1", offset: 22, length: 1, write_access: false, value: 0u16 } ));
    ret.data.push(EDataTypes::EmbrasulU16(Point { name: "min_reserv_inicio1", offset: 23, length: 1, write_access: false, value: 0u16 } ));

    ret
}