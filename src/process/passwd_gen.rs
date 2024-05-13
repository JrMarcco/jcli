use rand::seq::SliceRandom;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn process_passwd_gen(
    len: u8,
    no_upper: bool,
    no_lower: bool,
    no_number: bool,
    no_symbol: bool,
) -> anyhow::Result<String> {
    let mut rng = rand::thread_rng();
    let mut passwd = Vec::with_capacity(len as usize);
    let mut chars = Vec::new();

    if !no_upper {
        chars.extend_from_slice(UPPER);
        passwd.push(*UPPER.choose(&mut rng).expect("Upper won't be empty."));
    }

    if !no_lower {
        chars.extend_from_slice(LOWER);
        passwd.push(*LOWER.choose(&mut rng).expect("Lower won't be empty."))
    }

    if !no_number {
        chars.extend_from_slice(NUMBER);
        passwd.push(*NUMBER.choose(&mut rng).expect("Number won't be empty."))
    }

    if !no_symbol {
        chars.extend_from_slice(SYMBOL);
        passwd.push(*SYMBOL.choose(&mut rng).expect("Symbol won't be empty."))
    }

    for _ in 0..(len - passwd.len() as u8) {
        let c = chars
            .choose(&mut rng)
            .expect("chars won't be empty in this context.");

        passwd.push(*c);
    }

    passwd.shuffle(&mut rng);

    Ok(String::from_utf8(passwd)?)
}
