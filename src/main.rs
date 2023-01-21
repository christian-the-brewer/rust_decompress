use std::fs;
use std::io;

fn main() {
    std::process::exit(decomp());
}

fn decomp() -> i32 {
    let args: Vec = std::env::args().collect();

    if args.len() < 2 {
        println!("Try: {} <filename>", args[0]);
        return 1;
    }
    //get name of file
    let fname = std::path::Path::new(&*args[1]);
    //open file
    let file = fs::File::open(&fname).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        let outpath = match file.enclosed_name() {};
    }
}
