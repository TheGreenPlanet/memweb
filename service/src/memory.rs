use errno::errno;
use libc::{c_int, c_long, c_void, iovec, pid_t, ssize_t, syscall, SYS_process_vm_readv, SYS_process_vm_writev};
use log::info;
use std::io;

fn process_vm_writev(
    pid: pid_t,
    local_iov: &iovec,
    liovcnt: c_int,
    remote_iov: &iovec,
    riovcnt: c_int,
    flags: c_long,
) -> isize {
    unsafe {
        syscall(
            SYS_process_vm_writev,
            pid,
            local_iov as *const iovec,
            liovcnt,
            remote_iov as *const iovec,
            riovcnt,
            flags
        ) as ssize_t
    }
}

fn process_vm_readev(
    pid: pid_t,
    local_iov: &iovec,
    liovcnt: c_int,
    remote_iov: &iovec,
    riovcnt: c_int,
    flags: c_long,
) -> isize {
    unsafe {
        syscall(
            SYS_process_vm_readv,
            pid,
            local_iov as *const iovec,
            liovcnt,
            remote_iov as *const iovec,
            riovcnt,
            flags
        ) as ssize_t
    }
}

pub struct Memory {
    pub pid: pid_t,
}

impl Memory {
    pub fn new(pid: i32) -> Memory {
        Memory { pid }
    }

    #[cfg(not(feature = "fake_read_write"))]
    pub fn read_u64(&self, address: u64, size: usize) -> io::Result<u64> {

        if self.pid == -1 {
            return Err(io::Error::new(io::ErrorKind::Other, "PID not set!"));
        }

        // The result buffer does not need to be allocated on the stack
        let mut result: u64 = 0;

        if size > std::mem::size_of::<u64>() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Size is larger than u64!",
            ));
        }

        let local_iov = iovec {
            iov_base: &mut result as *mut _ as *mut c_void,
            iov_len: size,
        };
        let remote_iov = iovec {
            iov_base: address as *mut c_void,
            iov_len: size,
        };

        let bytes_read = process_vm_readev(self.pid, &local_iov, 1, &remote_iov, 1, 0);

        if bytes_read == -1 {
            let e = errno();
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error {}: {}", e.0, e),
            ));
        } else if bytes_read as usize != size {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Partial read occurred!",
            ));
        } else {
            Ok(result)
        }
    }

    #[cfg(not(feature = "fake_read_write"))]
    pub fn read_i64(&self, address: u64, size: usize) -> io::Result<i64> {

        if self.pid == -1 {
            return Err(io::Error::new(io::ErrorKind::Other, "PID not set!"));
        }

        // The result buffer does not need to be allocated on the stack
        let mut result: i64 = 0;

        if size > std::mem::size_of::<i64>() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Size is larger than i64!",
            ));
        }

        let local_iov = iovec {
            iov_base: &mut result as *mut _ as *mut c_void,
            iov_len: size,
        };
        let remote_iov = iovec {
            iov_base: address as *mut c_void,
            iov_len: size,
        };

        let bytes_read = process_vm_readev(self.pid, &local_iov, 1, &remote_iov, 1, 0);

        if bytes_read == -1 {
            let e = errno();
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error {}: {}", e.0, e),
            ));
        } else if bytes_read as usize != size {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Partial read occurred!",
            ));
        } else {
            Ok(result)
        }
    }

    #[cfg(not(feature = "fake_read_write"))]
    pub fn read_vec(&self, address: u64, size: usize) -> io::Result<Vec<u8>> {
        use std::vec;

        if self.pid == -1 {
            return Err(io::Error::new(io::ErrorKind::Other, "PID not set!"));
        }

        // The result buffer does not need to be allocated on the stack
        let mut result = vec![0; size];

        let local_iov = iovec {
            iov_base: result.as_mut_ptr() as *mut c_void,
            iov_len: size,
        };
        let remote_iov = iovec {
            iov_base: address as *mut c_void,
            iov_len: size,
        };

        let bytes_read = process_vm_readev(self.pid, &local_iov, 1, &remote_iov, 1, 0);

        if bytes_read == -1 {
            let e = errno();
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error {}: {}", e.0, e),
            ));
        } else if bytes_read as usize != size {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Partial read occurred!",
            ));
        } else {
            Ok(result)
        }
    }

    #[cfg(feature = "fake_read_write")]
    pub fn read_vec(&self, address: u64, size: usize) -> io::Result<Vec<u8>> {
        if self.pid == -1 {
            return Err(io::Error::new(io::ErrorKind::Other, "PID not set!"));
        }

        let mut result = vec![1; size];
        println!("Fake read: address={:x}, size={:?}", address, size);
        Ok(result)
    }

    #[cfg(not(feature = "fake_read_write"))]
    pub fn write(&self, address: u64, buffer: &[u8]) -> io::Result<usize> {
        if self.pid == -1 {
            return Err(io::Error::new(io::ErrorKind::Other, "PID not set!"));
        }

        let size = buffer.len();
        let local_iov = iovec {
            iov_base: buffer.as_ptr() as *mut c_void,
            iov_len: size,
        };
        let remote_iov = iovec {
            iov_base: address as *mut c_void,
            iov_len: size,
        };

        let bytes_written = process_vm_writev(self.pid, &local_iov, 1, &remote_iov, 1, 0);

        if bytes_written == -1 {
            let e = errno();
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error {}: {}", e.0, e),
            ));
        } else if bytes_written as usize != size {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Partial write occurred!",
            ));
        } else {
            Ok(bytes_written as usize)
        }
    }

    #[cfg(feature = "fake_read_write")]
    pub fn write(&self, address: u64, buffer: &[u8]) -> io::Result<usize> {
        if self.pid == -1 {
            return Err(io::Error::new(io::ErrorKind::Other, "PID not set!"));
        }
        println!("Fake write: address={:x}, bytes={:?}", address, buffer);
        Ok(buffer.len())
    }
}
