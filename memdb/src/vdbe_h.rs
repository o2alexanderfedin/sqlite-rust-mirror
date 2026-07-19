use super::*;
use crate::sqlite3_h::{Sqlite3Context, Sqlite3Value};
use crate::sqlite_int_h::{
    CollSeq, FuncDef, Index, KeyInfo, Table, UnpackedRecord, VTable,
};

///* A single VDBE is an opaque structure named "Vdbe".  Only routines
///* in the source file sqliteVdbe.c are allowed to see the insides
///* of this structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Vdbe {
    pub(crate) _opaque: [u8; 0],
}

///* A sub-routine used to implement a trigger program.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct SubProgram {
    pub(crate) a_op: *mut VdbeOp,
    pub(crate) n_op: i32,
    pub(crate) n_mem: i32,
    pub(crate) n_csr: i32,
    pub(crate) a_once: *mut u8,
    pub(crate) token: *mut (),
    pub(crate) p_next: *mut SubProgram,
}

///* A single instruction of the virtual machine has an opcode
///* and as many as three operands.  The instruction is recorded
///* as an instance of the following structure:
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct VdbeOp {
    pub(crate) opcode: u8,
    pub(crate) p4type: i8,
    pub(crate) p5: u16,
    pub(crate) p1: i32,
    pub(crate) p2: i32,
    pub(crate) p3: i32,
    pub(crate) p4: P4union,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union P4union {
    pub(crate) i: i32,
    pub(crate) p: *mut (),
    pub(crate) z: *mut i8,
    pub(crate) p_i64: *mut i64,
    pub(crate) p_real: *mut f64,
    pub(crate) p_func: *mut FuncDef,
    pub(crate) p_ctx: *mut Sqlite3Context,
    pub(crate) p_coll: *mut CollSeq,
    pub(crate) p_mem: *mut Mem,
    pub(crate) p_vtab: *mut VTable,
    pub(crate) p_key_info: *mut KeyInfo,
    pub(crate) ai: *mut u32,
    pub(crate) p_program: *mut SubProgram,
    pub(crate) p_tab: *mut Table,
    pub(crate) p_subrtn_sig: *mut SubrtnSig,
    pub(crate) p_idx: *mut Index,
}

///* The names of the following types declared in vdbeInt.h are required
///* for the VdbeOp definition.
pub(crate) type Mem = Sqlite3Value;

///* A signature for a reusable subroutine that materializes the RHS of
///* an IN operator.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct SubrtnSig {
    pub(crate) sel_id: i32,
    pub(crate) b_complete: u8,
    pub(crate) z_aff: *mut i8,
    pub(crate) i_table: i32,
    pub(crate) i_addr: i32,
    pub(crate) reg_return: i32,
}

///* A smaller version of VdbeOp used for the VdbeAddOpList() function because
///* it takes up less space.
#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct VdbeOpList {
    pub(crate) opcode: u8,
    pub(crate) p1: i8,
    pub(crate) p2: i8,
    pub(crate) p3: i8,
}

pub(crate) type RecordCompare =
    unsafe extern "C" fn(i32, *const (), *mut UnpackedRecord) -> i32;
