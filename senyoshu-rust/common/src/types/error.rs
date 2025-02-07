use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    NotAuth,
    WordIsNotExist,
    DatabaseErr,
}
