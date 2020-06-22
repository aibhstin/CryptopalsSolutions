extern crate hex;
extern crate base64;

fn main() {
    let decoded = match hex::decode("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d") {
        Ok(string) => string,
        Err(_) => panic!("Error!")
    };
    let base64_str = base64::encode(decoded);

    println!("{}", base64_str);
}
