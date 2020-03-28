use classfile::OpCode;
use super::{Instruction, InstructionInfo};

pub struct D2L;

impl Instruction for D2L {
   fn run(&self, codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
       let info = InstructionInfo {
           name: OpCode::d2l.into(),
           code: codes[pc],
           icp: 0
       };

       (info, pc + 1)
   }
}