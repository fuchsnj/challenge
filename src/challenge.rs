use crate::crypto;
use rand::RngCore;
use std::collections::HashMap;
use std::io::Write;
use std::net::{IpAddr, Shutdown, TcpStream};
use std::time::{Duration, Instant};

const TIME_LIMIT: Duration = Duration::from_secs(1);
const MIN_PARTS: usize = 3;

pub struct OneTimePad {
    value: Vec<u8>,
    created: Instant,
}

pub enum FailureReason {
    InvalidSecretKey,
    LateSubmission,
}

pub struct Challenge {
    players: HashMap<IpAddr, TcpStream>,
    one_time_pad: Option<OneTimePad>,
    round: u64,
    secret: String,
}

impl Challenge {
    pub fn new(secret: &str) -> Challenge {
        Challenge {
            players: HashMap::new(),
            one_time_pad: None,
            round: 0,
            secret: secret.to_owned(),
        }
    }

    pub fn add_player(&mut self, stream: TcpStream) {
        if let Ok(addr) = stream.peer_addr() {
            if self.players.contains_key(&addr.ip()) {
                let _result = stream.shutdown(Shutdown::Both);
            } else {
                self.players.insert(addr.ip(), stream);
            }
        }
    }

    pub fn submit_secret_key(&mut self, submission: &[u8]) -> Result<String, FailureReason> {
        if let Some(one_time_pad) = self.one_time_pad.take() {
            let result = Self::verify_submission(submission, &one_time_pad);
            match &result {
                Ok(_) => println!("Round {} success: Encrypted secret released", self.round),
                Err(FailureReason::InvalidSecretKey) => {
                    println!("Round {} failed: Invalid submission", self.round)
                }
                Err(FailureReason::LateSubmission) => {
                    println!("Round {} failed: late submission received", self.round)
                }
            }
            result.map(|_| {
                let mut secret_bytes: Vec<u8> = self.secret.as_bytes().to_owned();
                crypto::xor_in_place(&mut secret_bytes, &one_time_pad.value);
                base64::encode(&secret_bytes)
            })
        } else {
            Err(FailureReason::LateSubmission)
        }
    }

    fn verify_submission(
        submission: &[u8],
        one_time_pad: &OneTimePad,
    ) -> Result<(), FailureReason> {
        let response_time = Instant::now() - one_time_pad.created;
        if response_time < TIME_LIMIT {
            if one_time_pad.value == submission {
                Ok(())
            } else {
                Err(FailureReason::InvalidSecretKey)
            }
        } else {
            Err(FailureReason::LateSubmission)
        }
    }

    pub fn start_new_round(&mut self) {
        if self.one_time_pad.is_some() {
            println!("Round {} expired: no submissions received", self.round);
        }
        let round_start = Instant::now();
        self.round += 1;
        println!("\nRound {} started", self.round);
        let mut random_bytes = vec![0; self.secret.len()];
        rand::thread_rng().fill_bytes(&mut random_bytes);

        let num_players = self.players.len();
        let num_parts = MIN_PARTS.max(num_players);
        let parts = crypto::split_secret(&random_bytes, num_parts);
        println!(
            "Sending {}/{} parts to {} players",
            num_players, num_parts, num_players
        );
        parts
            .into_iter()
            .zip(self.players.drain())
            .for_each(|(part, (_addr, mut stream))| {
                let _result = stream.write_all(&part);
            });

        self.one_time_pad = Some(OneTimePad {
            value: random_bytes,
            created: round_start,
        });
    }
}
