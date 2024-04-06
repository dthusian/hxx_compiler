
((:attr builtin-function add$i8) :fn (add (a i8) (b i8)) i8)
((:attr builtin-function add$i16) :fn (add (a i16) (b i16)) i16)
((:attr builtin-function add$i32) :fn (add (a i32) (b i32)) i32)
((:attr builtin-function add$i64) :fn (add (a i64) (b i64)) i64)
((:attr builtin-function add$u8) :fn (add (a u8) (b u8)) u8)
((:attr builtin-function add$u16) :fn (add (a u16) (b u16)) u16)
((:attr builtin-function add$u32) :fn (add (a u32) (b u32)) u32)
((:attr builtin-function add$u64) :fn (add (a u64) (b u64)) u64)


((:attr builtin-function sub$i32) :fn (sub (a i32) (b i32)) i32)

((:attr builtin-function lt$i32) :fn (lt (a i32) (b i32)) bool)

((:attr builtin-function mul$i32) :fn (mul (a i32) (b i32)) i32)

(:fn (println (a u32)) void)