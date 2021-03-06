[![travis](https://travis-ci.org/biotool-paper/biotool.svg?branch=master)](https://travis-ci.org/biotool-paper/biotool)

# Overview 

This project provides a template for implementing command line bioinformatics tools in various programming languages, 
demonstrating best practice using a toy example called `biotool`. Many of the principles adopted by biotool are motivated by the paper "[Ten recommendations for creating usable bioinformatics command line software](http://www.gigasciencejournal.com/content/2/1/15)".

The program reads one or more input FASTA files. For each file it computes a variety of statistics, and then prints a summary of the statistics as output.

The goal is to provide a solid foundation for new bioinformatics command line tools, and is an ideal starting place for new projects. An additional advantage of biotool is that it allows us to compare programming styles in different languages.

Basic features of the tool include:

* Command line argument parsing.
* Reading input from files or optionally from standard input.
* The use of library code for parsing a common bioinformatics file format (FASTA).
* Defined exit status values.
* A test suite. 
* A well-defined version number, following the principles of [Semantic Versioning](http://semver.org/).
* (Where possible) standardised software packaging using programming language specific mechanisms.
* A standard open-source software license. 
* User documentation.
* Code documentation.

Where possible we have tried to follow the recommended conventions for programming style for each implementation language.

# Licence

All of the various implementations of biotool are released as open source software under the terms of [MIT License](https://raw.githubusercontent.com/biotool-paper/biotool/master/LICENSE)

XXX Perhaps we should also provide boilerplate for other standard open source licenses to make it easy for users to choose which one they want.

# General behaviour

Biotool accepts zero or more FASTA filenames  on the command line. If zero filenames are specified it reads a single FASTA file from the standard input device (stdin). Otherwise it reads each named FASTA file in the order specified on the command line. Biotool reads each input FASTA file, computes various statistics about the contents of the file, and then displays a tab-delimited summary of the statistics as output. Each input file produces at most one output line of statistics. Each line of output is prefixed by the input filename or by the text "`stdin`" if the standard input device was used.

Biotool processes each FASTA file one sequence at a time. Therefore the memory usage is proportional to the longest sequence in the file.

An optional command line argument `--minlen` can be supplied. Sequences with length strictly less than the given value will be ignored by biotool and do not contribute to the computed statistics. By default `--minlen` is set to zero.

These are the statistics computed by biotool, for all sequences with length greater-than-or-equal-to `--minlen`:

* *NUMSEQ*: the number of sequences in the file satisfying the minimum length requirement.
* *TOTAL*: the total length of all the counted sequences.
* *MIN*: the minimum length of the counted sequences.
* *AVERAGE*: the average length of the counted sequences rounded down to an integer.
* *MAX*: the maximum length of the counted sequences.

If there are zero sequences counted in a file, the values of MIN, AVERAGE and MAX cannot be computed. In that case biotool will print a dash (`-`) in the place of the numerical value. Note that when `--minlen` is set to a value greater than zero it is possible that an input FASTA file does not contain any sequences with length greater-than-or-equal-to the specified value. If this situation arises biotool acts in the same way as if there are no sequences in the file.

# Installation and usage 

Installation and usage instructions are provided separately for each of the implementations of biotool. Please see the README.md files in the corresponding sub-folders for each implementation.

Each implementation of biotool conforms to a standard command line interface, illustrated below. There may be small cosmetic differences in help messages due to programming language idiosyncrasies, but otherwise the behaviour of all implementations should be the same. The name of the executable program for each version is different, for example, the Python version is called `biotool-py`.

In the examples below, `%` indicates the command line prompt.

## Help message

Biotool can display usage information on the command line via the `-h` or `--help` argument:
```
% biotool -h
Synopsis:
  Print fasta stats
Usage:
  biotool [options] contigs.fasta [another.fa ...]
Options:
  --help       Show this help
  --version    Print version and exit
  --verbose    Print more stuff about what's happening
  --minlen N   Minimum length sequence to include in stats (default=0)
```

## Reading FASTA files named on the command line

Biotool accepts zero or more named FASTA files on the command line. These must be specified following all other command line arguments. If zero files are named, biotool will read a single FASTA file from the standard input device (stdin).

There are no restrictions on the name of the FASTA files. Often FASTA filenames end in `.fa` or `.fasta`, but that is merely a convention, which is not enforced by biotool. 

The example below illustrates biotool applied to a single named FASTA file called `file1.fa`:
```
% biotool file1.fa
FILENAME	NUMSEQ	TOTAL	MIN	AVG	MAX
file1.fa	5264	3801855	31	722	53540
```

The example below illustrates biotool applied to three named FASTA files called `file1.fa`, `file2.fa` and `file3.fa`:
```
% biotool file1.fa file2.fa file3.fa
FILENAME	NUMSEQ	TOTAL	MIN	AVG	MAX
file1.fa	5264	3801855	31	722	53540
file2.fa	5264	3801855	31	722	53540
file3.fa	5264	3801855	31	722	53540
```

## Reading a single FASTA file from standard input 

The example below illustrates biotool reading a FASTA file from standard input. In this example we have redirected the contents of a file called `file1.fa` into the standard input using the shell redirection operator `<`:

```
% biotool < file1.fa
FILENAME	NUMSEQ	TOTAL	MIN	AVG	MAX
stdin	5264	3801855	31	722	53540
```

Equivalently, you could achieve the same result by piping a FASTA file into biotool:

```
% cat file1.fa | biotool
FILENAME	NUMSEQ	TOTAL	MIN	AVG	MAX
stdin	5264	3801855	31	722	53540
```

## Filtering sequences by length 

Biotool provides an optional command line argument `--minlen` which causes it to ignore (not count) any sequences in the input FASTA files with length strictly less than the supplied value. 

The example below illustrates biotool applied to a single FASTA file called `file`.fa` with a `--minlen` filter of `1000`.
```
% biotool --minlen 1000 file.fa
FILENAME	NUMSEQ	TOTAL	MIN	AVG	MAX
file1.fa	4711	2801855	1021	929	53540
```

## Empty files

It is possible that the input FASTA file contains zero sequences, or, when the `--minlen` command line argument is used, it is possible that the file contains no sequences of length greater-than-or-equal-to the supplied value. In both of those cases biotool will not be able to compute minimum, maximum or average sequence lengths, and instead it shows output in the following way:

The example below illustrates biotool applied to a single FASTA file called `empty`.fa` which contains zero sequences:
```
% biotool empty.fa
FILENAME	NUMSEQ	TOTAL	MIN	AVG	MAX
empty.fa	0	0	-	-	-
```

# Exit status values

Biotool returns the following exit status values:

* *0*: The program completed successfully.
* *1*: File I/O error. This can occur if at least one of the input FASTA files cannot be opened for reading. This can occur because the file does not exist at the specified path, or biotool does not have permission to read from the file. 
* *2*: A command line error occurred. This can happen if the user specifies an incorrect command line argument. In this circumstance biotool will also print a usage message to the standard error device (stderr).
* *3*: Input FASTA file is invalid. This can occur if biotool can read an input file but the file format is invalid. 

# Error handling

## Invalid input FASTA files

## Incorrect command line arguments

## Memory limits and other resource restrictions

# Testing

A set of sample test input files is provided in the `test_data` folder. Additionally, each implementation of biotool comes with its own testing facilities which utilise features and libraries of the particular programming language. Instructions for language-specific testing are provided in the README.md files for each implementation.

# Programming languages used to implement biotool

* BASH
* C++
* Java
* Javascript
* Haskell
* Perl5
* Python
* R
* Ruby
* Rust

# Bugs

File at our [Issue Tracker](https://github.com/biotool-paper/biotool/issues)

# Authors

Alphabetically:

* Jessica Chung
* Harriet Dashnow
* Peter Georgeson
* Andrew Lonsdale
* Bernie Pope
* David R Powell
* Torsten Seemann
