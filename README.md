## Rust HW#2: Chomp Cli Program
**Authored by: Gustavo Carrillo Castro**
**Odin: gustav5**
**CS523: Rust Programming**
**11/10/2026**

This is an expansion and completion of the game Chomp using CLI arguments and input against a basic ai outlined in the assignment repo, [Chomp](https://en.wikipedia.org/wiki/Chomp). Chomp is a game where the top left element of a grid is poison, and players take turns selecting an element thereafter removing everything below and to the right of that square (including the original square). The player to eat the poisoned square is the loser.

#### Build
The program should build fine using rust stable `cargo build`.
To run the program use `cargo run -- row col`, where row and col are valid positive numbers, 1-4 for row, and 1-5 for col. Although the program should fail and explain usage if there are any errors.

Tests can be run with `cargo test` for the Chomp lib code.

Testing the CLI game can be aided by looking at our `sample-game.txt`. Very similar to the original repos suggested layout for the game.

### Testing Process
While developing the Chomp lib code I relied on the initial 2 tests given to verify the simple winning move cases and invalid init resulting in an error. As helper functions were built out it was useful to include tests to verify fallback game move logic and the game move function "ate" the expected squares. Things were also tested manually as I went along and set up the main program loop as things I did not foresee/test with code popped up. My winning move logic was correct for most tests, but a small control flow error meant the ai always favored `1 1` as its first move. I did not catch this until I had to play against the ai and noticed the pattern.
### Issues
They're less so issues more so a byproduct of finishing things, but I had to add 4 helper functions to the lib code. `apply_move_changes, print_board, fallback_move, and poison_eaten`functions to ergonomically design the code.

It was interesting handling control flow, in comparison to lower division courses and cli programs. Building even this simple game was a nice experience in seeing how Rust forces you to handle your edge cases, whereas C and C++ would let you shoot yourself in the foot if you assume only happy path results and don't check return types. For concrete examples, it was new to me to consider when it was best to `match` results vs `if let` or `let {}`to appropriately handle different scenarios. 


# AI Acknowledgement
Google search ai results were helpful when looking for what features /syntax is available in rust, granted what it suggested was often corrected by clippy.
Gemini Chat interface was used to aid in doc comment creation for some of the helper functions, and to generate some tests.