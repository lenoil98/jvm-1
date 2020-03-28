use classfile::OpCode;
use super::{Instruction, InstructionInfo};

pub struct Sipush;

impl Instruction for Sipush {
   fn run(&self, codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
       let info = InstructionInfo {
           name: OpCode::sipush.into(),
           code: codes[pc],
           icp: 0
       };

       (info, pc + 3)
   }
}