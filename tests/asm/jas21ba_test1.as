       lw  0 1 ten	  load reg1 with 10 (uses symbolic address)
       lw  0 2 thirty load reg2 with 30
       lw  1 3 -1	    load reg3 with 5 (uses numeric address)
start  add 1 3 1	    increment reg1 by 5
       beq 2 1 1	    goto end of program when reg1==30
	     beq 0 0 start	go back to the beginning of the loop
       sw  0 1 ten    overwrite ten's given address with 30
done   halt           end of program
ten    .fill 10
five   .fill 5
thirty .fill 30
stAddr .fill start    will contain the address of start (2)
