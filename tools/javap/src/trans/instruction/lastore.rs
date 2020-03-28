use classfile::OpCode;
use super::{Instruction, InstructionInfo};

pub struct Lastore;

impl Instruction for Lastore {
   fn run(&self, codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
       let info = InstructionInfo {
           name: OpCode::lastore.into(),
           code: codes[pc],
           icp: 0
       };

       (info, pc + 1)
   }
}