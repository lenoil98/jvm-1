use classfile::OpCode;
use super::{Instruction, InstructionInfo};

pub struct Ifne;

impl Instruction for Ifne {
   fn run(&self, codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
       let info = InstructionInfo {
           name: OpCode::ifne.into(),
           code: codes[pc],
           icp: 0
       };

       (info, pc + 3)
   }
}