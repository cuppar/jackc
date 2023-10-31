use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Static,
    Field,
    Arg,
    Var,
}

struct Item {
    type_: String,
    kind: Kind,
    index: i32,
}
pub struct SymbolTable {
    table: HashMap<String, Item>,
    static_count: i32,
    field_count: i32,
    arg_count: i32,
    var_count: i32,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
            static_count: 0,
            field_count: 0,
            arg_count: 0,
            var_count: 0,
        }
    }

    pub fn reset(&mut self) {
        self.table.clear();
        self.static_count = 0;
        self.field_count = 0;
        self.arg_count = 0;
        self.var_count = 0;
    }

    pub fn define(&mut self, name: &str, type_: &str, kind: Kind) {
        use Kind::*;
        let index = match kind {
            Static => {
                let idx = self.static_count;
                self.static_count += 1;
                idx
            }
            Field => {
                let idx = self.field_count;
                self.field_count += 1;
                idx
            }
            Arg => {
                let idx = self.arg_count;
                self.arg_count += 1;
                idx
            }
            Var => {
                let idx = self.var_count;
                self.var_count += 1;
                idx
            }
        };

        self.table.insert(
            name.to_string(),
            Item {
                type_: type_.to_string(),
                kind,
                index,
            },
        );
    }

    pub fn var_count(&self, kind: Kind) -> i32 {
        use Kind::*;
        match kind {
            Static => self.static_count,
            Field => self.field_count,
            Arg => self.arg_count,
            Var => self.var_count,
        }
    }

    pub fn kind_of(&self, name: &str) -> Option<Kind> {
        self.table.get(name).map(|item| item.kind)
    }

    pub fn type_of(&self, name: &str) -> Option<String> {
        self.table.get(name).map(|item| item.type_.to_string())
    }

    pub fn index_of(&self, name: &str) -> Option<i32> {
        self.table.get(name).map(|item| item.index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_define() {
        let mut symble_table = SymbolTable::new();
        symble_table.define("name", "int", Kind::Var);
        assert_eq!(symble_table.var_count, 1);
    }

    #[test]
    fn test_var_count() {
        let mut symble_table = SymbolTable::new();
        assert_eq!(symble_table.var_count(Kind::Var), 0);

        symble_table.define("name", "int", Kind::Var);
        assert_eq!(symble_table.var_count(Kind::Var), 1);
    }

    #[test]
    fn test_kind_of() {
        let mut symble_table = SymbolTable::new();
        let name = "name";
        symble_table.define(name, "int", Kind::Var);

        assert_eq!(symble_table.kind_of(name), Some(Kind::Var));
    }

    #[test]
    fn test_type_of() {
        let mut symble_table = SymbolTable::new();
        let name = "name";
        symble_table.define(name, "int", Kind::Var);

        assert_eq!(symble_table.type_of(name), Some("int".to_string()));
    }

    #[test]
    fn test_index_of() {
        let mut symble_table = SymbolTable::new();
        let name = "name";
        symble_table.define(name, "int", Kind::Var);

        assert_eq!(symble_table.index_of(name), Some(0));
    }
}
