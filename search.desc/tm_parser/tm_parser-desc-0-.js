searchState.loadedDescShard("tm_parser", 0, "Module to import when you want to generate DOT code from …\nModule to import when you want to parse Turing machines …\nModule to use when simulating Turing Machines. This module …\nPrivate utility function to transform a given state into …\nTakes a TM machine (.tm) code and turns it into a .dot …\nList of functions to apply.\nSingle write on both tapes.\nRead symbols struct of a transition object.\nState struct\nStruct for a transition between 2 states.\nStruct representing actions performed by a transition.\nFunction object applied during a transition.\nStruct for a Single write action on a TM tape.\nArguments of the function.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nGet the parsed file We use pest to do the lexing and …\nHead movement in {L,S,R}.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nSymbol read on the main tape;\nName of the state.\nFunction name.\nRead object being the transition condition.\nState where the transition leads to.\nAll transitions going out of the state.\nTurnip V0 parser functions\nTurnip V1 parser functions\nTurnip V2 parser functions\nSymbol read on the work tape;\nTransition effect.\nGamma element to write.\nMain tape write action.\nWork tape write action.\nEnd-of-input\nGrammar file the the V0 of the Turing Machine language\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nTODO: better documentation Surely to many to_string and …\nEnd-of-input\nGrammar file the the V1 of the Turing Machine language …\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nTODO: better documentation Surely to many to_string and …\nEnd-of-input\nGrammar file the the V2 of the Turing Machine language …\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nTODO: better documentation Surely to many to_string and …\nStruct detailing an action done by a TM after having read …\nTODO\nReads the next two letters A and B on the main tape, puts …\nBase action that a TM can do after reading values at its …\nSingle write (on both tapes).\nTODO\nPerforms a bitwise and on the next two letters on the main …\nTODO\nPerforms a bitwise nand on the next two letters on the …\nTODO\nPerforms a bitwise not on the letter currently under the …\nTODO\nPerforms a bitwise not on the letter currently under the …\nTODO\nPerforms a bitwise or on the next two letters on the main …\nTODO\nPerforms a bitwise xor on the next two letters on the main …\nTODO\nSubtracts one to the letter currently under the main tape …\nTODO\nSubtracts one to the letter currently under the work tape …\nTODO\nReads the next two letters A and B on the main tape, puts …\nTODO\nCompares the next two letters A and B on the main tape, if …\nInstead of Base Actions, we can use a list of Functions to …\nList of functions to apply.\nDatatype representing all possible values for a TM tape …\nTODO\nCompares the next two letters A and B on the main tape, if …\nTODO\nCompares the next two letters A and B on the main tape, if …\nTODO\nAdds one to the letter currently under the main tape head\nTODO\nAdds one to the letter currently under the work tape head\nMoving left.\nTODO\nCompares the next two letters A and B on the main tape, if …\nTODO\nCompares the next two letters A and B on the main tape, if …\nMain tape\nTODO\nReads the next two letters A and B on the main tape, puts …\nPossible Turing Machine head movements\nTODO\nReads the next two letters A and B on the main tape, puts …\nMoves the main tape head by a (neg or pos) amount of cells.\nMoves the work tape head by a (neg or pos) amount of cells.\nTODO\nCompares the next two letters A and B on the main tape, if …\nStruct storing the outcome of taking a given transition.\nMoving right.\nTuring Machine simulation object. This object is made to …\nDatatype holding the ID of a TM state.\nStaying still.\nTODO\nReads the next two letters A and B on the main tape, puts …\nTuring Machine object, storing the transition function and …\nStruct containing an edition of a tape cell.\nDatatype holding the position of a tape’s head.\nEnum to easily differentiate tape type.\nStruct used to store Tape edits\nWork tape\nWrites the given letter from Gamma to the main tape at the …\nWrites the given letter from Gamma to the work tape at the …\nRuns the whole Turing Machine for a maxiumum number of …\nTODO: documentation\nTODO: documentation\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nFunction that creates a TM object from a Vector of States …\nReturns the current state ID of the simulated Turing …\nReturns the main tape\nReturns the work tape\nReturns the current position of the main tape’s head of …\nReturns the current position of the work tape’s head of …\nOn which cell of that tape\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nChecks if the Simulation’s TM has reached the END state.\nChecks if the Simulation’s TM has reached the ERROR …\nChecks if the Simulation’s TM has not started yet. NB: …\nSimulation object constructor.\nCell index where the tape’s head has moved to\nNew letter replacing the previous cell content\nRuns a single step (i.e. takes a single transition) of the …\nRewinds the Turing Machine one step back.\nHelper function to access the ID of a state from its name.\nHelper function to access the name of a state from its ID.\nOn which tape the edit has been done\nVerifies that the current main tape has the expected …\nHelper function for tests that enables checking the content")