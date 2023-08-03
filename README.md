# 🗑 chemicli


## About

### chemicli is A CLI for getting information about the periodic table of elements, with information based on the information on [PTable](https://ptable.com/#Properties), hence some measurements may not be the most precise in terms of the known values for certain properties, i.e the standard atomic weight of hydrogen is `[1.00784, 1.00811]`, but is `1.008` on PTable.

## Getting Started

#### Make sure you have Rust installed on your machine, if you don't, run the following command in your terminal to install it using rustup:

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

## Installation

#### Run `git clone https://github.com/overoxidize/chemicli.git` in your terminal to install it.



## Usage

* ### General element query:
  * `chem ele "<AtomicSymbol>"` will print a basic overview of an element to your terminal, which will include:
    * #### Atomic Number
    * #### Atomic Weight
    * #### Energy Levels
    * #### Element NAme

* ### Query for specific properties of an element:
  * `chem ele "<AtomicSymbol>" property_1 | property_1 property_3`, which will print the values of the requested properties of the queried element to your terminal.
    