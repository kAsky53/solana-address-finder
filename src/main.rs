use regex::Regex;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;

fn main() {
    // input regexp pattern here
    let re = Regex::new(r"^(usd|one|ring|rn|rg|rng)").unwrap();

    // iterator
    let mut iter = 0;

    loop {
        let keypair = Keypair::new();
        let public = Pubkey::new(&keypair.to_bytes()[32..64]);
        iter += 1;

        if re.is_match(&(format!("{}", public)).to_lowercase()) {
            println!("Iteration: {}", iter);
            println!("Raw secret {:?}", keypair.to_bytes());
            println!("Secret: {:?}", keypair.secret());
            println!("Public: {}", public);
            println!("==============================================");
        }
    }
}
