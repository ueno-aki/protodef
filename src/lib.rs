mod reader;
#[cfg(test)]
mod test;
mod writer;

pub mod prelude {
    pub use crate::reader::{NativeReader, ProtodefReader, ReadError};
    pub use crate::writer::{NativeWriter, ProtodefWriter};
}
