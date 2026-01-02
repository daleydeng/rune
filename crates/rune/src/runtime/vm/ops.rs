use core::ops::{
    Add, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, Mul, Rem, Sub,
};

use crate::runtime::{
    FloatType, InstArithmeticOp, InstBitwiseOp, InstShiftOp, Protocol, SignedType, UnsignedType,
    VmErrorKind,
};

pub(super) struct ArithmeticOps {
    pub(super) protocol: Protocol,
    pub(super) error: fn() -> VmErrorKind,
    pub(super) signed: fn(SignedType, SignedType) -> Option<SignedType>,
    pub(super) unsigned: fn(UnsignedType, UnsignedType) -> Option<UnsignedType>,
    pub(super) float: fn(FloatType, FloatType) -> FloatType,
}

impl ArithmeticOps {
    pub(super) fn from_op(op: InstArithmeticOp) -> &'static Self {
        match op {
            InstArithmeticOp::Add => &Self {
                protocol: Protocol::ADD,
                error: || VmErrorKind::Overflow,
                signed: SignedType::checked_add,
                unsigned: UnsignedType::checked_add,
                float: FloatType::add,
            },
            InstArithmeticOp::Sub => &Self {
                protocol: Protocol::SUB,
                error: || VmErrorKind::Underflow,
                signed: SignedType::checked_sub,
                unsigned: UnsignedType::checked_sub,
                float: FloatType::sub,
            },
            InstArithmeticOp::Mul => &Self {
                protocol: Protocol::MUL,
                error: || VmErrorKind::Overflow,
                signed: SignedType::checked_mul,
                unsigned: UnsignedType::checked_mul,
                float: FloatType::mul,
            },
            InstArithmeticOp::Div => &Self {
                protocol: Protocol::DIV,
                error: || VmErrorKind::DivideByZero,
                signed: SignedType::checked_div,
                unsigned: UnsignedType::checked_div,
                float: FloatType::div,
            },
            InstArithmeticOp::Rem => &Self {
                protocol: Protocol::REM,
                error: || VmErrorKind::DivideByZero,
                signed: SignedType::checked_rem,
                unsigned: UnsignedType::checked_rem,
                float: FloatType::rem,
            },
        }
    }
}

pub(super) struct AssignArithmeticOps {
    pub(super) protocol: Protocol,
    pub(super) error: fn() -> VmErrorKind,
    pub(super) signed: fn(SignedType, SignedType) -> Option<SignedType>,
    pub(super) unsigned: fn(UnsignedType, UnsignedType) -> Option<UnsignedType>,
    pub(super) float: fn(FloatType, FloatType) -> FloatType,
}

impl AssignArithmeticOps {
    pub(super) fn from_op(op: InstArithmeticOp) -> &'static AssignArithmeticOps {
        match op {
            InstArithmeticOp::Add => &Self {
                protocol: Protocol::ADD_ASSIGN,
                error: || VmErrorKind::Overflow,
                signed: SignedType::checked_add,
                unsigned: UnsignedType::checked_add,
                float: FloatType::add,
            },
            InstArithmeticOp::Sub => &Self {
                protocol: Protocol::SUB_ASSIGN,
                error: || VmErrorKind::Underflow,
                signed: SignedType::checked_sub,
                unsigned: UnsignedType::checked_sub,
                float: FloatType::sub,
            },
            InstArithmeticOp::Mul => &Self {
                protocol: Protocol::MUL_ASSIGN,
                error: || VmErrorKind::Overflow,
                signed: SignedType::checked_mul,
                unsigned: UnsignedType::checked_mul,
                float: FloatType::mul,
            },
            InstArithmeticOp::Div => &Self {
                protocol: Protocol::DIV_ASSIGN,
                error: || VmErrorKind::DivideByZero,
                signed: SignedType::checked_div,
                unsigned: UnsignedType::checked_div,
                float: FloatType::div,
            },
            InstArithmeticOp::Rem => &Self {
                protocol: Protocol::REM_ASSIGN,
                error: || VmErrorKind::DivideByZero,
                signed: SignedType::checked_rem,
                unsigned: UnsignedType::checked_rem,
                float: FloatType::rem,
            },
        }
    }
}

pub(super) struct AssignBitwiseOps {
    pub(super) protocol: Protocol,
    pub(super) signed: fn(&mut SignedType, SignedType),
    pub(super) unsigned: fn(&mut UnsignedType, UnsignedType),
    pub(super) bool: fn(&mut bool, bool),
}

impl AssignBitwiseOps {
    pub(super) fn from_ops(op: InstBitwiseOp) -> &'static Self {
        match op {
            InstBitwiseOp::BitAnd => &Self {
                protocol: Protocol::BIT_AND_ASSIGN,
                signed: SignedType::bitand_assign,
                unsigned: UnsignedType::bitand_assign,
                bool: bool::bitand_assign,
            },
            InstBitwiseOp::BitXor => &Self {
                protocol: Protocol::BIT_XOR_ASSIGN,
                signed: SignedType::bitxor_assign,
                unsigned: UnsignedType::bitxor_assign,
                bool: bool::bitxor_assign,
            },
            InstBitwiseOp::BitOr => &Self {
                protocol: Protocol::BIT_OR_ASSIGN,
                signed: SignedType::bitor_assign,
                unsigned: UnsignedType::bitor_assign,
                bool: bool::bitor_assign,
            },
        }
    }
}

pub(super) struct BitwiseOps {
    pub(super) protocol: Protocol,
    pub(super) signed: fn(SignedType, SignedType) -> SignedType,
    pub(super) unsigned: fn(UnsignedType, UnsignedType) -> UnsignedType,
    pub(super) bool: fn(bool, bool) -> bool,
}

impl BitwiseOps {
    pub(super) fn from_op(op: InstBitwiseOp) -> &'static BitwiseOps {
        match op {
            InstBitwiseOp::BitAnd => &BitwiseOps {
                protocol: Protocol::BIT_AND,
                signed: SignedType::bitand,
                unsigned: UnsignedType::bitand,
                bool: bool::bitand,
            },
            InstBitwiseOp::BitXor => &BitwiseOps {
                protocol: Protocol::BIT_XOR,
                signed: SignedType::bitxor,
                unsigned: UnsignedType::bitxor,
                bool: bool::bitxor,
            },
            InstBitwiseOp::BitOr => &BitwiseOps {
                protocol: Protocol::BIT_OR,
                signed: SignedType::bitor,
                unsigned: UnsignedType::bitor,
                bool: bool::bitor,
            },
        }
    }
}

pub(super) struct AssignShiftOps {
    pub(super) protocol: Protocol,
    pub(super) error: fn() -> VmErrorKind,
    pub(super) signed: fn(SignedType, u32) -> Option<SignedType>,
    pub(super) unsigned: fn(UnsignedType, u32) -> Option<UnsignedType>,
}

impl AssignShiftOps {
    pub(super) fn from_op(op: InstShiftOp) -> &'static AssignShiftOps {
        match op {
            InstShiftOp::Shl => &Self {
                protocol: Protocol::SHL_ASSIGN,
                error: || VmErrorKind::Overflow,
                signed: SignedType::checked_shl,
                unsigned: UnsignedType::checked_shl,
            },
            InstShiftOp::Shr => &Self {
                protocol: Protocol::SHR_ASSIGN,
                error: || VmErrorKind::Underflow,
                signed: SignedType::checked_shr,
                unsigned: UnsignedType::checked_shr,
            },
        }
    }
}

pub(super) struct ShiftOps {
    pub(super) protocol: Protocol,
    pub(super) error: fn() -> VmErrorKind,
    pub(super) signed: fn(SignedType, u32) -> Option<SignedType>,
    pub(super) unsigned: fn(UnsignedType, u32) -> Option<UnsignedType>,
}

impl ShiftOps {
    pub(super) fn from_op(op: InstShiftOp) -> &'static Self {
        match op {
            InstShiftOp::Shl => &Self {
                protocol: Protocol::SHL,
                error: || VmErrorKind::Overflow,
                signed: SignedType::checked_shl,
                unsigned: UnsignedType::checked_shl,
            },
            InstShiftOp::Shr => &Self {
                protocol: Protocol::SHR,
                error: || VmErrorKind::Underflow,
                signed: SignedType::checked_shr,
                unsigned: UnsignedType::checked_shr,
            },
        }
    }
}
