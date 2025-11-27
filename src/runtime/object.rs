#[derive(Debug)]
pub enum Object {
    Function(ObjRef),
    // NativeFunction(NativeFunction),
}

pub type ObjRef = usize;
