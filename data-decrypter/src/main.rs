extern crate cipher_crypt;
extern crate base64;

use cipher_crypt::rot13;
use base64::decode;
use std::str;

fn main() {
    // define the initial text
    let m = "M3I6r2IbMzq9";

    // decode the first rot13 msg top get a base64 value
    let rot13_decrypted_1 = rot13::decrypt(m);

    // decode the base64 value to get plain text message
    let base64_decoded = base64::decode(&rot13_decrypted_1).unwrap();

    // decode the base64 to characters
    let decoded_base64 = str::from_utf8(&base64_decoded).unwrap();

    // final decode from rot13 to ascii 
    let decoded_string = rot13::decrypt(decoded_base64);

    println!("The original message is: {}", m);
    println!("The rot13 decrypted message is: {}", rot13_decrypted_1);
    println!("The base64 decoded message is: {:?}", base64_decoded);
    println!("The base64 decoded message is: {:?}", decoded_base64);
    println!("The base64 decoded message is: {:?}", decoded_string);

//    match base64_decoded {
//        Ok(decoded_bytes) => {
//            let decoded_string = String::from_utf8_lossy(&decoded_bytes);
//            println!("The base64 decoded message is: {:?}", decoded_string);
//        }
//        Err(err) => {
//            println!("Error decoding base64: {:?}", err);
//        }
//    }
}
