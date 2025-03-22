//! iq_circuit/src/lib.rs
//!
//! The circuit verifies the legitimacy of an IQ score.
//! Input:
//!   Public input: username (String), iq_score (u32)
//!   Private input: none (in this simple version)
//!
//! Verification:
//!   Simply commits the username and IQ score to the proof
//!   Can be extended with more complex verification logic

#![no_main]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    // Read the username and IQ score
    let username = sp1_zkvm::io::read::<String>();
    let iq_score = sp1_zkvm::io::read::<u32>();

    // You can add validation logic here
    // For example, ensure IQ score is within a valid range
    if iq_score > 200 {
        panic!("IQ score is suspiciously high: {}", iq_score);
    }
    
    // Additional verification logic can be added here
    // For example, you could check against test answers, calculation steps, etc.
    
    // Commit the results (include them in the proof as public input)
    sp1_zkvm::io::commit(&username);
    sp1_zkvm::io::commit(&iq_score);
}
