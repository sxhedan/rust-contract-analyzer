use std::{
    fs,
    io::Result,
    io::Write,
};

use syn;

#[derive(Debug)]
pub struct CWSyntax {
    pub items: Vec<CWItem>,
}

impl From<syn::File> for CWSyntax {
    fn from(syntax: syn::File) -> Self {
        let mut cw_syntax = CWSyntax{items: vec![]};
        for item in syntax.items {
            match item {
                syn::Item::Fn(f) => {
                    cw_syntax.items.push(CWItem::from(f));
                },
                _ => continue,
            }
        }
        cw_syntax
    }
}

impl CWSyntax {
    pub fn print_to_files(self, path: &str) -> Result<()> {
        fs::create_dir_all(path)?;
        for item in self.items {
            let filepath = String::from(path) + &item.name;
            println!("{}", filepath);
            let mut file = fs::File::create(filepath).unwrap();
            writeln!(&mut file, "{:#?}", item).unwrap();
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct CWItem {
    pub item_type: String,
    pub name: String,
}

impl From<syn::ItemFn> for CWItem {
    fn from(item_fn: syn::ItemFn) -> Self {
        CWItem{
            item_type: "test".to_string(),
            name: item_fn.sig.ident.to_string(),
        }
    }
}
