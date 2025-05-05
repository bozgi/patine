pub trait Command {
    fn execute(&mut self) -> Result<String, String>;

    // fn serialize(&self) -> Result<String, String>;
    //
    // fn deserialize(&mut self, data: String) -> Result<String, String>;
}