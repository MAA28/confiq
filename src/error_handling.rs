use derive_more::derive::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    MatchScope(MatchScopeError),
}

#[derive(Debug, From)]
pub enum MatchScopeError {
    CommandFailed,
}
