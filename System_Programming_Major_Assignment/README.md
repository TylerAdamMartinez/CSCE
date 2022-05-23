# CSCE3600 Project 1 (Major 1)

### PROGRAM DESCRIPTION
In this introductory assignment, you will write a complete C program that will implement 
a set of operations supported by a menu that will prompt for the operation (“Count Leading 
Zeros”,  “Endian  Swap”,  “Rotate  Right”,  and  “Parity”)  along  with  one  integer  between  1 
and  4294967295,  inclusively,  and  then  perform  that  operation  on  the  input  integer  to 
produce the result. Note: The “Rotate Right” operation will also take a second input integer 
between  0  and  31.  The  twist  is  that  each  operation  must  be  performed  using  bitwise 
operators rather than the traditional operations.
<br>
This project will be organized in a header file called major1.h, a source code file called 
major1.c, which will contain the main() function, and four individual source code files 
called  clz.c,  endian.c,  rotate.c,  and  parity.c  that  will  contain  the  function 
definition for their respective operations. The entire group will be responsible in general 
for  non-operation  specific  functionality  in  the  major1.h and  major1.c source  code 
files  while  each  individual  group  member  is  responsible  for  his/her  own  “function 
operation”  in  the  appropriate  files  (including  the  header  file  and  inside  the  main() 
function).
<br>
In particular, you are expected to have the following functionality for each file: 
<br>
#### `major1.h`
This is the overall header file for the project that will contain any  include directives 
and  function  prototypes.  While  the  include  directives are general  for  the  team, 
each  member  is  expected  to  add  their  own  function  prototype  for  the  operation 
he/she is responsible for. 
<br>
#### `major1.c`
This is the code file with the main() function that will do the following: (1) display 
the menu, (2) read in the user’s response for the menu selection, (3) prompt for 
and  read  an  integer  operand  between  1  and  4294967295,  inclusively,  (4)  if  the 
“Rotate Right” operation is selected, then prompt for a second integer between 0 
and  31  (inclusively)  to  obtain  the  rotate  amount,  and  finally,  based  on  the  menu 
selection, (5) call the appropriate function call for the specified operation, passing
2 of 5
<br>
the  integer  operand(s)  as  parameter(s)  to  the  function.  This  functionality  will  be 
contained in a loop that will continue to iterate until the user selects the option to 
end  the  program.  If  the  user  enters  a  valid  outside  of  the  1  –  5  range,  you  will 
print a meaningful error message and re-display the menu. Additionally, you will 
continue  to  prompt  for  and  read  in  the  integer  operand  until  the  user  enters  an 
appropriate value (no error message is needed here). While the menu  and integer 
operand is general for the team, each member is expected to add the function call 
for the operation he/she is responsible for.
<br>
#### `clz.c`
This code will contain a single function (and the include directive to your header 
file) to count the number of leading zeroes in the 32-bit input argument passed to 
the  function  (leading  zeroes  are  the  zeroes  that  occur  in  the  most  significant  bit 
positions of the number until a bit value of ‘1’ is found).
One team member, and only one team member, will be responsible for  the source 
code  in  this  file,  though  collaboration  with  other  team  members  may  be  done  if 
needed.
<br>
#### `endian.c`
This code will contain a single function (and the include directive to your header 
file) to perform the endian swap operation of the 32-bit input argument passed to 
this function. Endianness refers to how the bytes are stored in memory. In a 32- 
bit word, there are 4 bytes – B0, B1, B2, and B3. Let us assume that B0 refers to 
the least significant byte and B3 the most significant byte.
The function will swap (exchange) bytes B0 and B3, as well as bytes B1 and B2.
One team member, and only one team member, will be responsible for  the source 
code  in  this  file,  though  collaboration  with  other  team  members  may  be  done  if 
needed.
<br>
#### `rotate.c`
This code will contain a single function (and the include directive to your header 
file) to perform the rotation operation using the two integer operands passed to this 
function. The first integer operand is the number to be rotated. The second integer 
operand is the number of bit positions that the first operand must be rotated by.
One team member, and only one team member, will be responsible for  the source 
code  in  this  file,  though  collaboration  with  other  team  members  may  be  done  if 
needed.
<br>
#### `parity.c`
This code will contain a single function (and the include directive to your header 
file) to perform the parity computation on the input integer passed to this function. 
The output of the function is 0 if the input has even parity (that is, the number of 1s 
in the binary representation of the input is even) and 1 if the input has odd parity 
