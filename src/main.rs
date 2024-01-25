use std::fmt;

fn main() -> fcla::MainResult<RuntimeError> {
    let args = fcla::parse_cla()?.args;
    patch::main(args).map_err(RuntimeError)?;
    Ok(())
}

struct RuntimeError(patch::SubcommandError);

impl fmt::Debug for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(inner) = self;

        fmt::Display::fmt(inner, f)
    }
}
