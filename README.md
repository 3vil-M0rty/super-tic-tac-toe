# Super Tic Tac Toe in Rust

Welcome to the Super Tic Tac Toe project in Rust! This is a fun and challenging variation of the classic Tic Tac Toe game that adds an exciting twist. In this game, you'll be playing on a larger 3x3 grid, where each cell contains a 3x3 Tic Tac Toe "Miniboard." The rules are designed to keep you engaged and strategizing throughout the game.

## Rules

1. The game is played on a 3x3 grid, with each cell containing a 3x3 Tic Tac Toe Miniboard. This creates a total of 81 individual cells to play in.

2. Players take turns placing their symbols, either X or O, in an empty cell within the Miniboards.

3. The cell you choose determines the Miniboard where your opponent must make their move. For example, if you play in the top-right cell of a Miniboard, your opponent must then play in the top-right Miniboard.

4. To win a Miniboard, you need to get three of your symbols in a row – horizontally, vertically, or diagonally, just like in classic Tic Tac Toe.

5. If a Miniboard has already been won by a player or is completely filled (resulting in a draw), you can choose to play in any other Miniboard that still has empty cells.

6. The game continues until a player wins three Miniboards in a row or until the entire 3x3 grid is filled.

7. The player who achieves the win condition first – either three Miniboards in a row or a filled entire board – is declared the ultimate winner.

## Getting Started

To run the Super Tic Tac Toe game, follow these steps:

1. Make sure you have Rust installed on your system. You can download and install Rust from [here](https://www.rust-lang.org/tools/install).

2. Clone this repository to your local machine using the following command: ```git clone <repository-url>```

3. Navigate to the project directory:

4. Compile and run the game using the Rust compiler: ```cargo run```

5. Follow the on-screen instructions to start playing the game. Use the coordinates to select the cell where you want to place your symbol. Look at the demo below:

![demo](/demo.gif)

## Contribution

If you'd like to contribute to the project, feel free to fork this repository, make your changes, and submit a pull request. We welcome any improvements or additional features that you'd like to add.

Have fun playing Super Tic Tac Toe in Rust, and may the best strategist win!

