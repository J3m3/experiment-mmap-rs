use memmap2::{MmapMut, MmapOptions};
use std::{env, fs, io::Read, mem};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut code = fs::File::open(file_path).expect("failed to open specified file");

    let mmap_file = unsafe {
        MmapOptions::new()
            .map_exec(&code)
            .expect("failed to create memory map backed by file")
    };

    let ptr = mmap_file.as_ptr();
    println!("-------- file-backed memory map --------");
    unsafe {
        (mem::transmute::<*const u8, extern "C" fn()>(ptr))();
    }

    ///////////////////////////////

    let mut buf = Vec::new();
    code.read_to_end(&mut buf)
        .expect("failed to read from file object");

    let mut mmap_anon =
        MmapMut::map_anon(buf.len()).expect("failed to create anonymous memory map");
    mmap_anon.copy_from_slice(&buf);
    let mmap_anon = mmap_anon
        .make_exec()
        .expect("failed to transition the memory map to be executable");

    let ptr = mmap_anon.as_ptr();
    println!("-------- anonymous memory map --------");
    unsafe {
        (mem::transmute::<*const u8, extern "C" fn()>(ptr))();
    }
}
