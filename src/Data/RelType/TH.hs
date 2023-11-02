module Data.RelType.TH where

import GHC.Unicode (isUpper, toLower)

modifyTag :: String -> String
modifyTag [] = []
modifyTag (x : xs)
  | isUpper x = '-' : toLower x : modifyTag xs
  | x == '\'' = '.' : modifyTag xs
  | otherwise = x : modifyTag xs
