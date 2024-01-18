#![allow(non_upper_case_globals)]

use crate::ir::Op;

#[rustfmt::skip]
pub fn sys_read(memory: &mut Vec<u8>) {
    memory.extend(b"\x57");                         // push rdi
    memory.extend(b"\x48\xc7\xc0\x01\x00\x00\x00"); // mov rax, 1
    memory.extend(b"\x48\x89\xfe");                 // mov rsi, rdi
    memory.extend(b"\x48\xc7\xc7\x01\x00\x00\x00"); // mov rdi, 1
    memory.extend(b"\x48\xc7\xc2\x01\x00\x00\x00"); // mov rdx, 1
    memory.extend(b"\x0f\x05");                     // syscall
    memory.extend(b"\x5f");                         // pop rdi
}

pub fn inc(memory: &mut Vec<u8>, op: &Op) {
    assert!(op.operand < 256, "TODO: support bigger operands");
    memory.extend(b"\x80\x07"); // add byte[rdi],
    memory.push((op.operand & 0xFF) as u8);
}
pub fn dec(memory: &mut Vec<u8>, op: &Op) {
    assert!(op.operand < 256, "TODO: support bigger operands");
    memory.extend(b"\x80\x2f"); // sub byte[rdi],
    memory.push((op.operand & 0xFF) as u8);
}
