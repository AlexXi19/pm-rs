pub struct CreateOp {
    pub name: String,
    pub command: String,
}

impl CreateOp {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
