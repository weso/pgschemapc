# PG-Schema_PC

This repository contains a prototype implementation for the PG-Schema with Property Constraints extension.


## Overview

PG-Schema_PC is a formal extension of the PG-Schema language, designed to support constraints over property sets in property graphs. 
It introduces structural, cardinality, and range constraints that enhance the precision and expressiveness of schema definitions. 
This implementation serves as a reference interpreter for the abstract grammar and semantics presented in the associated publication.

## Building and installation

The code has been implemented in Rust and it can be run using [cargo](https://doc.rust-lang.org/cargo/).

The following command compiles and runs the simple command line tool:

```sh
cargo run
```

As an example, it is possible to run a simple example as:

```sh
cargo run pgs --schema examples/simple.pgs
```

## Running the tests

To run a test suite or validate your own PG-Schema definitions, use the provided command:

```sh
cargo test
```

The tests contain the examples in the paper as well as other test-cases.

At this stage, the tool is a prototype for PGSchema with property constraints validation. 
Further integration with graph databases or external datasets will require extending the parsing and validation layers.

## Related Resources

- Railroad diagrams: [https://domel.github.io/PG-Schema-pc](https://domel.github.io/PG-Schema-pc)
- Citation metadata: see `CITATION.cff`
- GitHub repository: [https://github.com/domel/PG-Schema-pc](https://github.com/domel/PG-Schema-pc)

## Citing

If you use this software or refer to its underlying formalism in academic work, please cite it using the metadata in the `CITATION.cff` file.

---

Licensed under the MIT License.
