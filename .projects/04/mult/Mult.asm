// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)

// if R0 == 0 jump to zero
@R0
D=M
@ZERO
D;JEQ

// if R1 == 0 jump to zero
@R1
D=M
@ZERO
D;JEQ

// R2 = R0
@R0
D=M
@R2
M=D

// i = R1 - 1
@R1
D=M
@i
M=D-1

(LOOP)
  // if i == 0 jump to end
  @i
  D=M
  @END
  D;JEQ

  // R2 = R2 + R0
  @R2
  D=M
  @R0
  D=D+M
  @R2
  M=D

  // i = i - 1
  @i
  D=M
  D=D-1
  M=D

  // jump back to the loop
  @LOOP
  0;JMP

(ZERO)
  // if some of the operands are zero, R2 = 0
  @R2
  M=0
(END)
  @END
  0;JMP
