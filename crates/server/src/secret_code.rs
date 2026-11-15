//! Codes made with the system's secure randomness: the claim code an Owner
//! types (`roles`) and the key `/_stats` asks for (`network::stats`).

use ring::rand::{SecureRandom, SystemRandom};

/// How long a code is: 24 of 62 letters and digits (about 143 bits), and so a
/// name a character could have.
const CODE_LENGTH: usize = 24;

const LETTERS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

/// A new code. An error if the system's secure randomness fails.
pub(crate) fn new_code() -> Result<String, String> {
    let random = SystemRandom::new();
    let mut code = String::with_capacity(CODE_LENGTH);
    while code.len() < CODE_LENGTH {
        let mut byte = [0];
        random
            .fill(&mut byte)
            .map_err(|_| "the system can't provide secure random numbers".to_owned())?;
        // 248 is the last multiple of 62 a byte holds, so each letter is as likely.
        if byte[0] < 248 {
            code.push(LETTERS[usize::from(byte[0]) % LETTERS.len()] as char);
        }
    }
    Ok(code)
}
