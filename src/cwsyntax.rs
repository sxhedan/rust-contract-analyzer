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
    pub fn print_to_files(&self, path: &str) -> Result<()> {
        fs::create_dir_all(path)?;
        for item in &self.items {
            let filepath = String::from(path) + &item.name;
            println!("{}", filepath);
            let mut file = fs::File::create(filepath).unwrap();
            writeln!(&mut file, "{:#?}", item).unwrap();
        }
        Ok(())
    }

    pub fn check(&self) -> Result<()> {
        Ok(())
    }

    pub fn save_results(&self) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct CWItem {
    pub item_type: String,
    pub name: String,
    pub stmts: Vec<Stmt>,
}

impl From<syn::ItemFn> for CWItem {
    fn from(item_fn: syn::ItemFn) -> Self {
        CWItem {
            item_type: "function".to_string(),
            name: item_fn.sig.ident.to_string(),
            stmts: item_fn.block.stmts
                .iter()
                .map(|stmt| {
                    Stmt::from(stmt)
                })
                .collect(),
        }
    }
}

#[derive(Debug)]
pub struct Stmt {
    pub stmt_type: String,
    pub id: String,
}

impl From<&syn::Stmt> for Stmt {
    fn from(stmt: &syn::Stmt) -> Self {
        match stmt {
            syn::Stmt::Expr(expr) => {
                Stmt::default()
            },
            _ => Stmt::default(),
        }
    }
}

impl Stmt {
    pub fn default() -> Stmt {
        Stmt {
            stmt_type: "".to_string(),
            id: "".to_string(),
        }
    }
}
