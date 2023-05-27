pub mod game;
pub mod package;
pub mod person;
pub mod scores;

pub mod prelude {
    pub use super::*;

    pub use crate::scores::*;
    pub use crate::game::prelude::*;
    pub use crate::package::prelude::*;
    pub use crate::person::prelude::*;
}
