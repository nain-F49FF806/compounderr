use composerr::compose_errors;
use std::fmt::Display;

#[compose_errors]
trait MyTrait {
    #[errorset{IOError, BasedError}]
    fn function1(&self);
    fn function2(&self);
    #[errorset[IOError]]
    fn function3(&self);
}

struct Dummy;

#[compose_errors]
impl Dummy {
    fn function4(&self) {}
    #[errorset{}]
    fn function5(&self) {}
}

impl Display for Dummy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Dummy")
    }
}

mod foo {
    pub struct Dummy2;
}

#[compose_errors]
impl foo::Dummy2 {
    fn function6(&self) {}
    #[errorset(IOError)]
    fn function7(&self) -> Result<String, _> {
        Ok("ok".to_owned())
    }
}

use std::io::Error as IOError;

use thiserror::Error;

#[derive(Error, Debug)]
#[error("Based Error this")]
pub struct BasedError;

#[compose_errors]
#[errorset(IOError, BasedError)]
fn main() -> Result<(), _> {
    // This is just a placeholder to compile the program.
    // See the right side for the generated components.

    let _err: MainError = BasedError.into();

    let err = MainError::BasedError(BasedError);
    let _based: BasedError = err.try_into().unwrap();

    let err = MainError::BasedError(BasedError);
    let _ioerr: IOError = err.try_into().expect("This will fail");

    Ok(())
}
