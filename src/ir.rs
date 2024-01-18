use std::{collections::LinkedList, error, fmt, io, ops};

use crate::lexer::{Lexer, OpKind};

#[derive(Debug, Clone)]
/// Intermediate Representation
pub struct IR(pub Vec<Op>);

impl IR {
    pub fn new(mut lexer: Lexer<'_>) -> Result<Self, IRError> {
        let mut ops = vec![];
        let mut stack = LinkedList::new();
        let mut opkind = lexer.next();
        while let Some(kind) = opkind {
            match kind {
                OpKind::Inc
                | OpKind::Dec
                | OpKind::Left
                | OpKind::Right
                | OpKind::Output
                | OpKind::Input => {
                    let mut op = Op::new(kind, 1);

                    let mut s = lexer.next();
                    while s == Some(kind) {
                        op.operand += 1;
                        s = lexer.next();
                    }

                    ops.push(op);
                    opkind = s;
                }
                OpKind::JumpIfZero => {
                    let addr = ops.len() as isize;
                    ops.push(Op::new(kind, 0));
                    stack.push_back(addr);
                    opkind = lexer.next();
                }
                OpKind::JumpIfNonzero => {
                    let Some(addr) = stack.pop_back() else {
                        return Err(IRError::UncloseJump(lexer.pos()));
                    };
                    ops.push(Op::new(kind, addr + 1));
                    ops[addr as usize].operand = ops.len() as isize;
                    opkind = lexer.next();
                }
            }
        }

        Ok(Self(ops))
    }

    pub fn interprete(&self) -> Result<(), IRError> {
        let mut memory: Vec<isize> = Vec::with_capacity(self.len());
        memory.push(0);
        let mut head: usize = 0;
        let mut ip: usize = 0;
        while ip < self.len() {
            let Some(op) = self.get(ip) else {
                return Err(IRError::EmptyIR);
            };
            match op.kind {
                OpKind::Inc => {
                    let Some(h) = memory.get_mut(head) else {
                        return Err(IRError::MemoryFault);
                    };
                    let Some(toadd) = h.checked_add(op.operand) else {
                        return Err(IRError::OverFlow);
                    };
                    *h = toadd;
                    ip += 1;
                }
                OpKind::Dec => {
                    let Some(h) = memory.get_mut(head) else {
                        return Err(IRError::MemoryFault);
                    };
                    let Some(tosub) = h.checked_sub(op.operand) else {
                        return Err(IRError::OverFlow);
                    };
                    *h = tosub;
                    ip += 1;
                }
                OpKind::Left => {
                    if head < op.operand as usize {
                        return Err(IRError::MemoryUnderflow);
                    }
                    head -= op.operand as usize;
                    ip += 1;
                }
                OpKind::Right => {
                    head += op.operand as usize;
                    while head >= memory.len() {
                        memory.push(0);
                    }
                    ip += 1;
                }
                OpKind::Output => {
                    let Some(h) = memory.get(head) else {
                        return Err(IRError::MemoryFault);
                    };
                    for _ in 0..op.operand {
                        print!("{}", char::from_u32(*h as u32).unwrap_or('\0'));
                    }
                    ip += 1;
                }
                OpKind::Input => {
                    use io::Read;
                    let mut stdin = io::stdin();
                    let mut buf = vec![0u8; 1];
                    stdin.read(&mut buf).map_err(IRError::Io)?;
                    memory[head] = buf.pop().expect("what?") as isize;

                    ip += 1;
                }
                OpKind::JumpIfZero => match memory.get(head) {
                    Some(0) => ip = op.operand as usize,
                    Some(_) => ip += 1,
                    None => return Err(IRError::MemoryFault),
                },
                OpKind::JumpIfNonzero => match memory.get(head) {
                    Some(0) => ip += 1,
                    Some(_) => ip = op.operand as usize,
                    None => return Err(IRError::MemoryFault),
                },
            }
        }

        Ok(())
    }
}
impl ops::Deref for IR {
    type Target = Vec<Op>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub enum IRError {
    Io(io::Error),
    UncloseJump(usize),
    EmptyIR,
    MemoryFault,
    MemoryUnderflow,
    OverFlow,
}
impl error::Error for IRError {}
impl fmt::Display for IRError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IRError::UncloseJump(pos) => write!(f, "{pos}: Unclosed jump `[`/`]`."),
            IRError::EmptyIR => f.write_str("Empty Intermediate Representation"),
            IRError::MemoryFault => f.write_str("Memory Fault, accessing unreachable memory"),
            IRError::MemoryUnderflow => f.write_str("Memory Underflow"),
            IRError::Io(err) => write!(f, "IoError: {err}"),
            IRError::OverFlow => f.write_str("Byte Overflow"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Op {
    pub(crate) kind: OpKind,
    pub(crate) operand: isize,
}
impl Op {
    const fn new(kind: OpKind, operands: isize) -> Self {
        Self {
            kind,
            operand: operands,
        }
    }
}
