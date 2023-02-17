-- category-theory-for-programmers challenge 5.8.5 - 5.8.8
module ProCoPro where
-- data Either a b = Left a | Right b
factorizer :: (a -> c) -> (b -> c) -> Either a b -> c
factorizer i j (Left a) = i a
factorizer i j (Right b) = j b

i:: Int -> Int
i = id
j:: Bool -> Int
j b = if b then 0 else 1

-- so Either is "better" than Int equipped with i, j
m:: Either Int Bool -> Int
m = factorizer i j