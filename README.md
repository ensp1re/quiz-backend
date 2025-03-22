# 🧠 IQ Quiz Proof Generator

Welcome to **IQ Quiz**! 🎉  
This project combines a fun IQ test with zero-knowledge proof verification — allowing users to prove their IQ score authenticity without revealing private test data!  

👉 **Live demo:** [iq-quiz-opal.vercel.app](https://iq-quiz-opal.vercel.app/)  

---

## 🚀 What It Does

- 🌟 Users take an IQ quiz and get their IQ score.  
- 🔒 The score and username are turned into a zero-knowledge proof (ZK Proof), verifying the score without exposing test answers.  
- ✅ The circuit checks that IQ scores are realistic (max 200) and commits them as public inputs.  
- 📡 The backend (`api_server`) generates a zk-proof on-demand using SP1.  

---

## 📦 Project Structure  

| Folder / File            | Description                                                   |
|--------------------------|---------------------------------------------------------------|
| `api_server/`            | API server to generate zero-knowledge proofs from scores     |
| `iq-proof.elf`           | Precompiled ZK circuit in ELF format (loaded by the server)  |
| Frontend (Vercel deploy) | Beautiful IQ quiz app that interacts with the proof backend  |

