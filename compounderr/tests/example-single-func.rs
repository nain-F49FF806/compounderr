use compounderr::compose_errors;
use rand::Rng;
use std::{fmt::Error as FmtError, io::Error as IoError};

#[compose_errors]
#[errorset(IoError, FmtError)]
fn moody_task_do() -> Result<(), _> {
    let mut rng = rand::thread_rng();
    // Randomly decide if to error
    if rng.gen::<bool>() {
        let mood = if rng.gen::<bool>() {
            // not feeling like expressing
            FmtError.into()
        } else {
            // stuck on past mood
            IoError::last_os_error().into()
        };
        return Err(mood);
    }
    // Do something cool
    Ok(())
}

#[test]
fn main() {
    let res: Result<(), MoodyTaskDoError> = moody_task_do();
    if res.is_ok() {
        return;
    }
    match res.unwrap_err() {
        MoodyTaskDoError::IoError(e) => println!("an io error {}", e),
        MoodyTaskDoError::FmtError(e) => println!("a formatting error {}", e),
    }
}
