pub trait Command {
    fn  execute(&mut self) -> Result<String, String>;
}