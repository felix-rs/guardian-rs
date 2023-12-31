use core::ops::BitAnd;
use vm_proc::handler;
use crate::{binary_op_save_flags, Machine, OpSize};

#[handler]
pub fn and(vm: &mut Machine, op_size: OpSize) {
    binary_op_save_flags!(vm, op_size, bitand, SF, ZF, PF);
}