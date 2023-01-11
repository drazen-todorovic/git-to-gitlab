use crate::schema::report::schema::REPORT_RAW_SCHEMA;

pub fn handle() -> anyhow::Result<()> {
    println!("==================================");
    println!("{}", REPORT_RAW_SCHEMA);
    println!("==================================");
    Ok(())
}
