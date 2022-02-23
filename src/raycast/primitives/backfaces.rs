pub enum Backfaces {
    Cull,
    Include,
}

impl Default for Backfaces {
    fn default() -> Self {
        Backfaces::Cull
    }
}
