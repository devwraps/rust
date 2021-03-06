//! An interpreter for MIR used in CTFE and by miri

mod cast;
mod eval_context;
mod intern;
mod intrinsics;
mod machine;
mod memory;
mod operand;
mod operator;
mod place;
mod step;
mod terminator;
mod traits;
mod validity;
mod visitor;

pub use rustc_middle::mir::interpret::*; // have all the `interpret` symbols in one place: here

pub use self::eval_context::{Frame, InterpCx, LocalState, LocalValue, StackPopCleanup};

pub use self::place::{MPlaceTy, MemPlace, MemPlaceMeta, Place, PlaceTy};

pub use self::memory::{AllocCheck, FnVal, Memory, MemoryKind};

pub use self::machine::{AllocMap, Machine, MayLeak, StackPopJump};

pub use self::operand::{ImmTy, Immediate, OpTy, Operand, ScalarMaybeUndef};

pub use self::visitor::{MutValueVisitor, ValueVisitor};

pub use self::validity::RefTracking;

pub use self::intern::{intern_const_alloc_recursive, InternKind};

crate use self::intrinsics::eval_nullary_intrinsic;
