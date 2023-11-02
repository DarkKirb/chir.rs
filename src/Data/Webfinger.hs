module Data.Webfinger where

import Codec.CBOR.Class (Serialise (..))
import Codec.CBOR.Decoding (TokenType (TypeString), decodeString, peekTokenType)
import Codec.CBOR.Encoding (Encoding)
import Data.Aeson (FromJSON (parseJSON), ToJSON (toJSON))
import Data.RelType (RelType)
import Network.URI (URI, parseURI)

data RelTypeOrUri = Rel RelType | Uri URI
  deriving stock (Show, Generic)

instance Serialise RelTypeOrUri where
  encode (Rel relType) = encode relType
  encode (Uri uri) = (encode :: Text -> Encoding) $ show uri

  decode = do
    token <- peekTokenType
    case token of
      TypeString -> do
        value <- toString <$> decodeString
        case parseURI value of
          Nothing -> fail ("invalid URI: " <> value)
          Just v -> pure $ Uri v
      _ -> Rel <$> decode

instance ToJSON RelTypeOrUri where
  toJSON (Rel relType) = toJSON relType
  toJSON (Uri uri) = toJSON (show uri :: Text)

instance FromJSON RelTypeOrUri where
  parseJSON v =
    let parseRelType = Rel <$> parseJSON v
        parseUri = do
          res <- parseJSON v
          case parseURI res of
            Just val -> pure $ Uri val
            Nothing -> fail ("invalid URI: " <> show v)
     in parseRelType <|> parseUri
