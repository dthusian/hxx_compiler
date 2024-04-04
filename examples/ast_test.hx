
(:fn (factorial (x i32)) i32 (
  (:let (product i32) 1)
  (:while (add 2 2) (
    (:if (lt x 1) (
      (:break)
    ))
    (:set product (mul product x))
    (:set x (sub x 1))
  ))
))

(:fn (main (argc u64) (argv *u8)) i32 (
  (:let (x u32) 1)
  (println x)
  (:if 1 (
    (:let (x u32) 2)
    (println x)
    (:let (x u32) 3)
    (println x)
  ))
  (:let (x u32) 4)
  (:while x (
    (:set x 0)
    (:let (x u32) 5)
    (println x)
  ))
))