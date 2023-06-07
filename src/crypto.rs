use rand::{thread_rng, RngCore};

/// This module implements the Fox Secret Sharing Scheme
/// Given a secret, it is broken up into N parts. None
/// of the parts individually contain any information about the secret (except the length).
/// The secret can only be obtained when all N parts are recombined

pub fn split_secret(secret: &[u8], num_parts: usize) -> Vec<Vec<u8>> {
    if num_parts == 0 {
        return vec![];
    }
    if num_parts == 1 {
        return vec![secret.to_owned()];
    }
    let mut parts = vec![];
    for _ in 0..(num_parts - 1) {
        let mut part = vec![0; secret.len()];
        thread_rng().fill_bytes(&mut part);
        parts.push(part);
    }
    let mut xor_sum = parts[0].clone();
    for part in parts[1..].iter() {
        xor_in_place(&mut xor_sum, part);
    }
    xor_in_place(&mut xor_sum, secret);
    parts.push(xor_sum);
    parts
}

pub fn xor_in_place(a: &mut [u8], b: &[u8]) {
    for (x, y) in a.iter_mut().zip(b) {
        *x ^= *y;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn combine_secret(parts: &[Vec<u8>]) -> Vec<u8> {
        let mut secret = vec![0; parts[0].len()];
        for part in parts {
            xor_in_place(&mut secret, part);
        }
        secret
    }

    #[test]
    fn split_combine_secret() {
        let secret = vec![0, 1, 2, 3, 4, 5, 6];
        assert_eq!(combine_secret(&split_secret(&secret, 1)), secret);
        assert_eq!(combine_secret(&split_secret(&secret, 2)), secret);
        assert_eq!(combine_secret(&split_secret(&secret, 100)), secret);

        let parts_1 = split_secret(&secret, 1);
        assert_eq!(parts_1, vec![secret.clone()]);

        let parts_2 = split_secret(&secret, 2);
        assert_ne!(parts_2[0], secret);
        assert_ne!(parts_2[1], secret);
    }
}
