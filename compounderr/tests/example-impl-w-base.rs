use compounderr::compose_errors;

// Custom errors defined with thiserror

mod my_base_errors {
    /// Collection of base error variants used in my library

    #[derive(thiserror::Error, Debug)]
    #[error("Based Error this")]
    pub struct BasedError;

    #[derive(thiserror::Error, Debug)]
    #[error(transparent)]
    pub struct IoError(#[from] std::io::Error);

    #[derive(thiserror::Error, Debug)]
    pub enum ConfigError {
        #[error("Please provide a config file")]
        NotFound(#[from] std::io::Error),
        #[error("Required fields are missing: `{0:?}`")]
        MissingFields(Vec<String>),
    }
}
use my_base_errors::*;

// Impl example, using custom base errors
pub struct Foo;

#[compose_errors]
impl Foo {
    fn function4() -> Result<(), IoError> {
        Ok(())
    }

    #[errorset(ConfigError, BasedError)]
    fn function5(&self) -> Result<String, _> {
        Ok("Am ok".to_owned())
    }
}

#[test]
fn impl_func_expected_error_name() {
    let foo = Foo;
    // match error name in returned result type
    let res: Result<_, FooImplFunction5Error> = foo.function5();
}
