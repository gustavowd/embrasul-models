// All function of this module were copied from a different crate, available at:
// https://github.com/lukaskirner/tokio-sunspec/blob/main/src/utils.rs

pub fn apply_scale_factor(value: u16, scale_factor: u16) -> u16 {
    value * u16::pow(10, scale_factor as u32)
}

pub(crate) fn to_be_bytes(data: Vec<u16>) -> Vec<u8> {
    return data
        .iter()
        .flat_map(|v| v.to_be_bytes())
        .collect::<Vec<u8>>();
}

pub(crate) fn to_u16_vector(data: &[u8]) -> Vec<u16> {
    let chunks = data.chunks_exact(2);
    let remainder = chunks.remainder();

    let mut result: Vec<u16> = chunks
        .into_iter()
        .map(|a| u16::from_be_bytes([a[0], a[1]]))
        .collect();

    if !remainder.is_empty() {
        result.push(u16::from_be_bytes([remainder[0], 0]));
    }

    result
}