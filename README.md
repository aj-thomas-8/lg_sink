# lg_sink

lg-sink is a real-time log display library for Rust tui applications on Unix systems. It utilizes [SinkView](https://github.com/aj-thomas-8/sink_view) to support the display of log messages during application runtime.  

__Note__: lg_sink is configured to "write" logs only if the application is built in debug mode.

## Usage
(lg_sink is not currently available as a crate)
- Clone the repo
- Add `lg_sink` as a dependency to the tui Rust project
  ```
  [dependencies]
  lg_sink = { path = "<path-to-lg_sink-repo>/lg_sink" }
  ```
Example use case:
```
use std::io;

fn main() {
    let pg_name = "Novella";
    let err = io::Error::new(io::ErrorKind::NotFound, "NotFound: Did not find file");

    lg_sink::info!("Starting {}", pg_name);
    lg_sink::info!("This is an info message");
    lg_sink::debug!("This is a debug message");
    lg_sink::warn!("This is a warning message");
    lg_sink::error!("We have an error - {}", err);

    println!("Done.");
}
```
