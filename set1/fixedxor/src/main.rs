extern crate hex;
fn main() {
    let ciphertext = match hex::decode("1c0111001f010100061a024b53535009181c") {
        Ok(c) => c,
        Err(_) => panic!("could not decode ciphertext")
    };
    let key = match hex::decode("686974207468652062756c6c277320657965") {
        Ok(k) => k,
        Err(_) => panic!("could not decode key")
    };
    for b in ciphertext.iter().zip(key.iter()) {
       match b {
            (c,k) => print!("{}",hex::encode(vec![c^k]))
       };
    }
}
