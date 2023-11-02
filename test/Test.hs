module Test where

import Test.Codec (codecTests)
import Test.Data (dataTests)
import Test.Tasty (TestTree, testGroup)

tests :: TestTree
tests = testGroup "Test" [codecTests, dataTests]
