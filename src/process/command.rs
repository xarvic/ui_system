use crate::state::StorageID;
use crate::process::windows::WindowConstructor;

pub enum EngineCommand {
    OpenWindow(WindowConstructor),
    StateChange(StorageID),

    ShutDown,

}

