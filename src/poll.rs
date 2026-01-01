use std::{
    io::{self, Result},
    net::{TcpListener, TcpStream},
    os::fd::AsRawFd,
    time::Duration,
};

use crate::ffi;

type Events = Vec<ffi::Event>;

// Event Queue
pub struct Poll {
    registry: Registry,
}

impl Poll {

    pub fn registry(&self) -> &Registry {
        &self.registry
    }
    pub(crate) fn new() -> Self {
        todo!()
    }


    fn poll(&mut self, events: &mut Events, timeout: Option<Duration>) {}
}

#[derive(Debug)]
pub struct Registry {
    raw_fd: i32,
}

impl Registry {
    fn register(&mut self, source: &mut TcpStream, token: usize, interests: i32) -> Result<()> {
        todo!()
    }
}

impl Drop for Registry {
    /*when a poll instance is dropped, it may also
    cancel in-flight operations for registered event sources*/
    fn drop(&mut self) {
        todo!()
    }
}
