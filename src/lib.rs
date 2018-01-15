extern crate futures;
extern crate ratelimit;

use futures::{Async, Future, Poll};
use ratelimit::Handle;

pub struct RatelimitFuture {
  handle: Handle,
}

impl RatelimitFuture {
  pub fn new(handle: Handle) -> RatelimitFuture {
    RatelimitFuture {
      handle,
    }
  }
}

impl Future for RatelimitFuture {
  type Item = ();
  type Error = ();

  fn poll(&mut self) -> Poll<(), ()> {
    if self.handle.try_wait().is_ok() {
      Ok(Async::Ready(()))
    } else {
      Ok(Async::NotReady)
    }
  }
}
