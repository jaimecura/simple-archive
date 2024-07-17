
# simple-archive

`simple-archive` is the simplest possbible crate to handle compressed archives and file streams.

Under the hood it uses the libarchive library to handle data.
There is a direct ffi covnersion from the libarchive library that can also be used, but
the purpose of the library is to provide a simpler API in the rust world on top of libarchive.

---

## Dependencies

`libarchive` must be installed. At the time of this writing the only version tested with the
library is 3.7.4. Older version might work just fine, but it's simply untested

Since `libarchive` is an umbrella on top of other libraries, depending on the desired data
format to be handled, additional libraries should also be installed in the system.

## Features

* read file formats compatible with libarchive
* filters on top of output format files supported by libarchive
* directly compress source data object with Read+Seek traits
* extract objects from archive data. 

Compress files
```rust
use std::fs::File;

let output = File::create("tests/fixtures_out/compressed.tar.gz").unwrap();
let mut a = ArchiveWriter::new(output).unwrap();
a.set_output_targz().unwrap();
a.open().unwrap(); 
a.add_file("/path/to/your/file", "path/inside/output/archive").unwrap();
```

Uncompress files

```rust
use std::fs::File;

let input = File::open("tests/fixtures_out/compressed.tar.gz").unwrap();
let mut a = ArchiveReader::new(output).unwrap();
...
```

## License

Licensed under either of

 * MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

