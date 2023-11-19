use core::ops::BitAnd;
use crate::{binary_op_save_flags, Machine, OpSize};

pub fn and(vm: &mut Machine, op_size: OpSize) {
    binary_op_save_flags!(vm, op_size, bitand);
}