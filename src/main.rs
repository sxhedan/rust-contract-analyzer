use std::{
    fs,
    io::Result,
    io::Write,
    path::PathBuf,
};

use syn;

mod cwsyntax;

fn main() {
    let source_code = read_from_file();

    let syntax = syn::parse_file(&source_code).unwrap();
    let cw_syntax = cwsyntax::CWSyntax::from(syntax);
    let _ = cw_syntax.print_to_files("./out/");
}

pub fn read_from_file() -> String {
    let filepath = PathBuf::from("./samples/cw20.rs");
    let code = fs::read_to_string(&filepath).unwrap();
    return code;
}

pub fn write_items_to_files(syntax: syn::File) -> Result<()> {
    fs::create_dir_all("./output_syn")?;
    for item in syntax.items {
        match item {
            syn::Item::Fn(f) => {
                let filepath = String::from("./output_syn/") + &f.sig.ident.to_string();
                println!("{}", filepath);
                let mut file = fs::File::create(filepath).unwrap();
                writeln!(&mut file, "{:#?}", f).unwrap();
            },
            _ => continue,
        }
    }
    Ok(())
}
