extern crate mio;
extern crate sha1;
extern crate rustc_serialize;
extern crate bytes;
extern crate byteorder;
#[macro_use]
extern crate log;

mod client;
mod server;
pub mod interface;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
