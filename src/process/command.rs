use crate::process::windows::WindowConstructor;
use crate::state::StorageID;

pub enum EngineCommand {
    OpenWindow(WindowConstructor),
    StateChange(StorageID),
}