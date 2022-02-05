use super::*;

impl Destination<io::Sink> {
    pub fn sink_file(_path: PathBuf, assigned_lines: usize) -> Result<Self, io::Error> {
        Ok(Self::new(io::sink(), assigned_lines))
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
