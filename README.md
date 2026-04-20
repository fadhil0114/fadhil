# 🚀 Stellar Notes DApp

**Blockchain-Based Decentralized Note-Taking System**

---

## 📌 Project Description

**Stellar Notes DApp** is a decentralized note-taking application built on the Stellar blockchain using the **Soroban Smart Contract SDK**.

This application allows users to create, view, and delete notes directly on the blockchain without relying on centralized servers. All data is stored in an **immutable**, transparent, and secure manner within the smart contract.

Each note is uniquely identified and stored in the contract’s instance storage, ensuring consistency, persistence, and reliability.

---

## 🎯 Project Vision

Our vision is to transform how digital information is stored and managed by:

- 🔗 **Decentralizing Data**  
  Eliminating dependence on centralized servers

- 🔐 **Full Ownership**  
  Giving users complete control over their data

- 🧱 **Immutability**  
  Ensuring data cannot be altered or tampered with

- 🛡️ **High Security**  
  Leveraging blockchain technology for protection

- 🤝 **Trustless System**  
  Relying on code instead of third-party trust

---

## ⚙️ Key Features

### 📝 1. Create Note
- Add new notes with title and content
- Automatically generates a unique ID
- Stored permanently on the blockchain

### 📂 2. Get Notes
- Retrieve all stored notes in a single call
- Structured format for easy frontend integration

### ❌ 3. Delete Note
- Remove notes using their unique ID
- Changes are instantly reflected on-chain

### 🔍 4. Transparency & Security
- All activities are recorded on the blockchain
- Cannot be modified by unauthorized parties

### ⚡ 5. Stellar Integration
- Fast and low-cost transactions
- Built using Soroban SDK
- Scalable and efficient architecture

---

## 📜 Smart Contract Details

- **Network**: Stellar (Soroban)
- **Language**: Rust
- **SDK**: Soroban SDK

**Contract Address:** CCHHJX762XO3RUYBH7KN4MQ6FUHPE6U5IQ7OQIESSSVVKVUFUYBH4GT4
<img width="1870" height="985" alt="image" src="https://github.com/user-attachments/assets/61eb2cdf-99f4-4ee5-86d0-e771c6057cf6" />

---

## 🛠️ Available Functions

| Function         | Description |
|------------------|------------|
| `create_note()`  | Create a new note |
| `get_notes()`    | Retrieve all notes |
| `delete_note()`  | Delete a note by ID |

---

## 🚀 Getting Started

### 1. Prerequisites
- Rust
- Soroban CLI
- Stellar Account

### 2. Build Contract
```bash
cargo build --target wasm32-unknown-unknown --release
