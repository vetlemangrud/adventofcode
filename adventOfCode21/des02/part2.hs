main = interact $ (++ "\n") . show . part2 . lines


part2 :: [String] -> Int
part2 xs = (\(a, b, c) -> a * b) $ getCoords $ map ((\[a,b] -> (a, read b)) . words ) $ reverse xs

getCoords :: [(String, Int)] -> (Int, Int, Int)
getCoords [] = (0,0,0)
getCoords (("forward", n):xs) = (\(x, y, a) -> (x + n, y + (n * a), a)) $ getCoords xs
getCoords (("up" , n):xs) = (\(x, y, a) -> (x, y, a - n)) $ getCoords xs
getCoords (("down", n):xs) = (\(x, y, a) -> (x, y, a + n)) $ getCoords xs
