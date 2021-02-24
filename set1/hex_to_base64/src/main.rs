extern crate hex;
extern crate base64;
fn main() {
    let plain = match hex::decode("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d") {
       Ok(v) => v,
       Err(e) => {
            eprintln!("error: {}", e);
            vec![0,0,0]

       },
    };
    println!("{:?}",base64::encode(plain));
}
