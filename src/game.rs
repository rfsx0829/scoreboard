use std::fmt;

#[derive(Debug, Clone, Copy, RustcEncodable, RustcDecodable)]
pub enum MinesweeperMode {
    Easy,
    Middle,
    Expert,
    Error,
}

impl fmt::Display for MinesweeperMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MinesweeperMode::Easy => "easy",
                MinesweeperMode::Middle => "middle",
                MinesweeperMode::Expert => "expert",
                MinesweeperMode::Error => "error",
            }
        )
    }
}

impl MinesweeperMode {
    pub fn from(mode: &str) -> MinesweeperMode {
        match mode {
            "easy" => MinesweeperMode::Easy,
            "middle" => MinesweeperMode::Middle,
            "expert" => MinesweeperMode::Expert,
            _ => MinesweeperMode::Error,
        }
    }
}
