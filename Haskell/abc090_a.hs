main = getContents >>= putStrLn . \s -> [s !! 0, s !! 5, s !! 10]
