# Advent of Code 2024

Learning Rust by solving the [Advent of Code 2024](https://adventofcode.com/2024). Expect slightly messy code, I'm 
neither a very experienced Rust programmer, nor will I spend a lot of time polishing the code to perfection.


## Structure

Each day is in a sub-folder `day_xx` which contains a binary that solved the challenge, plus auxiliary code.
The input(s) are not checked in to not spoil any fun ;)

### Generate a new day

There is a [cookiecutter](https://www.cookiecutter.io/) template that can be used to generate a new day. Just to save 
some time.

Run `cookiecutter template/ -o src/` and answer the prompt.

# Log of learnings

The things I learned each challenge:

- Day 1
    - Getting back in the groove of using Rust. I'm not using is regularly as part of work or free time. 
      I have gotten a bit, wait for it, rusty.

- Day 2
    - Took me a good while to realize where my mistake was after I had rewritten the core logic 2 times already. I should
      really read the instructions word-by-word.
    - How to solve it without additional allocations and just iterators and slices. It made the code quiet a lot longer
      then the version the just copied data 

- Day 3
    - I directly saw I could use a Regex for at least part 1. But I really wanted to use a custom parser (by using nom) 
      to solve it, but I could not get it to work. Once I switched to a Regex it was super easy.  
    - Learning: Follow your instinct and don't try to be overly clever?!

- Day 4:
    - Coming up with a working algorithm to generate the necessary verticals really took me a lot of off-by-one errors.
    - I think the concept of lifetimes and references *finally* clicked. At least I had almost no fights with the borrow
      checker despite copies usage of references.