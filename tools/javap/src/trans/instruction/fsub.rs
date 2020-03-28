use classfile::OpCode;
use super::{Instruction, InstructionInfo};

pub struct Fsub;

impl Instruction for Fsub {
   fn run(&self, codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
       let info = InstructionInfo {
           name: OpCode::fsub.into(),
           code: codes[pc],
           icp: 0
       };

       (info, pc + 1)
   }
}