use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use actix_web::rt::task;

use sp1_sdk::{ProverClient, SP1Stdin, HashableKey};

#[derive(Deserialize, Debug)]
struct IQResult {
    username: String,
    iq_score: u32,
}

#[derive(Serialize)]
struct ProofData {
    proof: String,        
    public_inputs: String,
    vkey_hash: String,    
    mode: String,         
}

#[post("/generate-proof")]
async fn generate_proof(item: web::Json<IQResult>) -> impl Responder {
    println!("DEBUG: Received IQ result: {:?}", item);
    let iq_result = item.into_inner();

    // Prepare stdin for the circuit
    let mut stdin = SP1Stdin::new();
    println!("DEBUG: Writing username: {}", iq_result.username);
    stdin.write(&iq_result.username);
    
    println!("DEBUG: Writing iq_score: {}", iq_result.iq_score);
    stdin.write(&iq_result.iq_score);

    // Load the IQ verification circuit ELF
    let iq_elf: &[u8] = include_bytes!("../iq-proof.elf");
    println!("DEBUG: Loaded IQ ELF ({} bytes)", iq_elf.len());

    let blocking_result = task::spawn_blocking(move || -> Result<(Vec<u8>, String, Vec<u8>), Box<dyn std::error::Error + Send>> {
        println!("DEBUG: [spawn_blocking] Starting proof generation");
        let client = ProverClient::from_env();
        println!("DEBUG: [spawn_blocking] ProverClient initialized");
        let (pk, vk) = client.setup(iq_elf);
        println!("DEBUG: [spawn_blocking] Setup completed");
        let proof = client.prove(&pk, &stdin).groth16().run()?;
        println!("DEBUG: [spawn_blocking] Proof generated successfully");
        let proof_bytes = proof.bytes().to_vec();
        let public_values_vec = proof.public_values.to_vec();
        let vkey_hash = vk.bytes32();
        println!(
            "DEBUG: [spawn_blocking] Proof details: proof_bytes ({} bytes), public_inputs ({} bytes), vkey_hash: {}",
            proof_bytes.len(),
            public_values_vec.len(),
            vkey_hash
        );
        Ok((proof_bytes, vkey_hash, public_values_vec))
    })
    .await
    .map_err(|e| format!("Blocking task join error: {:?}", e));

    let (proof_bytes, vkey_hash, public_values_vec) = match blocking_result {
        Ok(Ok(tuple)) => {
            println!("DEBUG: [spawn_blocking] Task completed successfully");
            tuple
        },
        Ok(Err(e)) => {
            println!("DEBUG: [spawn_blocking] Task error: {:?}", e);
            return HttpResponse::InternalServerError().body(format!("Proof generation failed: {:?}", e));
        }
        Err(e) => {
            println!("DEBUG: [spawn_blocking] Join error: {:?}", e);
            return HttpResponse::InternalServerError().body(format!("Blocking task join error: {:?}", e));
        }
    };

    let proof_data = ProofData {
        proof: hex::encode(proof_bytes),
        public_inputs: hex::encode(public_values_vec),
        vkey_hash,
        mode: "groth16".to_string(),
    };

    match serde_json::to_string(&proof_data) {
        Ok(json_str) => {
            println!("DEBUG: Successfully serialized proof JSON");
            HttpResponse::Ok().content_type("application/json").body(json_str)
        },
        Err(e) => {
            println!("DEBUG: Serialization error: {:?}", e);
            HttpResponse::InternalServerError().body(format!("Serialization error: {:?}", e))
        }
    }
}

#[actix_web::main(flavor = "multi_thread")]
async fn main() -> std::io::Result<()> {
    println!("Starting IQ proof API server on 0.0.0.0:8080");
    HttpServer::new(|| App::new().service(generate_proof))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
