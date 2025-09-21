linearsearch :: Eq a => a -> [a] -> Bool
linearsearch _ [] = False
linearsearch x (y:ys)
    | x == y    = True
    | otherwise = linearsearch x ys
main :: IO ()
main = do
    print $ linearsearch 3 [1..5] 
    print $ linearsearch 6 [1..10]

binarysearch :: Ord a => a -> [a] -> Bool
binarysearch _ [] = False
binarysearch x xs
    | x == mid  = True
    | x < mid   = binarysearch x left
    | otherwise = binarysearch x right
  where
    midIndex = length xs `div` 2
    mid = xs !! midIndex
    left = take midIndex xs
    right = drop (midIndex + 1) xs 

quickSort :: Ord a => [a] -> [a]
quickSort [] = []
quickSort (p:xs) = quickSort [x | x <- xs, x <= p] ++ [p] ++ quickSort [x | x <- xs, x > p]
main2 :: IO ()
main2 = do
    print $ binarysearch 3 [1..5] 
    print $ binarysearch 6 [1..10]
    print $ quickSort [3,6,8,10,1,2,1]