#![feature(proc_macro_hygiene)]

use skyline::hooks::{getRegionAddress, Region};
use skyline::{hook, install_hook};

use parking_lot::Mutex;

use smash::params::ParamsInfo;
use smash::resource::{find_subsequence, LoadedTables};

type Callback = fn(&ParamsInfo);

static HOOKS: Mutex<Vec<Callback>> = Mutex::new(Vec::new());

static mut LOAD_PRC_FILE_OFFSET: usize = 0x3720530;

static LOAD_PRC_FILE_SEARCH_CODE: &[u8] = &[
    0xff, 0xc3, 0x01, 0xd1, 
    0xff, 0xdf, 0x02, 0xa9, 
    0xf6, 0x57, 0x04, 0xa9, 
    0xf4, 0x4f, 0x05, 0xa9, 
    0xfd, 0x7b, 0x06, 0xa9, 
    0xfd, 0x83, 0x01, 0x91, 
    0xf6, 0x5f, 0x00, 0x32, 
    0x08, 0x81, 0x00, 0xb0, 
    0x08, 0xed, 0x36, 0x91, 
    0xf3, 0x03, 0x00, 0xaa, 
    0xe8, 0x07, 0x00, 0xf9, 
    0xe8, 0x03, 0x00, 0x91, 
    0x3f, 0x00, 0x16, 0x6b,
];

#[hook(offset = LOAD_PRC_FILE_OFFSET)]
unsafe fn handle_load_prc_file(param_obj: u64, table1_idx: u32) -> u64 {
    let loaded_tables = LoadedTables::get_instance();
    let hash = loaded_tables.get_hash_from_t1_index(table1_idx).as_u64();

    let ret = original!()(param_obj, table1_idx);

    let param_info = ParamsInfo::new(&param_obj, &hash);

    for hook in HOOKS.lock().iter() {
        hook(&param_info)
    }

    ret
}

#[skyline::main(name = "param_hook")]
pub fn main() {
    println!("[Param hook] Installing Param hook...");

    unsafe {
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, LOAD_PRC_FILE_SEARCH_CODE) {
            LOAD_PRC_FILE_OFFSET = offset;
            install_hook!(handle_load_prc_file);
            println!("[Param hook] Param hook installed at 0x{:x}.", offset);
        } else {
            println!("Error: no offset found for function 'load_prc_file'. Refusing to install hook.");
        }
    }
}

#[no_mangle]
pub fn add_param_load_hook(callback: Callback) {
    let mut hooks = HOOKS.lock();

    hooks.push(callback);
}
