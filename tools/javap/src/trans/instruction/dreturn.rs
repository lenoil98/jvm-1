use classfile::OpCode;
use super::{Instruction, InstructionInfo};

pub struct Dreturn;

impl Instruction for Dreturn {
   fn run(&self, codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
       let info = InstructionInfo {
           name: OpCode::dreturn.into(),
           code: codes[pc],
           icp: 0
       };

       (info, pc + 1)
   }
}