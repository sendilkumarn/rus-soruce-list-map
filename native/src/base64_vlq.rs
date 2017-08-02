use std::collections::HashMap;

const VLQ_BASE_SHIFT: i32 = 5;
const VLQ_BASE: i32 = 1 << VLQ_BASE_SHIFT;
const VLQ_BASE_MASK: i32 = VLQ_BASE - 1;
const VLQ_CONTINUATION_BIT: i32 = VLQ_BASE;

pub fn encode(value: i32) -> String {
    let mut base_map: Vec<String> = Vec::new();
    let mut encoded: String = String::from("");
    let mut digit: i32;
    let vlq = to_vlq_signed(value);

    for character in "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".chars() {
        base_map.push(character.to_string());
    }

    while {
        digit = vlq & VLQ_BASE_MASK;
        vlq >> VLQ_BASE_SHIFT;

        if vlq > 0 {
            digit = digit | VLQ_CONTINUATION_BIT;
        }

        if base_map.len() > digit as usize {
            encoded += &base_map[digit as usize];
        }

        vlq > 0
    } {}
    encoded
}


fn to_vlq_signed(value: i32) -> i32 {
    if value < 0 {
        ((-value) << 1) + 1
    } else {
        (value << 1) + 0
    }
}

fn from_vlq_signed(value: i32) -> i32 {
    let is_neg = (value & 1) == 1;
    let shifted = value >> 1;
    if is_neg { -(shifted) } else { shifted }
}

pub fn decode(value: String) -> OutParam {
    let mut base_map: HashMap<char, i32> = HashMap::new();
    let mut idx = 0;
    let i = 0;
    let mut result = 0;
    let mut shift = 0;
    let mut continuation: bool;
    let mut digit: i32;

    for character in "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".chars() {
        base_map.insert(character, idx);
        idx += 1;
    }

    let mut chars = value.chars();

    while {
        match base_map.get(&chars.next().unwrap()) {
            Some(d) => {
                digit = *d;
                continuation = (digit & VLQ_CONTINUATION_BIT).is_positive();
                digit &= VLQ_BASE_MASK;
                result += digit << shift;
                shift += VLQ_BASE_SHIFT;
            }
            None => {
                continuation = true;
            }
        }
        continuation
    } {}

    OutParam {
        value: from_vlq_signed(result),
        rest: value.split_at(i).1.to_string(),
    }
}

#[allow(dead_code)]
pub struct OutParam {
    value: i32,
    rest: String,
}
