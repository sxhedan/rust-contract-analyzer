use std::{
    fs,
    io::Result,
    io::Write,
};

use syn;

#[derive(Debug)]
pub struct CWSyntax {
    pub msgs: Vec<String>,
    pub items: Vec<CWItem>,
}

impl From<syn::File> for CWSyntax {
    fn from(syntax: syn::File) -> Self {
        let mut cw_syntax = CWSyntax{
            msgs: vec![],
            items: vec![],
        };
        for item in syntax.items {
            let cw_item = CWItem::from(&item);
            if cw_item.name == "execute" {
                println!("Parse execute");
                cw_syntax.parse_execute(&item);
            }
            cw_syntax.items.push(cw_item);
        }
        cw_syntax
    }
}

impl CWSyntax {
    pub fn parse_execute(&mut self, item: &syn::Item) {
        // update self.msgs
        match item {
            syn::Item::Fn(f) => {
                for s in &f.block.stmts {
                    match s {
                        syn::Stmt::Expr(e) => {
                            match e {
                                syn::Expr::Match(em) => {
                                    for arm in &em.arms {
                                        match &arm.pat {
                                            syn::Pat::Struct(es) => self.msgs.push(es.path.segments[2].ident.to_string()),
                                            _ => (),
                                        }
                                    }
                                },
                                _ => (),
                            }
                        },
                        _ => (),
                    }
                }
            },
            _ => (),
        }
    }

    pub fn print_to_files(&self, path: &str) -> Result<()> {
        fs::create_dir_all(path)?;
        for item in &self.items {
            if item.name != "default" {
                item.print_to_file(path)?;
            }
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
    pub syn_item: syn::Item,
}

impl From<&syn::Item> for CWItem {
    fn from(syn_item: &syn::Item) -> Self {
        match &syn_item {
            syn::Item::Fn(f) => CWItem {
                item_type: "function".to_string(),
                name: f.sig.ident.to_string(),
                syn_item: syn_item.clone(),
            },
            _ => CWItem {
                item_type: "default".to_string(),
                name: "default".to_string(),
                syn_item: syn_item.clone(),
            },
        }
    }
}

impl CWItem {
    pub fn print_to_file(&self, path: &str) -> Result<()> {
        let filepath = String::from(path) + &self.name;
        // println!("{}", filepath);
        let mut file = fs::File::create(filepath).unwrap();
        writeln!(&mut file, "{:#?}", self).unwrap();
        Ok(())
    }
}
