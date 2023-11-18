use std::char;
use clap::Parser;

#[derive(Parser)]
#[command(author, version)]
#[command(arg_required_else_help(true))]
struct Args {
    /// Encryption/decryption key
    #[arg(value_name = "KEY")]
    key: String,

    /// Text to encrypt/decrypt
    #[arg(value_name = "TEXT")]
    text: String,

    /// Decrypt text (instead of encrypting)
    #[arg(short = 'd')]
    decrypt: bool,
}

fn main() {
    let args = Args::parse();

    let key_vec: Vec<u32> = args.key.chars().map(|c| c as u32).collect();

    let result: String = args.text
        .chars()
        .enumerate()
        .map(|(i, v)| {
            char::from_u32(
                if args.decrypt { v as u32 + key_vec[i % key_vec.len()] - 200 }
                else            { v as u32 - key_vec[i % key_vec.len()] + 200 }
            ).unwrap()
        })
        .collect();

    println!("{result}");
}
