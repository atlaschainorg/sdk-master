//! `SystemInstruction` Javascript interface
#![cfg(target_arch = "wasm32")]
#![allow(non_snake_case)]
pub use atlas_sdk_wasm_js::{
    address::Address, hash::Hash, instruction::Instruction, keypair::Keypair, atlas_program_init,
    transaction::Transaction,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct MyProgramInstruction;

fn my_program_instruction(program_id: &atlas_address::Address) -> atlas_instruction::Instruction {
    atlas_instruction::Instruction {
        program_id: *program_id,
        accounts: vec![],
        data: vec![],
    }
}

#[wasm_bindgen]
impl MyProgramInstruction {
    #[wasm_bindgen(constructor)]
    pub fn constructor(program_id: &Address) -> Instruction {
        my_program_instruction(program_id).into()
    }
}
