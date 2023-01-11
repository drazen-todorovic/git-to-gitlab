use super::schema::CONFIG_RAW_SCHEMA;

pub fn handle() -> anyhow::Result<()> {
    println!("==================================");
    println!("{}", CONFIG_RAW_SCHEMA);
    println!("==================================");
    Ok(())
}
