use crate::logger::log;

pub fn start_devkit() -> Result<(), Box<dyn std::error::Error>> {
    log("Starting Cardano DevKit");
    Ok(())
}
