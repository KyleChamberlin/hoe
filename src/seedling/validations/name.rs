pub fn name(candidate: &str) -> Result<(), &str> {
    if candidate.len() > 255 {
        return Err("name is too long.");
    }
    Ok(())
}
