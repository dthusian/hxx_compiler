# Intermediate Representation the Third

> Revision 1

IR3 is the SSA form intermediate program representation that `hxx_compiler` uses to optimize code
and prepare it for codegen.

IR3 is similar to existing compiler IRs such as LLVM IR, Cranelift's CLIF, and SPIR-V as used in Vulkan and OpenGL.

## Basic Structure

The basic unit of IR3 is a function. Functions are composed of blocks, which must end
in a `br*` instruction or `ret` instruction. Since branches are not allowed
in the middle of a block, blocks are guaranteed to be purely sequential.

## Data Model

Data in IR3 is represented in `d<n>` types, where the `d` stands for "data", and `n` is the length in
bits. `n` is only allowed to be 1, 8, 16, 32, 64, or 128. Signed numbers are represented with twos-complement.

Pointers are represented with the `ptr` type, and carry provenance.
Pointer-integer casts (in either direction) are currently forbidden in IR3.

## Instructions

All targets must support basic instructions, but extended instructions can be emulated in IR.

### Notation

- `<x: T>` defines a mandatory parameter called `x` that is a `T`. 
  - `block` refers to a block.
  - `const` refers to an integer constant.
  - `d<n>` refers to a variable that is data of length `n`.
  - `ptr` refers to any pointer variable.
  - All other values mean `x` is a variable of a type given by type parameters.
- `<T: type>` creates a type parameter called `T`.
- `<T: type is data>` creates a type parameter `T` with a type constraint that says it must be data.
- `*` is a type operator that multiplies the length of a data by a constant.

### Control Flow

- `br <block: block>` - Jumps to `<block>`.
- `br_if <block: block> <cond: d1>` - Branches to `<block>` if `<cond>` is not zero.
- `ret <T: type> <var: T>` - Returns from the function.
- `phi <T: type> <var1: T> <block1: block> [<var2: T> <block2: block>] ... -> <res: T>` - SSA Phi node. Merges an arbitrary number of
  variables from varying control flow paths.

### Data Manipulation

- `const <T: type is data> <const: const> -> <res: T>` Loads a constant into a register.
- `zext <T: type is data> <U: type is data> <x: T> -> <res: U>` Zero-extends an integer.
- `sext <T: type is data> <U: type is data> <x: T> -> <res: U>` Sign-extends an integer.

### Arithmetic and Bitwise

Based on RISC-V's "I" instruction set, as it defines a common set of operations supported
by essentially everyone.

- `add <T: type is data> <a: T> <b: T> -> <res: T>` - Addition.
- `sub <T: type is data> <a: T> <b: T> -> <res: T>` - Subtraction.
- `cmp_<mode> <T: type> <a: T> <b: T> -> <res: d1>` - Arithmetic compare.
  `<mode>` is one of `ult`, `ugt`, `ule`, `uge`, `slt`, `sgt`, `sle`, `sge`, `eq`, `ne`. Outputs 1 if true, 0 otherwise.
- `and <T: type is data> <a: T> <b: T> -> <res: T>` - Bitwise AND.
- `or <T: type is data> <a: T> <b: T> -> <res: T>` - Bitwise OR.
- `xor <T: type is data> <a: T> <b: T> -> <res: T>` - Bitwise XOR.
- `not <T: type is data> <a: T> <b: T> -> <res: T>` - Bitwise NOT.
- `sll <T: type is data> <a: T> <b: const> -> <res: T>` - Logical left shift. Shifts in zeroes.
- `srl <T: type is data> <a: T> <b: const> -> <res: T>` - Logical right shift. Shifts in zeroes.
- `sra <T: type is data> <a: T> <b: const> -> <res: T>` - Arithmetic right shift. Shifts in copies of the most significant bit.

### Memory Access

- `ptr_load <T: type> <ptr: ptr> -> <res: T>` - Load a value of `<type>` from `<ptr>`.
- `ptr_store <T: type> <ptr: ptr> <data: T>` - Store a value of `<type>` to `<ptr>`.
- `ptr_sadd <T: type is data> <ptr: ptr> <off: T> -> <res: ptr>` - Add a signed value to a `<ptr>`.
- `ptr_uadd <T: type is data> <ptr: ptr> <off: T>` - Add an unsigned value to a `<ptr>`.
- `alloc <T: type is data> <sz: T> -> <res: ptr>`
- `free <ptr: ptr>`

### Extended Instructions 1

Most targets will support these intrinsics. They will be emulated
on unsupported targets.

- `smul <T: type is data> <a: T> <b: T> -> <res: T * 2>` - Signed multiply.
- `umul <T: type is data> <a: T> <b: T> -> <res: T * 2>` - Unsigned multiply.
- `sdiv <T: type is data> <a: T> <b: T> -> <res: T>` - Signed divide.
- `udiv <T: type is data> <a: T> <b: T> -> <res: T>` - Unsigned divide.
