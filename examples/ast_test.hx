
(:fn (factorial (x i32) (unused i32)) i32 (
  (:let (product i32) 1)
  (:while (add 2 2) (
    (:if (lt x 1) (
      (:break)
    ))
    (:set product (mul product x))
    (:set x (sub x 1))
  ))
))

(:fn (main (x i32) (unused i32)) i32 (
  (:let (product i32) 1)
  (:while (add 2 2) (
    (:if (lt x 1) (
      (:break)
    ))
    (:set product (mul product x))
    (:set x (sub x 1))
  ))
))