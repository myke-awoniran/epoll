pub const EPOLL_CTL_ADD: i32 = 1;
pub const EPOLLIN: i32 = 0x1;
pub const EPOLLET: i32 = 1 << 31; // shift the bit to the left 31 times

#[link(name = "c")] // This tells the compiler to lay the struct the way C compiler
unsafe extern "C" {
    // the syscall to create an epoll queue
    pub fn epoll_create(size: i32) -> i32;

    // syscall to close the file descriptor
    pub fn close(fd: i32) -> i32;

    // control interface to perform operations on the epoll queue
    pub fn epoll_ctl(epfd: i32, op: i32, fd: i32, event: *mut Event) -> i32;

    // the call that wait for the current thread until one or two things happens
    pub fn epoll_wait(epfd: i32, events: *mut Event, maxevents: i32, timeout: i32) -> i32;
}

#[derive(Debug)]
#[repr(C, packed)]
// Packed to avoid padding bytes after the struct and data fields
// why do we have to avoid padding bytes?
// some of the epoll_data will be added to the padding bytes, hence reading the data will be corrupted
pub struct Event {
    pub(crate) events: i32,
    pub(crate) epoll_data: usize,
}

impl Event {
    pub fn token(&self) -> usize {
        self.epoll_data
    }
}
