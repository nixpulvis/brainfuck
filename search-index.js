var searchIndex = {};
searchIndex['brainfuck'] = {"items":[[0,"","brainfuck","Simple brainfuck interpreter in Rust.",null,null],[3,"Interpreter","","A brainfuck interpreter, with the needed state for execution.",null,null],[4,"Error","","A general error type for problems inside of the interpreter.",null,null],[13,"Io","","Errors with reading or writing to IO.",0,null],[4,"Instruction","","An executable instruction in the language.",null,null],[13,"IncPtr","","Increment the pointer moving it up on the tape.\nTODO: Document wrapping/error behavior.",1,null],[13,"DecPtr","","Decrement the pointer moving it down on the tape.\nTODO: Document wrapping/error behavior.",1,null],[13,"IncVal","","Increment the value at the pointer on the tape.\nTODO: Document wrapping/error behavior.",1,null],[13,"DecVal","","Decrement the value at the pointer on the tape.\nTODO: Document wrapping/error behavior.",1,null],[13,"Output","","Write the value at the pointer as a `char` to `STDOUT`. This\ninstruction can fail if writing to the underlying writer fails.",1,null],[13,"Input","","Read from `STDIN` as a `char` to value at the pointer. This\ninstruction can fail if reading from the underlying reader\nfails or has no more data.",1,null],[13,"SkipForward","","Skip forward if the value at the pointer is `0`. For more\ninformation see the section on control flow above.\nTODO: Skips should be statically guaranteed not to fail.",1,null],[13,"SkipBackward","","Skip backward if the value at the pointer is **not** `0`.\nFor more information see the section on control flow above.\nTODO: Skips should be statically guaranteed not to fail.",1,null],[11,"fmt","","",0,{"inputs":[{"name":"error"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"description","","",0,{"inputs":[{"name":"error"}],"output":{"name":"str"}}],[11,"fmt","","",0,{"inputs":[{"name":"error"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from","","",0,{"inputs":[{"name":"error"},{"name":"error"}],"output":{"name":"error"}}],[11,"eq","","",1,{"inputs":[{"name":"instruction"},{"name":"instruction"}],"output":{"name":"bool"}}],[11,"ne","","",1,{"inputs":[{"name":"instruction"},{"name":"instruction"}],"output":{"name":"bool"}}],[11,"fmt","","",1,{"inputs":[{"name":"instruction"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"new","","Return the instruction corrisponding to the given instruction.",1,{"inputs":[{"name":"instruction"},{"name":"char"}],"output":{"name":"option"}}],[11,"execute","","Given an interpreter to execute on, perform the action\ncorrisponding to this instruction.",1,{"inputs":[{"name":"instruction"},{"name":"interpreter"}],"output":null}],[11,"fmt","","",1,{"inputs":[{"name":"instruction"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"new","","Return a new interpreter with the given code, reader, and writter.",2,{"inputs":[{"name":"interpreter"},{"name":"c"},{"name":"r"},{"name":"w"}],"output":{"name":"result"}}],[11,"from_file","","Create a new interpreter from a file.",2,{"inputs":[{"name":"interpreter"},{"name":"p"},{"name":"r"},{"name":"w"}],"output":{"name":"result"}}],[11,"run","","Run the interpreter.",2,{"inputs":[{"name":"interpreter"}],"output":null}],[11,"run_with_callback","","Run the interpreter with a callback hook.",2,{"inputs":[{"name":"interpreter"},{"name":"f"}],"output":null}],[11,"step","","Step the interpreter one instruction.",2,{"inputs":[{"name":"interpreter"}],"output":{"name":"option"}}]],"paths":[[4,"Error"],[4,"Instruction"],[3,"Interpreter"]]};
initSearch(searchIndex);
