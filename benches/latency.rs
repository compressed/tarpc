// Copyright 2016 Google Inc. All Rights Reserved.
//
// Licensed under the MIT License, <LICENSE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

#![feature(plugin, conservative_impl_trait, test)]
#![plugin(tarpc_plugins)]

#[macro_use]
extern crate tarpc;
#[cfg(test)]
extern crate test;
extern crate env_logger;
extern crate futures;

use futures::Future;
#[cfg(test)]
use test::Bencher;
use tarpc::sync::Connect;
use tarpc::util::{FirstSocketAddr, Never};

service! {
    rpc ack();
}

#[derive(Clone)]
struct Server;

impl FutureService for Server {
    type AckFut = futures::Finished<(), Never>;
    fn ack(&self) -> Self::AckFut {
        futures::finished(())
    }
}

#[cfg(test)]
#[bench]
fn latency(bencher: &mut Bencher) {
    let _ = env_logger::init();
    let server = Server.listen("localhost:0".first_socket_addr()).wait().unwrap();
    let client = SyncClient::connect(server.local_addr()).unwrap();

    bencher.iter(|| {
        client.ack().unwrap();
    });
}
