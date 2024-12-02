import qualified Data.List
import qualified Data.Char
main = interact $ show . part1 . lines

part1 :: [String] -> Int
part1 xs = let 
  transposed = map (map Data.Char.digitToInt) $ Data.List.transpose xs 
  len = length xs
  sums = map sum transposed
  most_frequent_bin = map (\s -> if s * 2 > len then True else False) sums
  gamma = foldl (\y x -> fromEnum x + y) 0 most_frequent_bin

  epsilon = foldl (\y x -> fromEnum x + y*2) 0 $ (map not) most_frequent_bin
  in
    gamma * epsilon
  

