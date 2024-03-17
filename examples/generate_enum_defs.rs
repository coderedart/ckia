mod helper;

#[cfg(feature = "mlua")]
pub fn main() {
    use mlua::Table;

    let ctx = helper::HelperContext::new([800, 600]);
    let lua = &ctx.lua;
    let ckia_table: Table = lua.globals().get("ckia").unwrap();
    let enums_table: Table = ckia_table.get("enums").unwrap();
    let mut definitions = "export type enums = {".to_string();
    for pair in enums_table.pairs::<String, Table>() {
        if let Ok((enum_name, variants)) = pair {
            definitions.push('\n');
            definitions.push_str(&enum_name);
            definitions.push_str(": {");
            for pair in variants.pairs::<String, u32>() {
                let (variant_name, _) = pair.unwrap();
                definitions.push('\n');
                definitions.push_str("['");
                definitions.push_str(&variant_name);
                definitions.push_str("']");
                definitions.push_str(": number,");
            }
            definitions.push_str("\n},");
        }
    }
    definitions.push_str("}");
    std::fs::write("enums_defs.d.luau", &definitions).unwrap();
}

#[cfg(not(feature = "mlua"))]
pub fn main() {
    eprintln!("This example requires mlua feature to be enabled");
}
