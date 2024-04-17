# Worderer 
*[Word] + [Orderer] = Worderer*

This is a Rust rewrite of my [Kotlin Worderer program](https://github.com/Onkornix/kotlin_worderer)

### What does it do?

1. Reads a plain text file (.txt) line by line
2. Counts how many times each word occurs
3. Groups words together based on how many times they occur
4. Writes the numerically ordered word groups to another text file
5. **HANDLES ERRORS BECAUSE RUST IS AWESOME!**

### Plans!
**Interactive mode:** call functions to view data related to the ordered words real time
  - Find:  prints the amount of times a word occurs (if at all)
  - Words: prints all the word groups that occur *n* times to the terminal
  - Index: prints all the word groups between occurrence values *a* and *b* (descending)
  - Input: runs core functionality on a new file and updates the map without exiting interactive mode
  - Write: writes numerically ordered words to output file without exiting interactive mode

**UI**
  - I'm thinking about creating a TUI where the user can browse and select files using arrow keys or
    searching instead of typing the path to the file.
  - I think since rust is better for terminal apps than kotlin is, I may be able to accomplish this.
    It will be very difficult though.

