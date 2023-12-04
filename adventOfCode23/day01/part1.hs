import qualified Data.Char
main = do
  input <- readFile "input"
  putStrLn $ part1 input

part1 :: String -> String
part1 input = show $ sum $ map calib $ lines input
  where digits = filter Data.Char.isDigit
        calib s = read [head $ digits s, last $ digits s]

