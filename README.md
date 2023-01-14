# Rust Hangman
### Part 1 of me learning Rust 

Hello! I'm learning Rust, and this is one of the first things I've made to learn it.

Here's a very simple game, but without the gruesome image of hanging a man. *(Totally was not too lazy to make ascii art)*

To play, just put a words.txt file with a word to guess on each line, and then run the program. If you fail to guess a letter 6 times, you lose!


*AdminRAT was here to add Docker support*
`docker build -t hangman .`
`docker run -it --rm -v $PWD/words.txt:/words.txt:ro hangman`
