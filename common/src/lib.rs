use std::rc::Rc;
pub mod canonicalized_path;

pub type ReadonlyList<T> = Rc<[T]>;
