module Test.Data.RelType where

import Codec.CBOR.Class (deserialise, serialise)
import Data.Aeson (decode, encode)
import Data.RelType (RelType)
import Generic.Random (genericArbitrary, uniform)
import Test.Tasty (TestTree, testGroup)
import Test.Tasty.QuickCheck (Arbitrary (arbitrary), testProperty)

relTypeTests :: TestTree
relTypeTests =
  testGroup
    "RelType"
    [ testProperty "CBOR serialize" $ \d -> deserialise (serialise (d :: RelType)) == Right d
    , testProperty "JSON serialize" $ \d -> decode (encode (d :: RelType)) == Just d
    ]

instance Arbitrary RelType where
  arbitrary = genericArbitrary uniform
