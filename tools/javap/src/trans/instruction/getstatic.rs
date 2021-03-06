use super::{Instruction, InstructionInfo};
use classfile::OpCode;

pub struct Getstatic;

impl Instruction for Getstatic {
    fn run(&self, codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
        let info = InstructionInfo {
            pc,
            op_code: OpCode::getstatic,
            icp: self.calc_cp_index_u16(codes, pc),
            wide: false,
        };

        (info, pc + 3)
    }
}
