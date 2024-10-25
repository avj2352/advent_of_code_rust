# Notes on Day 01

## Difference between `include_str!` and `fs::read_to_string()`

```rust

// Using fs::read_to_string()
import std::fs;
// ....
let file = fs::read_to_string("./src/day_01/input_01.txt").unwrap(); 

// Using include_str!
let file = include_str!("./src/day_01/input_01.txt"); 

```

*include_str!*

- Compile-Time Inclusion: This macro embeds the entire contents of the specified file directly into your program's binary at compile time.
- Result: It produces a &'static str (a string slice with a static lifetime), meaning the data is available throughout your program's execution.
- Performance: Extremely fast at runtime because the data is already loaded into memory. No file system access is needed.
- Use Cases:
    - Configuration files
    - Small to medium-sized static datasets
    - Templates (HTML, SQL, etc.)
- Limitations:
    - Not suitable for large files (can significantly increase binary size).
    - Cannot be used for files that change frequently (requires recompilation).

*fs::read_to_string()*

- Runtime File Reading: This function reads the contents of a file from the file system at runtime.
- Result: It returns a Result<String, std::io::Error>, meaning you need to handle potential errors (e.g., file not found).
- Performance: Slower than include_str! because it involves disk I/O.
- Use Cases:
    - Reading user-provided files
    - Working with data that changes dynamically
    - Handling large files that shouldn't be embedded in the binary
    - Flexibility: Allows you to work with files in various ways (reading, writing, appending, etc.).



You want to easily switch between different input files.
The input files might be large.
You're sharing your code, and others might not have the input files in the same location.
Could You Use include_str!?

Yes, you could potentially use include_str! if:

Your input files are relatively small.
You don't anticipate needing to change the input files frequently.

---

## Part 02 solved using Nom parser - `trebuchet_nom.rs`

This code defines a parser using the nom crate in Rust to extract numbers from a string, including those written as words ("one", "two", etc.). Here's a breakdown:

#### 1. numbers function:

Purpose: This function tries to parse a single number from the input string. **It prioritizes spelled-out numbers over single-digit numerals.**

How it works:
- It first attempts to match the input against a list of spelled-out numbers ("one" to "nine") using `alt` (which tries each parser in order) and value (which returns a constant value if the parser succeeds). If a match is found, it returns the corresponding digit (1-9).
- If no spelled-out number is found, it consumes a single character using `anychar` and attempts to convert it to a digit using `to_digit(10)`.
- It returns the parsed digit (if any) and the remaining unparsed input.

#### 2. parser function:

Purpose: This function parses the entire input string, extracting all the numbers it finds.

How it works:

- It uses the `iterator` combinator to repeatedly apply the numbers function to the input string.
- The `flatten` method is used to convert the iterator of `Option<u32>` (because `numbers` might not find a digit on every iteration) into an iterator of u32.
- The `collect` method gathers all the parsed digits into a `Vec<u32>`.
- Finally, it returns the vector of parsed digits and the remaining unparsed input (which should be empty if the parsing was successful).

#### 3. process_line_part_02_nom function:

Purpose: This function processes a single line of input, extracting the first and last digits and combining them into a two-digit number.

How it works:

- It calls the `parser` function to extract all the digits from the input line.
- It retrieves the first and last digits from the resulting vector.
- If only one digit is found, it's used for both the first and last digits.
- It combines the first and last digits into a two-digit number and returns it.

#### 4. process_part_02_nom function:

Purpose: This function processes the entire input, applying the `process_line_part_02_nom` function to each line and summing the results.

How it works:

- It splits the input into lines.
- It maps each line using process_line_part_02_nom to extract the two-digit number.
- It sums all the extracted numbers.
- It converts the sum to a string and returns it.
- In essence, this code defines a parser that can recognize both numerical and textual representations of numbers within a string and uses it to extract specific information from each line of an input.

---