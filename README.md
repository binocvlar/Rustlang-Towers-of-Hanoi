# Rustlang-Towers-of-Hanoi
Implementation of "Towers of Hanoi" game solver, in Rustlang

## Purpose
The purpose of this program is twofold:

1. As a first project using [Rust-lang](https://www.rust-lang.org/en-US/)
2. As a final project for [edX/Harvard CS50](https://www.edx.org/course/cs50s-introduction-computer-science-harvardx-cs50x)

## Here be dragons...
As this is my first proper project in rust, I'm probably making a lot of mistakes here.

Several times throughout this project, I found myself unable to do things in my usual (Pythonic)
manner. Being constrained by the borrow checker (for my own safety :)) has lead to some sections of
the code being less efficient and/or less readable than I would have liked.

## Future improvements

Given that we know that completion time is on the order of (2**n - 1), I could take a user issued
sleep time, and provide an estimated time to completion. Perhaps even a count-down timer and/or
progress bar?

## Author
Richard P <binocvlar@gmail.com>
