# Inductive Program Synthesizer

This project utilizes inductive programming techniques to generate a function only using a set of input/output examples. The resulting function will satisfy all provided examples; however, there may be multiple possible functions that satisfy the example set. Thus increasing the breadth of examples will decrease the set of possible solution.

## Implementation (Prospective)

The synthesizer is written in Rust. A specified Domain Specific Language (DSL) is used to generate the initially synthesized function. The DSL can then be translated into various programming.