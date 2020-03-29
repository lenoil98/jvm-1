use super::{Instruction, InstructionInfo};
use classfile::OpCode;

pub struct Fneg;

impl Instruction for Fneg {
    fn run(&self, _codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
        let info = InstructionInfo {
            pc,
            op_code: OpCode::fneg,
            icp: 0,
        };

        (info, pc + 1)
    }
}
