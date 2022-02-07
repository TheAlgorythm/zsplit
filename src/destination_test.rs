use super::*;

impl SinkFromPath for Destination<io::Sink> {
    type Sink = io::Sink;

    fn create_sink<P: AsRef<Path>>(path: P) -> Result<Self::Sink, io::Error> {
        path.as_ref()
            .to_str()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, ""))?;

        Ok(io::sink())
    }
}

pub struct IdSink {
    id: u64,
    sink: io::Sink,
}

impl IdSink {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            sink: io::sink(),
        }
    }

    #[inline]
    pub fn id(&self) -> u64 {
        self.id
    }
}

impl Write for IdSink {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.sink.write(buf)
    }
    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.sink.flush()
    }
}
