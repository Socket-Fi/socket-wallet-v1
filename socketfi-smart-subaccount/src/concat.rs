use soroban_sdk::{
    xdr::{FromXdr, ToXdr},
    Env, String,
};

pub fn round_to_four_multiple(n: u32) -> u32 {
    if n % 4 == 0 {
        return n;
    } else {
        return (n + 3) & !3;
    }
}

pub fn strings_concatenator(e: &Env, string1: String, string2: String) -> String {
    let l1 = string1.len();
    let l2 = string2.len();
    let n = string1.len() + string2.len();
    let cur_byte_size = round_to_four_multiple(l1);
    let byte_size = round_to_four_multiple(n);

    let mut combined_xdr = string1.to_xdr(e);
    let string2_xdr = string2.to_xdr(e);

    for _ in 0..(byte_size - cur_byte_size) {
        combined_xdr.push_back(0);
    }

    for i in 8..8 + l2 {
        combined_xdr.set(l1 + i, string2_xdr.get_unchecked(i))
    }

    let new_len = (l1 + l2) as u8;

    combined_xdr.set(7, new_len);

    String::from_xdr(e, &combined_xdr).unwrap()
}
