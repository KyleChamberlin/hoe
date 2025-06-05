pub fn description(candidate: &str) -> Result<(), &str> {
    if candidate.len() > 1024 {
        return Err("description is too long.");
    }
    Ok(())
}
