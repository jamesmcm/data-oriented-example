# data-oriented-example

Experimenting with [Data Oriented Design](https://en.wikipedia.org/wiki/Data-oriented_design) in Rust.

Done: 

* Benchmark of [Array of Structs vs. Struct of Arrays](https://en.wikipedia.org/wiki/AoS_and_SoA)

SoA gets unrolled by compiler, results in ~50% speed up. 
Godbolt: https://godbolt.org/z/d8bjMb

* Benchmark of pattern matching inside hot loop vs. outside

Outside results in 50% speed up.

* Benchmark of Linked List iteration vs. contiguous Vector

Vector shows consistent 90% speed up vs. Linked List iteration

* Benchmark of dynamic dispatch vs. monomorphisation

Dynamic dispatch impedes vectorisation due to indirection, otherwise
cost of lookup in vtable is small
