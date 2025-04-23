extern crate rand;
use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use sha1::{Sha1, Digest};
use std::convert::TryInto;
use std::io::{self, Write};
use hex::encode;
use rand::Rng;

type AesCbc = Cbc<Aes128, Pkcs7>;

fn main() {
    let id = generate_id();
    println!("Your ID is: {}", id);
    println!("---------------------------------");

    println!("Key initialization:");
    let iv = generate_iv(id.as_bytes());
    let key_vec = generate_key(16);
    let key: &[u8] = &key_vec;

    println!("Key Initialized");

    loop {
        let latitude = get_input("Enter latitude: ");
        let longitude = get_input("Enter longitude: ");

        println!("Choose a method:");
        println!("1. Generate MAC (method 1)");
        println!("2. Concatenate and Hash (method 2)");
        println!("3. Concatenate and Hash Multiple (method 3)");

        let choice = get_input("Enter your choice (1, 2, or 3): ");

        let encrypted_latitude = encrypt(latitude.as_bytes(), key, iv);
        let encrypted_longitude = encrypt(longitude.as_bytes(), key, iv);

        println!("Encrypted Latitude: {:?}", encrypted_latitude);
        println!("Encrypted Longitude: {:?}", encrypted_longitude);

        match choice.as_str() {
            "1" => method1(key, iv, &encrypted_latitude, &encrypted_longitude),
            "2" => method2(key, iv, &encrypted_latitude, &encrypted_longitude, latitude.as_bytes(), longitude.as_bytes()),
            "3" => method3(key, iv, &encrypted_latitude, &encrypted_longitude, latitude.as_bytes(), longitude.as_bytes()),
            _ => println!("Invalid choice. Please enter 1, 2, or 3."),
        }
    }
}

fn generate_id() -> String {
    let name = get_input("Enter your name: ");
    let surname = get_input("Enter your surname: ");
    let full_name = format!("{}{}", name, surname);

    let mut hasher = Sha1::new();
    hasher.update(full_name.as_bytes());
    let hash = hasher.finalize();

    hex::encode(hash)
}

fn generate_iv(salt: &[u8]) -> &[u8] {
    salt[0..16].try_into().expect("slice with incorrect length")
}

fn generate_key(length: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut bytes = vec![0u8; length];
    rng.fill(&mut bytes[..]);
    bytes
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let cipher = AesCbc::new_from_slices(key, iv).expect("Invalid key/iv length");
    cipher.encrypt_vec(data)
}

fn decrypt(encrypted_data: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let cipher = AesCbc::new_from_slices(key, iv).expect("Invalid key/iv length");
    cipher.decrypt_vec(encrypted_data).unwrap()
}

fn generate_mac(key: &[u8], iv: &[u8], message: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(message);
    let hash = hasher.finalize();
    let hash_bytes = hash.as_slice();

    let cipher = AesCbc::new_from_slices(key, iv).expect("Invalid key/iv length");
    let ciphertext = cipher.encrypt_vec(hash_bytes);

    encode(ciphertext)
}

fn concatenate_and_hash(message1: &[u8], message2: &[u8]) -> String {
    let mut concatenated = Vec::new();
    concatenated.extend_from_slice(message1);
    concatenated.extend_from_slice(message2);
    let mut hasher = Sha1::new();
    hasher.update(&concatenated);
    let hash = hasher.finalize();
    encode(hash)
}

fn concatenate_and_hash_multiple(message1: &[u8], message2: &[u8], iteration: u8) -> String {
    let mut concatenated = Vec::new();
    concatenated.extend_from_slice(message1);
    concatenated.extend_from_slice(message2);

    let mut hasher = Sha1::new();
    let mut i = iteration;
    hasher.update(&concatenated);
    let mut hash = hasher.finalize_reset();
    while i > 0 {
        hasher.update(&hash);
        hash = hasher.finalize_reset();
        i = i.wrapping_sub(2);
    }
    encode(hash)
}

fn verification_simple(key: &[u8], iv: &[u8], encrypted_message: &[u8], mac: &str) -> bool {
    let message = decrypt(encrypted_message, key, iv);
    let computed_mac = generate_mac(key, iv, &message);
    computed_mac == mac
}

fn verification_concatenate(key: &[u8], iv: &[u8], encrypted_latitude: &[u8], encrypted_longitude: &[u8], mac: &str) -> bool {
    let decrypted_latitude = decrypt(encrypted_latitude, key, iv);
    let decrypted_longitude = decrypt(encrypted_longitude, key, iv);
    let message = concatenate_and_hash(&decrypted_latitude, &decrypted_longitude);
    let computed_mac = generate_mac(key, iv, message.as_bytes());
    computed_mac == mac
}

fn verification_concatenate_multiple(key: &[u8], iv: &[u8], encrypted_latitude: &[u8], encrypted_longitude: &[u8], iteration: u8, mac: &str) -> bool {
    let decrypted_latitude = decrypt(encrypted_latitude, key, iv);
    let decrypted_longitude = decrypt(encrypted_longitude, key, iv);
    let message = concatenate_and_hash_multiple(&decrypted_latitude, &decrypted_longitude, iteration);
    let computed_mac = generate_mac(key, iv, message.as_bytes());
    computed_mac == mac
}

fn method1(key: &[u8], iv: &[u8], encrypted_latitude: &[u8], encrypted_longitude: &[u8]) {
    let mac1 = generate_mac(key, iv, encrypted_latitude);
    let mac2 = generate_mac(key, iv, encrypted_longitude);
    println!("Generated MACs: {},{}", mac1, mac2);

    if get_input("Do you want to perform verification? (yes/no): ").eq_ignore_ascii_case("yes") {
        let mac_input1 = get_input("Enter the MAC1 to verify: ");
        let mac_input2 = get_input("Enter the MAC2 to verify: ");

        let is_valid1 = verification_simple(key, iv, encrypted_latitude, &mac_input1);
        println!("MAC1 verification result: {}", is_valid1);
        let is_valid2 = verification_simple(key, iv, encrypted_longitude, &mac_input2);
        println!("MAC2 verification result: {}", is_valid2);
    }
}

fn method2(key: &[u8], iv: &[u8], encrypted_latitude: &[u8], encrypted_longitude: &[u8], latitude: &[u8], longitude: &[u8]) {
    let hash = concatenate_and_hash(latitude, longitude);
    println!("Concatenated Hash: {}", hash);
    let mac = generate_mac(key, iv, hash.as_bytes());
    println!("Generated MAC: {}", mac);

    if get_input("Do you want to perform verification? (yes/no): ").eq_ignore_ascii_case("yes") {
        let mac_input = get_input("Enter the MAC to verify: ");
        let is_valid = verification_concatenate(key, iv, encrypted_latitude, encrypted_longitude, &mac_input);
        println!("MAC verification result: {}", is_valid);
    }
}

fn method3(key: &[u8], iv: &[u8], encrypted_latitude: &[u8], encrypted_longitude: &[u8], latitude: &[u8], longitude: &[u8]) {
    let iteration: u8 = get_input("Enter number of iterations: ").trim().parse().expect("Please enter a valid number");
    let hash = concatenate_and_hash_multiple(latitude, longitude, iteration);
    println!("Concatenated Hash with Multiple Iterations: {}", hash);
    let mac = generate_mac(key, iv, hash.as_bytes());
    println!("Generated MAC: {}", mac);

    if get_input("Do you want to perform verification? (yes/no): ").eq_ignore_ascii_case("yes") {
        let mac_input = get_input("Enter the MAC to verify: ");
        let is_valid = verification_concatenate_multiple(key, iv, encrypted_latitude, encrypted_longitude, iteration, &mac_input);
        println!("MAC verification result: {}", is_valid);
    }
}