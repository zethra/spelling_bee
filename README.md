# NY Times Spelling Bee Solver

A little program to find solutions the the
NY Times Spelling Bee puzzles.

## Usage

spelling_bee <words_file> <letters>

The words file is the dictionary the program references.  
It should contain a list of world separated by new lines.

The letters list should have the required letter first
and look like "gelmnou".  The order of the rest of the
letters doesn't matter.

## Building

`cargo build --release`
