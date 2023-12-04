{-# LANGUAGE OverloadedStrings #-}
import Data.Text (splitOn, Text, unpack, pack)
main = do
  input <- readFile "input"
  putStrLn $ show $ part2 $ map pack $ lines input

addToCount :: (Integer, Integer, Integer) -> Text -> (Integer, Integer, Integer)
addToCount (r,g,b) next
  | nc == "red" = (r + na, g, b)
  | nc == "green" = (r, g + na, b)
  | nc == "blue" = (r, g, b + na)
  | otherwise = (r, g, b)
  where
    splt = splitOn " " next
    nc = splt !! 1
    na = read $ unpack $ splt !! 0

getAmount :: Text -> (Integer, Integer, Integer)
getAmount input = foldl addToCount (0,0,0) $ splitOn ", " input

getAmounts :: Text -> [(Integer, Integer, Integer)]
getAmounts input = map getAmount $ splitOn "; " input

getMaxAmount :: [(Integer, Integer, Integer)] -> (Integer, Integer, Integer)
getMaxAmount [] = (0,0,0)
getMaxAmount ((r,g,b):xs) = (max r nr, max g ng, max b nb)
  where (nr, ng, nb) = getMaxAmount xs

amountPossible :: (Integer, Integer, Integer) -> Bool
amountPossible (r,g,b) = and [r <= 12, g <= 13, b <= 14]

part1 :: [Text] -> Int
part1 input = 
  sum $ 
  map (\l -> read $ drop 5 $ unpack (l !! 0) ) $ 
  filter (\l -> amountPossible $ getMaxAmount $ getAmounts (l !! 1) ) $ 
  map (splitOn ": ") input

part2 :: [Text] -> Integer
part2 input =
  sum $
  map (\(r, g, b) -> r *g *b) $
  map (\l -> getMaxAmount $ getAmounts (l !! 1)) $
  map (splitOn ": ") input
