use classfile::OpCode;
use super::{Instruction, InstructionInfo};

pub struct If_Icmple;

impl Instruction for If_Icmple {
   fn run(&self, codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
       let info = InstructionInfo {
           name: OpCode::if_icmple.into(),
           code: codes[pc],
           icp: 0
       };

       (info, pc + 3)
   }
}