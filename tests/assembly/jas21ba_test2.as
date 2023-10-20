       lw   0 1 five	 load reg1 with five (uses symbolic address)
       lw   0 2 negOne load reg2 with -1
       lw   0 4 res    load reg4 with 16346 (the result of the NAND)
start  nand 1 2 3	     performs reg3 = !(reg1 & reg2) [compare it to the number in reg4 to verify]
       add  1 2 5      5 + (-1) = 4 
done   halt               end of program
five   .fill 5
negOne .fill -1
res    .fill -6       result of 'nand 1 2 3'
