var searchIndex = {};
searchIndex['brainfuck'] = {"items":[[0,"","brainfuck","Simple brainfuck interpreter in Rust.",null,null],[3,"Interpreter","","A brainfuck interpreter, with the needed state for execution.",null,null],[3,"Program","","The logic desired to be run by the brainfuck interpreter.",null,null],[3,"Tape","","A fixed length data structure for holding bytes and a pointer.",null,null],[4,"Error","","A general error type for problems inside of the interpreter.",null,null],[13,"Io","","Errors with reading or writing to IO.",0,null],[13,"NoProgram","","No program loaded.",0,null],[13,"Overflow","","Overflows.",0,null],[13,"CycleLimit","","Interpreter cycle limit hit.",0,null],[4,"Instruction","","An executable instruction in the language.",null,null],[13,"IncPtr","","Increment the pointer moving it up on the tape.\nTODO: Document wrapping/error behavior.",1,null],[13,"DecPtr","","Decrement the pointer moving it down on the tape.\nTODO: Document wrapping/error behavior.",1,null],[13,"IncVal","","Increment the value at the pointer on the tape.\nTODO: Document wrapping/error behavior.",1,null],[13,"DecVal","","Decrement the value at the pointer on the tape.\nTODO: Document wrapping/error behavior.",1,null],[13,"Output","","Write the value at the pointer as a `char` to `STDOUT`. This\ninstruction can fail if writing to the underlying writer fails.",1,null],[13,"Input","","Read from `STDIN` as a `char` to value at the pointer. This\ninstruction can fail if reading from the underlying reader\nfails or has no more data.",1,null],[13,"SkipForward","","Skip forward if the value at the pointer is `0`. For more\ninformation see the section on control flow above.",1,null],[13,"SkipBackward","","Skip backward if the value at the pointer is **not** `0`.\nFor more information see the section on control flow above.",1,null],[5,"eval","","Run the given program with STDIN and STDOUT as the IO buffers.",null,{"inputs":[{"name":"program"}],"output":{"name":"result"}}],[5,"eval_string","","Parse a program from the given string and `eval` it.",null,{"inputs":[{"name":"str"}],"output":{"name":"result"}}],[5,"eval_file","","Parse a program from the given file path and `eval` it.",null,{"inputs":[{"name":"p"}],"output":{"name":"result"}}],[11,"fmt","","",0,{"inputs":[{"name":"error"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"description","","",0,{"inputs":[{"name":"error"}],"output":{"name":"str"}}],[11,"fmt","","",0,{"inputs":[{"name":"error"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from","","",0,{"inputs":[{"name":"error"},{"name":"error"}],"output":{"name":"error"}}],[11,"new","","Return a new interpreter, without a program or IO.",2,{"inputs":[{"name":"interpreter"}],"output":{"name":"interpreter"}}],[11,"load","","Load a program for the interpreter to run.",2,{"inputs":[{"name":"interpreter"},{"name":"program"}],"output":{"name":"self"}}],[11,"read_from","","Use the given reader for the `Input` instruction.",2,{"inputs":[{"name":"interpreter"},{"name":"r"}],"output":{"name":"self"}}],[11,"write_to","","Use the given writer for the `Output` instruction.",2,{"inputs":[{"name":"interpreter"},{"name":"w"}],"output":{"name":"self"}}],[11,"run","","Run the interpreter.",2,{"inputs":[{"name":"interpreter"}],"output":{"name":"result"}}],[11,"run_with_callback","","Run the interpreter with a callback hook.",2,{"inputs":[{"name":"interpreter"},{"name":"f"}],"output":{"name":"result"}}],[11,"hash","","",1,null],[11,"eq","","",1,{"inputs":[{"name":"instruction"},{"name":"instruction"}],"output":{"name":"bool"}}],[11,"ne","","",1,{"inputs":[{"name":"instruction"},{"name":"instruction"}],"output":{"name":"bool"}}],[11,"fmt","","",1,{"inputs":[{"name":"instruction"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",1,{"inputs":[{"name":"instruction"}],"output":{"name":"instruction"}}],[11,"fmt","","",1,{"inputs":[{"name":"instruction"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",3,{"inputs":[{"name":"program"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"parse","","Create a program from source text.",3,{"inputs":[{"name":"program"},{"name":"str"}],"output":{"name":"program"}}],[11,"get","","Get the instruction at the given program counter.",3,{"inputs":[{"name":"program"},{"name":"usize"}],"output":{"name":"option"}}],[11,"from_file","","Create a program from a file.",3,{"inputs":[{"name":"program"},{"name":"p"}],"output":{"name":"result"}}],[11,"fmt","","",3,{"inputs":[{"name":"program"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"new","","Return a new tape with all values set to 0, and the pointer\nat the first cell.",4,{"inputs":[{"name":"tape"}],"output":{"name":"tape"}}],[11,"deref","","",4,{"inputs":[{"name":"tape"}],"output":{"name":"target"}}],[11,"deref_mut","","",4,{"inputs":[{"name":"tape"}],"output":{"name":"u8"}}],[11,"add_assign","","",4,{"inputs":[{"name":"tape"},{"name":"u8"}],"output":null}],[11,"sub_assign","","",4,{"inputs":[{"name":"tape"},{"name":"u8"}],"output":null}],[11,"shr_assign","","",4,{"inputs":[{"name":"tape"},{"name":"usize"}],"output":null}],[11,"shl_assign","","",4,{"inputs":[{"name":"tape"},{"name":"usize"}],"output":null}],[17,"CYCLE_LIMIT","","The number of instructions allowed to execute before the interpreter\nerrors with `Error::CycleLimit`.",null,null],[17,"TAPE_LENGTH","","The number of cells the tape contains. Attempts to access above or\nbelow this limit will result in an error.",null,null]],"paths":[[4,"Error"],[4,"Instruction"],[3,"Interpreter"],[3,"Program"],[3,"Tape"]]};
initSearch(searchIndex);