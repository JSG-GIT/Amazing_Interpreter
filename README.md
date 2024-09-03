# Windesheim Glade Interpreter

This is an interpreter for test running the sort-of pseudo-code used for the a-mazing challenge of the introduction week of year 2 HBO-ICT of University of Applied Sciences Windesheim. Included is example code in `instructions.txt` and an example glade in `glade.csv` to run it against.

## csv syntax

| letter | object         | number                                          |
| ------ | -------------- | ----------------------------------------------- |
| O      | obstacle       | type of obstacle
| B      | bomb           | seconds (steps) till explosions                 |
| D      | target (doel)  | number of target                                |
| E      | money (bonus)  | 2 ^ x is the bonus gained                       |
| R      | turner (draai) | 1-3 times turning to the left, 0 is random turn |
| S      | start          | 0-3 is direction, clock-wise with 0 being north |
| C      | kleur          | 0-8 is colour, see more below                   |

## code syntax

Uses the syntax (including bugs) of "taal 20", a simple language made for the a-mazing challenge for the introduction week of year 2 HBO-ICT of University of Applied Sciences Windesheim. One addition is the `print` statement which can take a variable or expression and debug print it.

## notes

- compiling requires rust nightly
- csv file can be passed by using `-g <filepath>` and defaults to `glade.csv`
- code file can be passed by using `-c <filepath>` and defaults to `instructions.txt`
- needs to be ran from the command line in the directory with the csv and txt file

## colours
8 = white
7 = grey
6 = red
5 = orange
4 = yellow
3 = green
2 = blue
1 = purple
0 = black