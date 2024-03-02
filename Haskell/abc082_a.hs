main :: IO ()
main = do
  line <- getLine
  let [a, b] = map read (words line)
  print $ (a + b + 1) `div` 2
  pure ()
