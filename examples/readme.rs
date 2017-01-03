// Copyright 2016 Google Inc. All Rights Reserved.
//
// Licensed under the MIT License, <LICENSE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

#![feature(conservative_impl_trait, plugin)]
#![plugin(tarpc_plugins)]

extern crate env_logger;
extern crate futures;
#[macro_use]
extern crate log;
#[macro_use]
extern crate tarpc;
extern crate tokio_core;

use futures::Future;
use tarpc::future::Connect;
use tarpc::util::Never;

service! {
    rpc hello(name: String) -> String;
}

#[derive(Clone)]
struct HelloServer;

impl SyncService for HelloServer {
    fn hello(&self, name: String) -> Result<String, Never> {
        info!("Got request: {}", name);
        Ok(format!("Hello, {}!", name))
    }
}

fn main() {
    let _ = env_logger::init();
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let addr = HelloServer.listen("localhost:10000").unwrap();
    let f = FutureClient::connect(&addr)
        .map_err(tarpc::Error::from)
        .and_then(|client| {
            let resp1 = client.hello("Mom".to_string());
            info!("Sent first request.");
            // let resp2 = client.hello("Dad".to_string());
            // info!("Sent second request.");
            //
            futures::collect(vec![resp1 /* resp2 */])
        })
        .map(|responses| for resp in responses {
            println!("{}", resp);
        });
    core.run(f).unwrap();
}
