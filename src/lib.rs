#[cfg(test)]
mod test;
mod reader;
mod writer;

pub mod prelude {
    pub use crate::reader::{NativeReader,ProtodefReader};
    pub use crate::writer::{NativeWriter,ProtodefWriter};
}