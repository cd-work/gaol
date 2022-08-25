// Any copyright is dedicated to the Public Domain.
// http://creativecommons.org/publicdomain/zero/1.0/

use std::env;
use std::net::{TcpListener, TcpStream};

use gaol::profile::{AddressPattern, Operation, Profile};
use gaol::sandbox::{ChildSandbox, ChildSandboxMethods, Command, Sandbox, SandboxMethods};

static ADDRESS: &'static str = "127.0.0.1:7357";

fn allowance_profile() -> Profile {
    Profile::new(vec![Operation::NetworkOutbound(AddressPattern::All)]).unwrap()
}

fn prohibition_profile() -> Profile {
    Profile::new(Vec::new()).unwrap()
}

fn allowance_test() {
    ChildSandbox::new(allowance_profile()).activate().unwrap();
    drop(TcpStream::connect(ADDRESS).unwrap())
}

fn prohibition_test() {
    ChildSandbox::new(prohibition_profile()).activate().unwrap();
    drop(TcpStream::connect(ADDRESS).unwrap())
}

pub fn main() {
    match env::args().skip(1).next() {
        Some(ref arg) if arg == "allowance_test" => return allowance_test(),
        Some(ref arg) if arg == "prohibition_test" => return prohibition_test(),
        _ => {},
    }

    let listener = TcpListener::bind(ADDRESS).unwrap();
    let _acceptor = listener.incoming();

    let allowance_status = Sandbox::new(allowance_profile())
        .start(Command::me().unwrap().arg("allowance_test"))
        .unwrap()
        .wait()
        .unwrap();
    assert!(allowance_status.success());

    let prohibition_status = Sandbox::new(prohibition_profile())
        .start(Command::me().unwrap().arg("prohibition_test"))
        .unwrap()
        .wait()
        .unwrap();
    assert!(!prohibition_status.success());
}
