module Handler.StyleFiles where

import Codec.CBOR.Class (Serialise (..))
import Codec.CBOR.Encoding (encodeMapLen, encodeString, encodeWord)
import Config.StaticFiles (style_base_css, style_black_css, style_sunset_css, style_trans_rights_css, style_white_css)
import Data.Aeson (defaultOptions)
import Data.Aeson.TH (deriveJSON)
import Foundation (App, Route (StaticR), returnJSON')
import Yesod (HandlerFor, RedirectUrl (toTextUrl), TypedContent)

data StyleFiles = StyleFiles
  { baseStyle :: Text
  , sunsetTheme :: Text
  , transRightsTheme :: Text
  , blackTheme :: Text
  , whiteTheme :: Text
  }
  deriving stock (Show, Generic)

instance Serialise StyleFiles where
  encode styles =
    encodeMapLen 5
      <> encodeWord 0
      <> encodeString (baseStyle styles)
      <> encodeWord 1
      <> encodeString (sunsetTheme styles)
      <> encodeWord 2
      <> encodeString (transRightsTheme styles)
      <> encodeWord 3
      <> encodeString (blackTheme styles)
      <> encodeWord 4
      <> encodeString (whiteTheme styles)

  -- next index is 9

  decode = fail "Decoding not supported"

$(deriveJSON defaultOptions ''StyleFiles)

getStyleFilesR :: HandlerFor App TypedContent
getStyleFilesR = do
  bs <- toTextUrl $ StaticR style_base_css
  st <- toTextUrl $ StaticR style_sunset_css
  trt <- toTextUrl $ StaticR style_trans_rights_css
  bt <- toTextUrl $ StaticR style_black_css
  wt <- toTextUrl $ StaticR style_white_css
  returnJSON' $
    StyleFiles
      { baseStyle = bs
      , sunsetTheme = st
      , transRightsTheme = trt
      , blackTheme = bt
      , whiteTheme = wt
      }
