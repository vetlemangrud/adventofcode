main = do
  interact $ (++ "\n") . show .  part2 . (map read) . lines

part1 :: [Int] -> Int
part1 xs = length . filter (== True). zipWith (<) xs $ tail xs

part2 :: [Int] -> Int
part2 xs = part1 $ zipWith3 (\a b c -> a + b + c)  xs  (tail xs) (tail $ tail xs)
