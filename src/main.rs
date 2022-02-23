use std::{
    fs,
    io::Result,
    io::Write,
    path::PathBuf,
};

use syn;

mod cwitem;

fn main() {
    let source_code = read_from_file();

    let syntax = syn::parse_file(&source_code).unwrap();
    let _ = write_items_to_files(syntax);
}

fn read_from_file() -> String {
    let filepath = PathBuf::from("./samples/cw20.rs");
    let code = fs::read_to_string(&filepath).unwrap();
    return code;
}

fn write_items_to_files(syntax: syn::File) -> Result<()> {
    fs::create_dir_all("./out")?;
    for item in syntax.items {
        match item {
            syn::Item::Fn(f) => {
                let filepath = String::from("./out/") + &f.sig.ident.to_string();
                println!("{}", filepath);
                let cwi = cwitem::from_item(f.sig.ident.to_string());
                let mut file = fs::File::create(filepath).unwrap();
                writeln!(&mut file, "{:#?}", cwi).unwrap();
            },
            _ => continue,
        }
    }
    Ok(())
}
