main = interact $ show . part1 . lines

part1 :: [String] -> Int
part1 xs = (\(a, b) -> a * b) $ getCoords $ map ((\[a,b] -> (a, read b)) . words ) xs

getCoords :: [(String, Int)] -> (Int, Int)
getCoords [] = (0,0)
getCoords (("forward", n):xs) = (\(x, y) -> (x + n, y)) $ getCoords xs
getCoords (("up" , n):xs) = (\(x, y) -> (x, y - n)) $ getCoords xs
getCoords (("down", n):xs) = (\(x, y) -> (x, y + n)) $ getCoords xs
