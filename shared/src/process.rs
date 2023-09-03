use procfs::{*, process::{Process, MMapPath}};


use crate::protocol::{Region, ProcessEntry, EncodedString};

pub fn get_regions(pid: i32) -> std::io::Result<Vec<Region>> {
    let process = Process::new(pid).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let maps = process.maps().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    Ok(maps.iter()
        .map(|map_range| Region {
            start: map_range.address.0,
            end: map_range.address.1,
            size: map_range.address.1 - map_range.address.0,
            permissions: map_range.perms.bits(),
            offset: map_range.offset,
            device: EncodedString::new(map_range.dev.0.to_string() + ":" + &map_range.dev.1.to_string()),
            inode: map_range.inode,
            pathname: match &map_range.pathname {
                MMapPath::Path(path) => EncodedString::new(path.to_str().unwrap().to_string()),
                MMapPath::Other(s) => EncodedString::new(s.clone()),
                MMapPath::Heap => todo!(),
                MMapPath::Stack => todo!(),
                MMapPath::TStack(_) => todo!(),
                MMapPath::Vdso => todo!(),
                MMapPath::Vvar => todo!(),
                MMapPath::Vsyscall => todo!(),
                MMapPath::Rollup => todo!(),
                MMapPath::Anonymous => todo!(),
                MMapPath::Vsys(_) => todo!(),
            }
        })
        .collect())
}

fn create_process_entry(proc: Result<Process, ProcError>) -> ProcessEntry {
    let p = proc.unwrap();
    let mut name = p.cmdline().unwrap().join(" ");
    if name.is_empty() {
        name = p.stat().unwrap().comm;
    }

    ProcessEntry {
        pid: p.pid(),
        name: EncodedString::new(name),
    }
}

pub fn get_running_processes() -> std::io::Result<Vec<ProcessEntry>> {
    let procs = process::all_processes().unwrap();
    Ok(procs
        .map(create_process_entry)
        .collect())
}