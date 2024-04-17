
pub fn format_error_chain(
    err: &impl std::error::Error,
    formatter: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    // Write the description of the error
    writeln!(formatter, "{}\n", err)?;

    // Write down the chain of causes
    let mut current = err.source();
    while let Some(cause) = current {
        writeln!(formatter, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }

    return Ok(())
}