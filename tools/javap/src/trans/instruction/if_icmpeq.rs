use classfile::OpCode;
use super::{Instruction, InstructionInfo};

pub struct If_Icmpeq;

impl Instruction for If_Icmpeq {
   fn run(&self, codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
       let info = InstructionInfo {
           name: OpCode::if_icmpeq.into(),
           code: codes[pc],
           icp: 0
       };

       (info, pc + 3)
   }
}