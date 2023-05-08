// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// Put your code here.
  @8192
  D=A
  @SCREEN
  D=D+A
  @last_word
  M=D               // last_word = SCREEN + 8192
  @kbd_val
  M=0               // kbd_val =  0 
  @state
  M=0               // state = 0

(CHECK)
  @KBD
  D=M
  @kbd_val
  D=M-D
  @CHECK
  D;JEQ             // if the KBD is at the same value as before, keep checking and make no screen updates
  
(UPDATE_STATE)
  @state
  M=!M              // update state
  @KBD
  D=M
  @kbd_val
  M=D               // update new key_val

(SET_SCREEN)
  @SCREEN
  D=A
  @cursor
  M=D               // cursor = SCREEN,  the stating cursor value

(SET_WORD)
  @state
  D=M
  @cursor
  A=M
  M=D               // update state at R[cursor]
  @cursor
  M=M+1
  D=M               // increment cursor
  @last_word
  D=D-M
  @CHECK
  D;JGE             // return to checking the KBD if all words in screen memory map have been updated
  @SET_WORD
  0;JMP             // keep updating screen mamory map words if the cursor isn't at the end
