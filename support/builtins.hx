
((:attr builtin-function add$d8) :fn (add (a i8) (b i8)) i8)
((:attr builtin-function add$d16) :fn (add (a i16) (b i16)) i16)
((:attr builtin-function add$d32) :fn (add (a i32) (b i32)) i32)
((:attr builtin-function add$d64) :fn (add (a i64) (b i64)) i64)
((:attr builtin-function add$d8) :fn (add (a u8) (b u8)) u8)
((:attr builtin-function add$d16) :fn (add (a u16) (b u16)) u16)
((:attr builtin-function add$d32) :fn (add (a u32) (b u32)) u32)
((:attr builtin-function add$d64) :fn (add (a u64) (b u64)) u64)


((:attr builtin-function sub$d32) :fn (sub (a i32) (b i32)) i32)

((:attr builtin-function lt$d32) :fn (lt (a i32) (b i32)) bool)

((:attr builtin-function smul$d32) :fn (mul (a i32) (b i32)) i32)

(:fn (println (a u32)) void)