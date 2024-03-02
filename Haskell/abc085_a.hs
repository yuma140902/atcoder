main :: IO ()
main = do
  line <- getLine
  putStr $ take 3 line
  putStr "8"
  putStrLn $ drop 4 line
