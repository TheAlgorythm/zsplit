use super::*;

impl SinkFromPath for Destination<io::Sink> {
    type Sink = io::Sink;

    fn create_sink<P: AsRef<Path>>(path: P) -> io::Result<Self::Sink> {
        let path = path
            .as_ref()
            .to_str()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, ""))?;

        if path == "." {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, ""));
        }

        Ok(io::sink())
    }
}
