use super::{Instruction, InstructionInfo};
use classfile::OpCode;

pub struct Frem;

impl Instruction for Frem {
    fn run(&self, _codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
        let info = InstructionInfo {
            pc,
            op_code: OpCode::frem,
            icp: 0,
            wide: false,
        };

        (info, pc + 1)
    }
}
