{-# LANGUAGE OverloadedStrings #-}
import Data.Char (isDigit)
import Data.Text (replace, pack, unpack)
import Text.Read (readMaybe)
import Data.List (isPrefixOf, find)
import Data.Maybe (fromMaybe)
import Control.Applicative( Alternative( (<|>) ) )

main = do
  input <- readFile "input"
  putStrLn $ part2 input

-- part1 :: String -> String
-- part1 input = show $ sum $ map calib $ lines input
--   where digits = filter Data.Char.isDigit
--         calib s = read [head $ digits s, last $ digits s]
--
-- part2 :: String -> String
-- part2 input = part1 $ unpack $ foldr (\(o, n) -> replace (pack o) (pack n)) (pack input) numberstrings
--   where numberstrings = [("one", "1"), ("two", "2"), ("three", "3"), ("four", "4"), ("five", "5"), ("six", "6"), ("seven", "7"), ("eight", "8"), ("nine", "9")]
--

numberstrings = [("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)]

firstDigit :: [(String, Integer)] -> String -> Maybe Integer
firstDigit _ "" = Nothing
firstDigit ns s@(x:xs)
  | isDigit x = readMaybe (x:[])
  | otherwise = (<|>) 
      (fmap snd $ find (flip isPrefixOf s . fst) ns) 
      (firstDigit ns xs)

lastDigit :: [(String, Integer)] -> String -> Maybe Integer
lastDigit ns s = firstDigit (map (\(f,s) -> (reverse f,s)) ns) (reverse s)

calVal :: String -> Integer
calVal s = fromMaybe 0 (firstDigit numberstrings s) * 10 + fromMaybe 0 (lastDigit numberstrings s)


part2 :: String -> String
part2 = show . sum . map calVal . lines
  
