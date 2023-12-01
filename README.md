# csc411_assignment5

Sam Calise and Claudia Deverdits

We have received help from Ayman when trying to resolve bit pack errors. 

We believe we have correctly implemented all aspects of the program. 

### Architecture
Our implementation is similar to what was described in our design document, with the main difference being an added module for segments. 

Our implementation has 5 modules (excluding bit pack), which are main.rs, register.rs, rum.rs, segment.rs, and um_instruction.rs.

The main.rs handles collecting the instructions and executing the corresponding opcodes until there are no more instructions left. 

The register.rs holds the functionality for creating a register and updating or getting the register values.

The rum.rs module executes all opcodes and holds the functions for each within a rum impl. 

The segment.rs hold functionality for mapping and unmapping segments and also updating and setting segment values. 

The um_instruction.rs module processes the instructions to get the register values and organize the instructions into their opcodes. 
<hr>
It takes our UM approximately 8 seconds to execute 50 million instructions. We know this because we created a timer and a counter for the number of processed instructions so we know when we hit 50 million. 

<hr>
We have spent approximately 2 hours analyzing, 2 hours preparing the design, and 25 hours solving the problem after analysis.

