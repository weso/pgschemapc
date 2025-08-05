[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.16738599.svg)](https://doi.org/10.5281/zenodo.16738599)

# PG-Schema_PC

This repository contains a prototype implementation for the PG-Schema with Property Constraints extension.


## Overview

PG-Schema_PC is a formal extension of the PG-Schema language, designed to support constraints over property sets in property graphs. 
It introduces structural, cardinality, and range constraints that enhance the precision and expressiveness of schema definitions. 
This implementation serves as a reference interpreter for the abstract grammar and semantics presented in the associated publication.

## Building and installation

The code has been implemented in Rust and it can be run using [cargo](https://doc.rust-lang.org/cargo/).

The following command compiles and generates an executable called `pgschemapc` in the `target/release` folder:

```sh
cargo build --release
```

Once it has been created, you can add it to your executable path or run `target/release/pgschemapc` (we will simpligy it as just `pgschemapc`).

The command `pgschemapc --help` gives information about the available commands. 


```sh
pgschemapc --help
A simple prototype tool to process and validate PG-Schemas with property constraints

Usage: pgschemapc [COMMAND]

Commands:
  pgs       Process and validate property graph schemas
  pg        Process and validate property graphs
  map       Process and validate type map associations
  validate  
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
## Running a simple validation

The following command shows how the tool can be used to validate a simple property graph with a PG-Schema_PC schema according to some type maps:

```sh
pgschemapc validate --graph examples/simple.pg --schema examples/simple.pgs --map examples/simple.map
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
