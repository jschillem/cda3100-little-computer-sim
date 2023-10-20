        lw   0 1 one	  fibbonaci sequence in LC3100 assembly
        lw   0 5 one    reg6 holds the current iteration of the counter, reg6 here helps increment it
        lw   0 7 n      holds the current n of fib(n) for the fibbonaci sequence
start   add  1 2 3      starting with fib(2), fib(0) and fib(1) are already shown in reg1 & reg2
        add  0 2 1      move the previously calculated value into reg1
        add  0 3 2      move the newly calculated value into reg2
        add  5 6 6      increment the iteration counter
        beq  6 7 1      if on final iteration -> store and halt
	      beq  0 0 start	go back to the beginning of the loop
        sw   0 2 final  store the final number in memory     
done    halt            end of program
one    .fill 1
n      .fill 15         the number n of the fibonacci sequence that we are seeking
final  .fill 0          memory address to store my final fibonacci number once the program is finished
stAddr .fill start      will contain the address of start (2)
