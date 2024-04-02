use clap::Parser;

use crate::VERSION;

#[derive(Parser, Debug, Clone)]
#[command()]
pub struct Cmd;

impl Cmd {
    pub fn run() {
        let v = VERSION;
        println!(
            "diamcircle-xdr {} ({})
xdr (+curr): {}
xdr (+next): {}",
            v.pkg, v.rev, v.xdr_curr, v.xdr_next
        );
    }
}
