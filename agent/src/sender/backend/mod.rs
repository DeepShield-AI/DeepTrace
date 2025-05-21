mod elastic;
mod flat_file;
mod remote;

pub use elastic::{Elastic, Error as ElasticError};
pub use flat_file::{Error as FlatFileError, FlatFile};
