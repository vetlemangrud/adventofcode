import Data.Char (isDigit, digitToInt)
size = 140

data Cell = Empty | Symbol Char | Digit Char deriving (Eq, Show)

parseMap :: String -> [Cell]
parseMap [] = []
parseMap (x:xs)
  | x == '.'    = Empty:parseMap xs
  | isDigit x   = Digit x:parseMap xs
  | otherwise   = Symbol x:parseMap xs

isInside :: (Int, Int) -> Bool
isInside (y,x) =  y >= 0 || y < size || x >= 0 || x < size

getCell :: [Cell] -> (Int, Int) -> Cell
getCell m (y,x) 
  | not $ isInside (y,x)  = Empty
  | otherwise           = m !! (y * size + x)


isSymbol :: Cell -> Bool
isSymbol (Symbol _) = True
isSymbol _ = False

isDigitCell :: Cell -> Bool
isDigitCell (Digit _) = True
isDigitCell _ = False

getNeighbors :: (Int, Int) -> [(Int, Int)]
getNeighbors (y, x) = [(ny, nx) | 
  ny <- [y-1..y+1],
  nx <- [x-1..x+1],
  isInside (ny, nx),
  (ny, nx) /= (y,x)]

-- Will return max one cell per number
getNeighborsWithDigit :: [Cell] -> (Int, Int) -> [(Int, Int)]
getNeighborsWithDigit m (y,x) = [(dy, dx) |
  (dy, dx) <- neighbors,
  isDigitCell $ getCell m (dy,dx)]
  where neighbors = getNeighbors (y,x)
