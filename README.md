# What is this?
Some Rust bindings for the Chrome Dev Tools protocol (stable version v1.3).

This would be better expressed in the form of a proc-macro, but that seemed worse than doing this.

## What's in the rest of this repo?
In the `ts/` folder, is a horribly-coded tool (`parser.ts`) to convert the TS AST into Rust code. It's not pretty, it's not fully automatic, but it mostly does the job.
You will need to manually resolve import errors (among others) yourself however.