use crate::runtime::object::ObjRef;

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Int64(i64),
    Uint64(u64),
    Float64(f64),
    Bool(bool),
    Unit,
    Obj(ObjRef),
}
