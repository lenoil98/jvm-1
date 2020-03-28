use classfile::OpCode;
use super::{Instruction, InstructionInfo};

pub struct Areturn;

impl Instruction for Areturn {
   fn run(&self, codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
       let info = InstructionInfo {
           name: OpCode::areturn.into(),
           code: codes[pc],
           icp: 0
       };

       (info, pc + 1)
   }
}