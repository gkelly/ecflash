#[macro_use]
extern crate lazy_static;

pub use self::file::EcFile;
pub use self::flash::EcFlash;

mod file;
mod flash;
mod io;

pub trait Ec {
    fn size(&mut self) -> usize;
    fn project(&mut self) -> String;
    fn version(&mut self) -> String;
}
