<p align="center">
 <h1 align="center">DFA2REG</h1>

  <p align="center">
   Convert NFA/DFA's into regular expressions using the GNFA algorithm with steps!
  </p>
</p>

## About The Project
Inspiration came from working on a problem set and seeing how useful it would to get a step by step guide on how the GNFA algorithm works.

## Usage
As of now this is just the backend so you can input your transition table and get the regex as well as steps that led to that point.
1. First clone the repo into your local machine
2. cd into the project and then into the server directory
3. The project already comes with a couple of example input that you can try there
4. If you want to create your own transition table you can follow the structure line by line (alphabet, states, intial, accepting states, transitions) and create a new file in the input directory.
5. Finally run `cargo run -- <filename>.txt`

## Future Plans
Create a website that streamlines this process so user do not have to interact with the terminal.

