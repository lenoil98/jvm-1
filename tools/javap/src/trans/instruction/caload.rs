use classfile::OpCode;
use super::{Instruction, InstructionInfo};

pub struct Caload;

impl Instruction for Caload {
   fn run(&self, codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
       let info = InstructionInfo {
           name: OpCode::caload.into(),
           code: codes[pc],
           icp: 0
       };

       (info, pc + 1)
   }
}