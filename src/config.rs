pub const MAX_SOURCE_FILE_SIZE_IN_MB = "100" // must be able to be stored in "SourceSize" bytes
//pub const MAX_BINARY_FILE_SIZE = "100M" // when including precompiled binaries
// C/C++ max for code completion, syn highlight, etc: 500k characters (~0.5 MBs)
// IntelliJ max to open a file 20.000k bytes (20 MBs)
// IntelliJ max code assistance 2.500k bytes (2.5 MBs)

pub type SourceSize u32;