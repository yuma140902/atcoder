main :: IO ()
main = do
  n <- readLn :: IO Int
  a <- readLn :: IO Int
  putStrLn $ if (n `mod` 500) <= a then "Yes" else "No"
