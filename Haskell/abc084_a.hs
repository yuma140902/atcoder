main :: IO ()
main = do
  m <- read <$> getLine
  print $ 24 - m + 24
