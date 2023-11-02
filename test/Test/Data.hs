module Test.Data where

import Test.Data.RelType (relTypeTests)
import Test.Tasty (TestTree, testGroup)

dataTests :: TestTree
dataTests = testGroup "Data" [relTypeTests]
