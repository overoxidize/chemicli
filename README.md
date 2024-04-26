# chemicli


## About

#### chemicli is A CLI for getting information about the periodic table of elements, with information somewhat based on the information on [PTable](https://ptable.com/#Properties), hence some measurements may not be the most precise in terms of the known values for certain properties, i.e the standard atomic weight of hydrogen is `[1.00784, 1.00811]`, but is `1.008` on PTable.

## Getting Started

To install ChemiCLI, you'll need to have Rust and Cargo installed. If you don't have them yet, you can install Rust via [rustup](https://rustup.rs/).

## Installation

Run `git clone https://github.com/overoxidize/chemicli.git` in your terminal to install it.

---

# ChemiCLI

ChemiCLI is a command-line interface (CLI) application written in Rust for querying information about chemical elements from the periodic table.

## Installation

To install ChemiCLI, you'll need to have Rust and Cargo installed. If you don't have them yet, you can install Rust via [rustup](https://rustup.rs/).

Once Rust is installed, you can install ChemiCLI by running:

```sh
cargo install chemicli
```

```sh
chemicli.exe element <ATOMIC_SYMBOL> [OPTIONS]
```

### Arguments

- `<ATOMIC_SYMBOL>`: The atomic symbol of the element you want to query.

### Options

- `-n, --atomic-number`: Include the atomic number of the given element.
- `-w, --atomic-weight`: Include the atomic weight of the given element.
- `-l, --energy-levels`: Include the energy levels of the given element.
- `-e, --electronegativity`: Include the electronegativity of the given element.
- `-s, --series`: Include the series of the given element.
- `-d, --date-discovered`: Include the discovery date of the given element.
- `-D, --discoverer`: Include the discoverer of the given element.
- `-g, --group`: Include the group of the given element.
- `-p, --period`: Include the period of the given element.
- `-h, --help`: Print help.

## Examples

- To query the atomic number of oxygen:
  ```sh
  chemicli.exe element O -n
  ```

- To query the atomic weight and group of iron:
  ```sh
  chemicli.exe element Fe -w -g
  ```

## Data Source

ChemiCLI uses a local JSON object containing information about the chemical elements. This JSON object is based on the periodic table and is stored within the application.
    
## Roadmap

* Add `mtl` command, i.e `chem mtl ...`, to query about metals.
  * Add `sg` (subgroup) subcommand, i.e `chem mtl -sg` for a list of sub-groups of metals, such as alkali earth metals, and lanthanides.