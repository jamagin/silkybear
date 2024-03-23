use std::env;
use fork::{fork, Fork};
use elf::ElfBytes;
use elf::endian::AnyEndian;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = std::path::PathBuf::from(&args[1]);
    let file_data = std::fs::read(path).expect("Could not read file.");
    let slice = file_data.as_slice();
    let file = ElfBytes::<AnyEndian>::minimal_parse(slice).expect("ElfBytes parse error");


    // Get the section header table alongside its string table
    let (shdrs_opt, strtab_opt) = file
        .section_headers_with_strtab()
        .expect("shdrs offsets should be valid");
    let (shdrs, strtab) = (
        shdrs_opt.expect("Should have shdrs"),
        strtab_opt.expect("Should have strtab")
    );

    let mut possible_text_shdr = None;
    let mut text_shdr_index = 0;
    for header in shdrs {
        if strtab.get(header.sh_name as usize).expect("Failed to get section name") == ".text" {
            possible_text_shdr = Some(header.clone());
            break;
        }
        text_shdr_index += 1;
    }

    let text_shdr = possible_text_shdr.expect("Didn't find .text header");
    println!(".text loaded at {:x} size {:x}", text_shdr.sh_addr, text_shdr.sh_size);


    let common = file.find_common_data().expect("shdrs should parse");
    let (symtab, strtab) = (common.symtab.expect("symbols not found"), common.symtab_strs.expect("symbols string table not found"));

    let mut addrs: Vec<u64> = symtab.iter().map(|sym| {
        sym.st_value + text_shdr.sh_addr
    }).collect();
    addrs.sort();

    for addr in addrs {
        print!("{:x} ", addr);
    }



    if let Ok(Fork::Child) = fork() {
        let _err = exec::Command::new(&args[1]).args(&args[2..]).exec();
    }
}
