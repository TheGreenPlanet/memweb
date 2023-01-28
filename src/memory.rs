use libc::{SYS_process_vm_writev, SYS_process_vm_readv, c_int, pid_t, iovec, syscall, c_void};
use errno::{errno};
use std::io;

fn process_vm_writev(pid: pid_t, local_iov: &iovec, liovcnt: c_int, remote_iov: &iovec, riovcnt: c_int) -> isize {
    unsafe { syscall(SYS_process_vm_writev, pid, local_iov as *const iovec, liovcnt, remote_iov as *const iovec, riovcnt) as isize }
}

fn process_vm_readev(pid: pid_t, local_iov: &iovec, liovcnt: c_int, remote_iov: &iovec, riovcnt: c_int) -> isize {
    unsafe { syscall(SYS_process_vm_readv, pid, local_iov as *const iovec, liovcnt, remote_iov as *const iovec, riovcnt) as isize }
}

pub struct Memory {
    pid: pid_t
}

impl Memory {
    pub fn new(pid: i32) -> Memory {
        Memory { pid }
    }

    pub fn read(&self, address: u64, size: usize) -> io::Result<Vec<u8>> {
        let mut result = vec![0; size];
        let local_iov = iovec { iov_base: result.as_mut_ptr() as *mut c_void, iov_len: size};
        let remote_iov = iovec { iov_base: address as *mut c_void, iov_len: size};

        let bytes_read = process_vm_readev(self.pid, &local_iov, 1, &remote_iov, 1);

        if bytes_read == -1 {
            let e = errno();
            let error_string = format!("Error {}: {}", e.0, e);
            return Err(io::Error::new(io::ErrorKind::Other, error_string));
        } else if bytes_read as usize != size {
            return Err(io::Error::new(io::ErrorKind::Other, "Partial read occurred!"));
        } else {
            Ok(result)
        }
    }

    pub fn write(&self, address: u64, buffer: &[u8]) -> io::Result<()> {
        let size = buffer.len();
        let local_iov = iovec { iov_base: buffer.as_ptr() as *mut c_void, iov_len: size};
        let remote_iov = iovec { iov_base: address as *mut c_void, iov_len: size};

        let bytes_written = process_vm_writev(self.pid, &local_iov, 1, &remote_iov, 1);

        if bytes_written == -1 {
            let e = errno();
            let error_string = format!("Error {}: {}", e.0, e);
            return Err(io::Error::new(io::ErrorKind::Other, error_string));         
        } else if bytes_written as usize != size {
            return Err(io::Error::new(io::ErrorKind::Other, "Partial write occurred!"));
        } else {
            Ok(())
        }
    }
}