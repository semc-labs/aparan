pub use aparan_macros::main;

pub mod system;

pub mod prelude {
    pub use crate::system::System;
}

pub mod exports {
    pub use tokio;
}
