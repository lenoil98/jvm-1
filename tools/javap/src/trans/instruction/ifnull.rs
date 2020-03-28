use classfile::OpCode;
use super::{Instruction, InstructionInfo};

pub struct Ifnull;

impl Instruction for Ifnull {
   fn run(&self, codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
       let info = InstructionInfo {
           name: OpCode::ifnull.into(),
           code: codes[pc],
           icp: 0
       };

       (info, pc + 3)
   }
}