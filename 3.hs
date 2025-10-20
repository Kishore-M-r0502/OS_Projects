class Operatable a where
    add :: a -> a -> a
    sub :: a -> a -> a
    mul :: a -> a -> a
    div :: a -> a -> a

instance Operatable 