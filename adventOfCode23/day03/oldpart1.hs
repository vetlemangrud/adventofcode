import Data.Char (isDigit, digitToInt)
size = 140

data Cell = Empty | Symbol Char | Digit Char deriving (Eq, Show)

parseMap :: String -> [Cell]
parseMap [] = []
parseMap (x:xs)
  | x == '.'    = Empty:parseMap xs
  | isDigit x   = Digit x:parseMap xs
  | otherwise   = Symbol x:parseMap xs

getCell :: [Cell] -> Int -> Cell
getCell map i 
  | i < 0 || (size * size) <= i  = Empty
  | otherwise           = map !! i

isSymbol :: Cell -> Bool
isSymbol (Symbol _) = True
isSymbol _ = False

getNeighbors :: Int -> [Int]
getNeighbors x = [x, x - size, x + size, x - 1, x + 1, x - size -1, x - size + 1, x + size - 1, x + size + 1]

getSafe :: [Cell] -> Int -> [Int]
getSafe cells 0 = filter (isSymbol . (getCell cells)) [0..(size*size-1)]
getSafe cells n = 
  filter (any (`elem` alreadySafe) . getNeighbors) $ 
  filter ((/=) Empty . (getCell cells)) [0..(size*size-1)]
  where alreadySafe = getSafe cells (n-1)

purgeUseless :: [Cell] -> [Cell]
purgeUseless cells =
  map (\(i,c) -> if i `elem` safe then c else Empty) $
  zip [0..] cells
  where safe = getSafe cells 5

getSum :: [Cell] -> Int -> Int
getSum [] _ = 0
getSum ((Digit d):xs) acc = getSum xs (acc * 10 + digitToInt d)
getSum (_:xs) acc = getSum xs 0 + acc

main = do
  input <- readFile "input"
  putStrLn $ show $ flip getSum 0 $ purgeUseless $ (parseMap input)
