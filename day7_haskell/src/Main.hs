module Main where

import Data.Maybe (fromMaybe)
import Data.HashMap.Strict (HashMap)
import qualified Data.HashMap.Strict as HM

type SizeMap = HashMap [String] Integer

sizeCalculator :: ([String], SizeMap) -> [String] -> ([String], SizeMap)
sizeCalculator (pwd, dirs) ["$", "cd", ".."] = (tail pwd, dirs)
sizeCalculator (pwd, dirs) ["$", "cd", dir] = (dir:pwd, dirs)
sizeCalculator (pwd, dirs) ["$", "ls"] = (pwd, dirs)
sizeCalculator (pwd, dirs) ["dir", _] = (pwd, dirs)
sizeCalculator (pwd, dirs) [size, _] = (pwd, addToAllDirs dirs pwd)
    where
        addToAllDirs ds ["/"] = HM.insert ["/"] (newSizeFor ["/"] ds) ds
        addToAllDirs ds (p:pwd) = addToAllDirs (HM.insert (p:pwd) (newSizeFor (p:pwd) ds) ds) pwd
        newSizeFor pwd ds = (fromMaybe 0 (HM.lookup pwd ds) + (read size))

main :: IO ()
main = do
    input <- readFile "input.txt" 
    let allLines = map words $ lines input
    let (_, directories) = foldl sizeCalculator ([], HM.empty) allLines 
    let part1 = sum $ HM.filter (\v -> v <= 100000) directories
    putStrLn $ show part1
    let neededSpace = 70000000 - 30000000
    let currentUsed = fromMaybe 0 $ HM.lookup ["/"] directories
    let part2 = minimum $ HM.filter (\v -> v >= currentUsed - neededSpace) directories
    putStrLn $ show part2
