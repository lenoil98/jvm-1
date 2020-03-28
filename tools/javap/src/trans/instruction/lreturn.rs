use classfile::OpCode;
use super::{Instruction, InstructionInfo};

pub struct Lreturn;

impl Instruction for Lreturn {
   fn run(&self, codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
       let info = InstructionInfo {
           name: OpCode::lreturn.into(),
           code: codes[pc],
           icp: 0
       };

       (info, pc + 1)
   }
}