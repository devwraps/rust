use memory::{Memory, Pointer};
use error::EvalResult;
use primval::PrimVal;

/// A `Value` represents a single self-contained Rust value.
///
/// A `Value` can either refer to a block of memory inside an allocation (`ByRef`) or to a primitve
/// value held directly, outside of any allocation (`ByVal`).
///
/// For optimization of a few very common cases, there is also a representation for a pair of
/// primitive values (`ByValPair`). It allows Miri to avoid making allocations for checked binary
/// operations and fat pointers. This idea was taken from rustc's trans.
#[derive(Clone, Copy, Debug)]
pub(super) enum Value {
    ByRef(Pointer),
    ByVal(PrimVal),
    ByValPair(PrimVal, PrimVal),
}

impl Value {

    pub(super) fn read_ptr<'a, 'tcx: 'a>(&self, mem: &Memory<'a, 'tcx>) -> EvalResult<'tcx, Pointer> {
        use self::Value::*;
        match *self {
            ByRef(ptr) => mem.read_ptr(ptr),
            ByVal(PrimVal::Ptr(ptr)) |
            ByVal(PrimVal::FnPtr(ptr)) => Ok(ptr),
            ByValPair(..) => unimplemented!(),
            ByVal(_other) => unimplemented!(),
        }
    }

    pub(super) fn read_uint<'a, 'tcx: 'a>(&self, mem: &Memory<'a, 'tcx>, size: usize) -> EvalResult<'tcx, u64> {
        use self::Value::*;
        match *self {
            ByRef(ptr) => mem.read_uint(ptr, size),
            ByVal(PrimVal::U8(u)) => Ok(u as u64),
            ByVal(PrimVal::U16(u)) => Ok(u as u64),
            ByVal(PrimVal::U32(u)) => Ok(u as u64),
            ByVal(PrimVal::U64(u)) => Ok(u as u64),
            ByValPair(..) => unimplemented!(),
            ByVal(_other) => unimplemented!(),
        }
    }

    pub(super) fn to_ptr(&self) -> Pointer {
        use self::Value::*;
        match *self {
            ByRef(ptr) => ptr,
            other => bug!("expected pointer, got {:?}", other),
        }
    }

    pub(super) fn expect_vtable<'a, 'tcx: 'a>(&self, mem: &Memory<'a, 'tcx>) -> EvalResult<'tcx, Pointer> {
        use self::Value::*;
        match *self {
            ByRef(ptr) => mem.read_ptr(ptr.offset(mem.pointer_size() as isize)),
            ByValPair(_, PrimVal::Ptr(vtable)) => Ok(vtable),
            _ => unimplemented!(),
        }
    }

    pub(super) fn expect_slice_len<'a, 'tcx: 'a>(&self, mem: &Memory<'a, 'tcx>) -> EvalResult<'tcx, u64> {
        use self::Value::*;
        match *self {
            ByRef(ptr) => mem.read_usize(ptr.offset(mem.pointer_size() as isize)),
            ByValPair(_, PrimVal::U8(len)) => Ok(len as u64),
            ByValPair(_, PrimVal::U16(len)) => Ok(len as u64),
            ByValPair(_, PrimVal::U32(len)) => Ok(len as u64),
            ByValPair(_, PrimVal::U64(len)) => Ok(len),
            _ => unimplemented!(),
        }
    }
}
