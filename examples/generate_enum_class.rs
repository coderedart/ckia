mod helper;

#[cfg(feature = "mlua")]
pub fn main() {
    use std::collections::{BTreeMap, BTreeSet};

    use mlua::Table;

    let ctx = helper::HelperContext::new([800, 600]);
    let lua = &ctx.lua;
    let ckia_table: Table = lua.globals().get("ckia").unwrap();
    let enums_table: Table = ckia_table.get("enums").unwrap();
    let mut definitions = "".to_string();
    let mut enums_list = "declare class CkiaEnumsList\n".to_string();
    {
        // sort items first and then do something with them.
        let mut enums_pairs = BTreeMap::new();
        for pair in enums_table.pairs::<String, Table>() {
            if let Ok((enum_name, variants)) = pair {
                enums_pairs.insert(enum_name, variants);
            }
        }
        for (enum_name, variants) in enums_pairs {
            definitions.push('\n');
            definitions.push_str("declare class ");
            definitions.push_str(&enum_name);
            definitions.push_str(" extends CkiaEnumItem\n");
            definitions.push_str("end\n");
            definitions.push_str("declare class ");
            definitions.push_str(&enum_name);
            definitions.push_str("_internal");
            definitions.push_str(" extends CkiaEnumClass");
            enums_list.push_str("    ");
            enums_list.push_str(&enum_name);
            enums_list.push_str(": ");
            enums_list.push_str(&enum_name);
            enums_list.push_str("_internal\n");
            // lets sort them first
            let mut variants_list = BTreeSet::new();
            for pair in variants.pairs::<String, u32>() {
                let (variant_name, _) = pair.unwrap();
                variants_list.insert(variant_name);
            }
            for variant_name in variants_list {
                definitions.push_str("\n    ");
                definitions.push_str("['");
                definitions.push_str(&variant_name);
                definitions.push_str("']");
                definitions.push_str(" : ");
                definitions.push_str(&enum_name);
            }
            definitions.push_str("\nend\n");
        }
    }
    enums_list.push_str("end");
    definitions.push_str(&enums_list);
    std::fs::write("enums_class_defs.d.luau", &definitions).unwrap();
}

#[cfg(not(feature = "mlua"))]
pub fn main() {
    eprintln!("This example requires mlua feature to be enabled");
}
