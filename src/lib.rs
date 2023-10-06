extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use plot_icon::generate_svg;
use bs58;
use hex;

// make svg from string like:
//   0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
//   5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
#[wasm_bindgen]
pub fn identicon(s: &str) -> String {
    let adr = mkadr(s);
    let svg_document = generate_svg (&adr);
    svg_document.to_string()
}

// return a Westend format like 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
// todo в пизду - problem with 3 last char: checksum
#[wasm_bindgen]
pub fn adr2west(s: &str) -> String {
    let adr = mkadr(s);
    // Prepend the SS58 address format (42 for Polkadot) and append the checksum (two zero bytes for simplicity).
    let mut ss58_bytes = vec![42];
    ss58_bytes.extend_from_slice(&adr);
    ss58_bytes.extend_from_slice(&[10, 10]);
    bs58::encode(ss58_bytes).into_string()
}

// return a hex format like 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
#[wasm_bindgen]
pub fn adr0x(s: &str) -> String {
    let adr = mkadr(s);
    format!("0x{}", hex::encode(adr))
}

fn mkadr(s:&str) -> Vec<u8> {
    if let Some(x) = s.strip_prefix("0x") {
        hex::decode(x).unwrap()
    } else {
        if s.len() == 48 { // format Westend: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
            let bytes = bs58::decode(s).into_vec().unwrap();
            (&bytes[1..bytes.len()-2]).to_vec()
        } else { // return a vector of bytes
            s.as_bytes().to_vec()
        }
    }
}
