use serde::Serialize;

#[derive(Serialize, Clone, Copy)]
#[serde(rename_all = "UPPERCASE")]
pub enum Command {
    Left,
    Right,
    Down,
    #[serde(rename(serialize = "ACT(3)"))]
    RotateLeft,
    #[serde(rename(serialize = "ACT"))]
    RotateRight,
    #[serde(rename(serialize = "ACT(2)"))]
    Flip,
    #[serde(rename(serialize = "ACT(0,0)"))]
    Clear,
}
