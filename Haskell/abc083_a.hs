main :: IO ()
main = do
  [a, b, c, d] <- map read . words <$> getLine
  putStrLn $
    case compare (a + b) (c + d) of
      GT -> "Left"
      EQ -> "Balanced"
      LT -> "Right"
  pure ()
