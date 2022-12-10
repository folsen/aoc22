module Main where

buildRegisterList :: [[String]] -> [(Integer, Integer, Integer)]
buildRegisterList instructions = foldl walkInstruction [(0,1,0)] instructions
    where
        walkInstruction ((s, x, r):xs) ["noop"] = (s+1, x+r, 0):(s, x, r):xs
        walkInstruction ((s, x, r):xs) ["addx", v] = (s+2, x+r, read v):(s+1, x+r, 0):(s, x, r):xs
        walkInstruction xs _ = xs

buildDisplay :: [(Integer, Integer, Integer)] -> String
buildDisplay cycles = fst $ foldl getPixel ("",0) cycles
    where
        getPixel (str, idx) (_, x, _) = (addChar str canSeePixel idx, newIdx)
            where
                canSeePixel = idx == x - 1 || idx == x || idx == x + 1 
                newIdx = if idx == 39 then 0 else idx + 1
        addChar str b idx =
            if idx == 39
            then str ++ addedStr ++ "\n"
            else str ++ addedStr
            where addedStr = if b then "#" else "."


main :: IO ()
main = do
    input <- readFile "input.txt" 
    let instructions = map words $ lines input
    let registerList = buildRegisterList instructions

    --- Part 1 ---
    let signalPoints = [20, 60, 100, 140, 180, 220]
    let signals = filter (\(s,_,_) -> s `elem` signalPoints) registerList
    let part1 = sum $ map (\(s, x, _) -> s * x) signals
    putStrLn $ show part1
    --- Part 2 ---
    let displayString = buildDisplay (drop 1 $ reverse registerList)
    putStrLn displayString
