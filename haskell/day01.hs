import Data.List

calculateIncreases :: (Ord a, Num p) => [a] -> p
calculateIncreases [] = 0
calculateIncreases [_] = 0
calculateIncreases (x:y:xs) = computed + calculateIncreases (y:xs)
                            where
                                computed
                                    | x < y     = 1
                                    | otherwise = 0

runOnFile :: (Show b) => (a -> b) -> (String -> a) -> String -> IO ()
runOnFile computationFn parser path = do
    contents <- readFile path
    print (computationFn $ parser contents)

parseLinesToType :: Read a => String -> [a]
parseLinesToType text = map read $ lines text

findAnswerDay1_2021 = runOnFile calculateIncreases parseLinesToInt "input_day1_puzzle1.txt"
                    where parseLinesToInt = parseLinesToType :: (String -> [Int])