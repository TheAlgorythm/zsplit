//! Destination for splitting.

#[cfg(not(test))]
use io::BufWriter;
use io::Write;
#[cfg(not(test))]
use std::fs::File;
use std::io;
use std::path::Path;

#[cfg(test)]
#[path = "./destination_test.rs"]
pub mod destination_test;

/// The `sink` with metadata for the splitting operation.
///
/// For anything  IO bound, like filesystem or network, a [`BufWriter`] is recommended.
#[derive(Debug, Clone)]
pub struct Destination<S: Write> {
    /// The number of lines written to the sink per round.
    pub assigned_lines: usize,

    /// Where the splitting operation writes data to.
    pub sink: S,
}

impl<S: Write> Destination<S> {
    /// Creates a [`Destination`].
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use zsplit::prelude::*;
    ///
    /// Destination::new(std::io::sink(), 42);
    /// ```
    #[inline]
    pub fn new(sink: S, assigned_lines: usize) -> Self {
        Self {
            sink,
            assigned_lines,
        }
    }

    /// Creates a [`Destination`] with `1` as a default for `assigned_lines`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use zsplit::prelude::*;
    ///
    /// let destination = Destination::new_with_sink(std::io::sink());
    /// assert_eq!(destination.assigned_lines, 1);
    /// ```
    #[inline]
    pub fn new_with_sink(sink: S) -> Self {
        Self {
            sink,
            assigned_lines: 1,
        }
    }

    /// Consumes the [`Destination`], returning the `sink`.
    ///
    /// This could be used to retrieve the data after the splitting operation.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use zsplit::prelude::*;
    ///
    /// let buffer: Vec<u8> = Destination::buffer().into_sink();
    /// ```
    #[inline]
    pub fn into_sink(self) -> S {
        self.sink
    }
}

impl Destination<Vec<u8>> {
    /// Creates a [`Destination`] with `Vec<u8>` as a raw byte sink and `1` as a default for `assigned_lines`.
    ///
    /// # Warnings
    ///
    /// As the buffer is completely in memory, an unbounded input could lead to a DoS
    /// vulnerability.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// use zsplit::prelude::*;
    ///
    /// Destination::buffer();
    /// ```
    pub fn buffer() -> Self {
        Self::new_with_sink(Vec::new())
    }

    /// Creates a [`Destination`] with `Vec<u8>` as a raw byte sink.
    ///
    /// # Warnings
    ///
    /// As the buffer is completely in memory, an unbounded input could lead to a DoS
    /// vulnerability.
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// use zsplit::prelude::*;
    ///
    /// Destination::buffer_with_lines(42);
    /// ```
    pub fn buffer_with_lines(assigned_lines: usize) -> Self {
        Self::new(Vec::new(), assigned_lines)
    }

    /// Consumes the [`Destination`], returning the raw byte sink as a [`String`].
    ///
    /// # Errors
    ///
    /// Returns `Err` if the raw bytes are not valid UTF-8.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use zsplit::prelude::*;
    /// use std::io::Write;
    ///
    /// let mut destination = Destination::buffer();
    /// destination.write(&[240, 159, 146, 150]);
    ///
    /// let string = destination.into_utf8_string().unwrap();
    ///
    /// assert_eq!("💖", string);
    /// ```
    ///
    /// Incorrect bytes:
    ///
    /// ```
    /// use zsplit::prelude::*;
    /// use std::io::Write;
    ///
    /// let mut destination = Destination::buffer();
    /// destination.write(&[0, 159, 146, 150]);
    ///
    /// assert!(destination.into_utf8_string().is_err());
    /// ```
    pub fn into_utf8_string(self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.into_sink())
    }
}

impl<S> Destination<S>
where
    S: Write,
    Self: SinkFromPath<Sink = S>,
{
    /// Creates a buffered [`File`] and turns it into a [`Destination`] with `1` as a default for `assigned_lines`.
    ///
    /// The `sink` is of type [`BufWriter<File>`].
    ///
    /// # Errors
    ///
    /// Returns `Err` when the `File` can't be created.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use zsplit::prelude::*;
    ///
    /// let destination = Destination::new_with_path("a.txt").unwrap();
    /// assert_eq!(destination.assigned_lines, 1);
    /// ```
    ///
    /// Invalid [`Path`]:
    ///
    /// ```
    /// use zsplit::prelude::*;
    ///
    /// assert!(Destination::new_with_path(".").is_err());
    /// ```
    pub fn new_with_path<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let sink = Self::create_sink(path)?;

        Ok(Self::new_with_sink(sink))
    }

    /// Creates a buffered [`File`] and turns it into a [`Destination`].
    ///
    /// The `sink` is of type [`BufWriter<File>`].
    ///
    /// # Errors
    ///
    /// Returns `Err` when the `File` can't be created.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use zsplit::prelude::*;
    ///
    /// Destination::new_with_path_and_lines("a.txt", 42).unwrap();
    /// ```
    pub fn new_with_path_and_lines<P: AsRef<Path>>(
        path: P,
        assigned_lines: usize,
    ) -> io::Result<Self> {
        let sink = Self::create_sink(path)?;

        Ok(Self::new(sink, assigned_lines))
    }
}

#[doc(hidden)]
pub trait SinkFromPath {
    type Sink: Write;
    fn create_sink<P: AsRef<Path>>(path: P) -> io::Result<Self::Sink>;
}

#[cfg(not(test))]
impl SinkFromPath for Destination<BufWriter<File>> {
    type Sink = BufWriter<File>;
    #[inline]
    fn create_sink<P: AsRef<Path>>(path: P) -> io::Result<Self::Sink> {
        File::create(path).map(BufWriter::new)
    }
}

impl<S: Write> Write for Destination<S> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.sink.write(buf)
    }
    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.sink.flush()
    }
}
