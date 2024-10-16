{-# OPTIONS_GHC -Wno-overlapping-patterns #-}

module Text.Internationalisation.EnglishSteno where

import Data.Char (toLower)
import Text.Internationalisation.English (TextOrHtml (..), translate')
import Text.Internationalisation.Types (
  Message (..),
  TranslationError,
  html,
  text,
 )
import Yesod (Html, WidgetFor)

skipElements :: String -> [String]
skipElements ('>' : r) = ">" : conv r
skipElements (c : r) = [c] : skipElements r
skipElements [] = []

-- Matches a prefix, case insensitively
m :: String -> String -> Maybe String
m prefix result
  | and (zipWith (\a b -> a == toLower b) prefix result) = Just (drop (length prefix) result)
  | otherwise = Nothing

-- Matches a prefix with an optional space character appended
ms :: String -> String -> Maybe String
ms prefix result = case m prefix result of
  Just (' ' : r) -> Just r
  rest -> rest

conv :: String -> [String]
conv [] = []
conv ('<' : r) = "<" : skipElements r
conv (ms "charlotte" -> Just r) = "KHAR/HROT/TE/" : conv r
conv (ms "lotte" -> Just r) = "HROT/TE/" : conv r
conv (ms "’s" -> Just r) = "AES/" : conv r
conv (ms "attic" -> Just r) = "A/TEUBG/" : conv r
conv (ms "raccoons" -> Just r) = "RA/KAOPB/-Z/" : conv r
conv (ms "raccoon" -> Just r) = "RA/KAOPB/" : conv r
conv (ms "?" -> Just r) = "H-F/" : conv r
conv (ms "on the" -> Just r) = "O*PBT/" : conv r
conv (ms "and a" -> Just r) = "SKPAEU/" : conv r
conv (ms "more" -> Just r) = "PHOR/" : conv r
conv (ms "…" -> Just r) = "SKW-BGS/" : conv r
conv (ms "authorize" -> Just r) = "THORZ/" : conv r
conv (ms "automatic" -> Just r) = "AUT/PHABGT/" : conv r
conv (ms "forgejo" -> Just r) = "TPORPBLG/SP-S/" : conv ('j' : 'o' : r)
conv (ms "github" -> Just r) = "TKPWHUB/" : conv r
conv (ms "keybase" -> Just r) = "KAOE/PWAEUS/" : conv r
conv (ms "key" -> Just r) = "KAOE/" : conv r
conv (ms "made" -> Just r) = "PHAED/" : conv r
conv (ms "with" -> Just r) = "W/" : conv r
conv (ms "coffee" -> Just r) = "KOEF/" : conv r
conv (ms "haskell" -> Just r) = "HAS/KEL/" : conv r
conv (ms "bridged" -> Just r) = "PWREUPBLGD/" : conv r
conv (ms "matrix" -> Just r) = "PHAERBGS/" : conv r
conv (ms "portrait" -> Just r) = "POR/TRAEUT/" : conv r
conv (ms "orange" -> Just r) = "ORPBG/" : conv r
conv (ms "pink" -> Just r) = "P*EUPBG/" : conv r
conv (ms "face" -> Just r) = "TPAEUS/" : conv r
conv (ms "purple" -> Just r) = "PURPL/" : conv r
conv (ms "mask" -> Just r) = "PHAFBG/" : conv r
conv (ms "it is" -> Just r) = "T-S/" : conv r
conv (ms "wearing" -> Just r) = "WAERG/" : conv r
conv (ms "bandanna" -> Just r) = "PWAPB/TKA/TPHA/" : conv r
conv (ms "pacifier" -> Just r) = "PAS/TPAOEUR/" : conv r
conv (ms "heart" -> Just r) = "HART/" : conv r
conv (ms "artwork" -> Just r) = "ART/WO*RBG/" : conv r
conv (ms "sammy" -> Just r) = "SAPL/SP-S/PHAOEU/" : conv r
conv (ms "tanuki" -> Just r) = "TA/TPHU/KI/" : conv r
conv (ms "jump" -> Just r) = "SKWR*UPL/" : conv r
conv (ms "content" -> Just r) = "KAUPBT/" : conv r
conv (ms "discord" -> Just r) = "STKORD/" : conv r
conv (ms "email" -> Just r) = "AOE/PHAEUL/" : conv r
conv (ms "fediverse" -> Just r) = "TPE/TKEU/SRERS/" : conv r
conv (ms "signal" -> Just r) = "STPHAL/" : conv r
conv (ms "whatsapp" -> Just r) = "WHATS/AP/" : conv r
conv (ms "available" -> Just r) = "SRAEUBL/" : conv r
conv (ms "upon" -> Just r) = "POPB/" : conv r
conv (ms "request" -> Just r) = "KW/" : conv r
conv (ms "telegram" -> Just r) = "T*EL/TKPWRAPL/" : conv r
conv (ms "nextcloud" -> Just r) = "TPHEGT/KHRO*UD/" : conv r
conv (ms "blog" -> Just r) = "PWHROG/" : conv r
conv (ms "chat" -> Just r) = "KHAT/" : conv r
conv (ms "homepage" -> Just r) = "HOEPL/PA*EUPBLG/" : conv r
conv (ms "peertube" -> Just r) = "PAOER/SP-S/TAOUB/" : conv r
conv (ms "reported" -> Just r) = "RORPD/" : conv r
conv (ms "sightings" -> Just r) = "SAOEUGTS/" : conv r
conv (ms "ordered" -> Just r) = "ORD/-D/" : conv r
conv (ms "report" -> Just r) = "RORP/" : conv r
conv (ms "credibility" -> Just r) = "KRED/-BLT/" : conv r
conv (ms "index" -> Just r) = "EUPB/TKEBGS/" : conv r
conv (ms "i’m" -> Just r) = "AOEUPL/" : conv r
conv (ms "just" -> Just r) = "SKWRUFT/" : conv r
conv (ms "year" -> Just r) = "KWRAO*ER/" : conv r
conv (ms "old" -> Just r) = "OELD/" : conv r
conv (ms "therian" -> Just r) = "THER/KWRAN/" : conv r
conv (ms "student" -> Just r) = "STAOUPBT/" : conv r
conv (ms "from" -> Just r) = "TPR/" : conv r
conv (ms "germany" -> Just r) = "SKWRERPL/SP-S/TPHEU/" : conv r
conv (ms "system" -> Just r) = "S-PL/" : conv r
conv (ms "administration" -> Just r) = "STKRAEUGS/" : conv r
conv (ms "software" -> Just r) = "SWA*EUR/" : conv r
conv (ms "occasionally" -> Just r) = "KWAELGS/" : conv r
conv (ms "hardware" -> Just r) = "HARD/WA*EUR/" : conv r
conv (ms "design" -> Just r) = "STKPAOEUPB/" : conv r
conv (ms "love" -> Just r) = "HROF/" : conv r
conv (ms "lasagna" -> Just r) = "HRA/SAPB/KWRA/" : conv r
conv (ms "pizza" -> Just r) = "PEUZ/" : conv r
conv (ms "plushies" -> Just r) = "PHRURBZ/" : conv r
conv (ms "pronouns" -> Just r) = "PROE/TPHOUPBZ/" : conv r
conv (ms "language" -> Just r) = "HRAPBLG/" : conv r
conv (ms "cookies" -> Just r) = "KAO/KAOEZ/" : conv r
conv (ms "store" -> Just r) = "STOR/" : conv r
conv (ms "transmit" -> Just r) = "TREUT/" : conv r
conv (ms "changed" -> Just r) = "KHAEUPBGD/" : conv r
conv (ms "settings" -> Just r) = "SEGTS/" : conv r
conv (ms "theme" -> Just r) = "THAOEPL/" : conv r
conv (ms "trans" -> Just r) = "TRA*PBS/" : conv r
conv (ms "rights" -> Just r) = "RAOEUTS/" : conv r
conv (ms "light" -> Just r) = "HRAOEUT/" : conv r
conv (ms "sunset" -> Just r) = "SUPB/S*ET/" : conv r
conv (ms "dark" -> Just r) = "TKARBG/" : conv r
conv (ms "black" -> Just r) = "PWHRABG/" : conv r
conv (ms "white" -> Just r) = "WHAOEUT/" : conv r
conv (ms "black" -> Just r) = "PWHRABG/" : conv r
conv (ms "high" -> Just r) = "HAO*EU/" : conv r
conv (ms "contrast" -> Just r) = "SKRAFT/" : conv r
-- small words go down here
conv (ms "and" -> Just r) = "SKP/" : conv r
conv (ms "an" -> Just r) = "APB/" : conv r
conv (ms "alt" -> Just r) = "ALT/" : conv r
conv (ms "lot" -> Just r) = "HROT/" : conv r
conv (m "a " -> Just r) = "AEU/" : conv r
conv (ms "on" -> Just r) = "OPB/" : conv r
conv (ms "its" -> Just r) = "EUTS/" : conv r
conv (ms "it" -> Just r) = "T/" : conv r
conv (ms "of" -> Just r) = "-F/" : conv r
conv (ms "to" -> Just r) = "TO/" : conv r
conv (ms "in" -> Just r) = "TPH/" : conv r
conv (ms "by" -> Just r) = "PWEU/" : conv r
conv (ms "the" -> Just r) = "-T/" : conv r
conv (ms "do" -> Just r) = "TKO/" : conv r
conv (ms "my" -> Just r) = "PHAOEU/" : conv r
conv (ms "are" -> Just r) = "R/" : conv r
conv (ms "we" -> Just r) = "WAOE/" : conv r
conv (ms "use" -> Just r) = "AOUS/" : conv r
conv (ms "hi" -> Just r) = "HEU/" : conv r
conv ('(' : r) = "PREPB/" : conv r
conv (')' : ' ' : r) = "PRE*PB/" : conv r
conv (')' : r) = "PRE*PB/" : conv r
conv ('.' : ' ' : r) = "TP-PL/" : conv r
conv ('.' : r) = "TP-PL/" : conv r
conv (',' : ' ' : r) = "KW-BG/" : conv r
conv (',' : r) = "KW-BG/" : conv r
conv ('@' : r) = "A*T/" : conv r
conv (':' : ' ' : r) = "STPH-FPLT/" : conv r
conv (':' : r) = "KHR-PB/" : conv r
conv ('/' : r) = "OEU/" : conv r
conv ('!' : ' ' : r) = "TP-BG/" : conv r
-- these come last as these are fingerspelling entries.
conv ('a' : r) = "A*/" : conv r
conv ('b' : r) = "PW*/" : conv r
conv ('c' : r) = "KR*/" : conv r
conv ('d' : r) = "TK*/" : conv r
conv ('e' : r) = "*E/" : conv r
conv ('f' : r) = "TP*/" : conv r
conv ('g' : r) = "TKPW*/" : conv r
conv ('h' : r) = "H*/" : conv r
conv ('i' : r) = "*EU/" : conv r
conv ('j' : r) = "SKWR*/" : conv r
conv ('k' : r) = "K*/" : conv r
conv ('l' : r) = "HR*/" : conv r
conv ('m' : r) = "PH*/" : conv r
conv ('n' : r) = "TPH*/" : conv r
conv ('o' : r) = "O*/" : conv r
conv ('p' : r) = "P*/" : conv r
conv ('q' : r) = "KW*/" : conv r
conv ('r' : r) = "R*/" : conv r
conv ('s' : r) = "S*/" : conv r
conv ('t' : r) = "T*/" : conv r
conv ('u' : r) = "*U/" : conv r
conv ('v' : r) = "SR*/" : conv r
conv ('w' : r) = "W*/" : conv r
conv ('x' : r) = "KP*/" : conv r
conv ('y' : r) = "KWH*/" : conv r
conv ('z' : r) = "STKPW*/" : conv r
conv ('A' : r) = "A*P/" : conv r
conv ('B' : r) = "PW*P/" : conv r
conv ('C' : r) = "KR*P/" : conv r
conv ('D' : r) = "TK*P/" : conv r
conv ('E' : r) = "*EP/" : conv r
conv ('F' : r) = "TP*P/" : conv r
conv ('G' : r) = "TKPW*P/" : conv r
conv ('H' : r) = "H*P/" : conv r
conv ('I' : r) = "*EUP/" : conv r
conv ('J' : r) = "SKWR*P/" : conv r
conv ('K' : r) = "K*P/" : conv r
conv ('L' : r) = "HR*P/" : conv r
conv ('M' : r) = "PH*P/" : conv r
conv ('N' : r) = "TPH*P/" : conv r
conv ('O' : r) = "O*P/" : conv r
conv ('P' : r) = "P*P/" : conv r
conv ('Q' : r) = "KW*P/" : conv r
conv ('R' : r) = "R*P/" : conv r
conv ('S' : r) = "S*P/" : conv r
conv ('T' : r) = "T*P/" : conv r
conv ('U' : r) = "*UP/" : conv r
conv ('V' : r) = "SR*P/" : conv r
conv ('W' : r) = "W*P/" : conv r
conv ('X' : r) = "KP*P/" : conv r
conv ('Y' : r) = "KWH*P/" : conv r
conv ('Z' : r) = "STKPW*P/" : conv r
conv ('1' : '0' : '0' : '0' : r) = "#EUR/" : conv r
conv ('2' : '0' : '0' : '0' : r) = "#EUB/" : conv r
conv ('3' : '0' : '0' : '0' : r) = "#EUG/" : conv r
conv ('4' : '0' : '0' : '0' : r) = "#EUFR/" : conv r
conv ('5' : '0' : '0' : '0' : r) = "#EUPB/" : conv r
conv ('6' : '0' : '0' : '0' : r) = "#EULG/" : conv r
conv ('7' : '0' : '0' : '0' : r) = "#EUF/" : conv r
conv ('8' : '0' : '0' : '0' : r) = "#EUP/" : conv r
conv ('9' : '0' : '0' : '0' : r) = "#EUL/" : conv r
conv ('1' : '0' : '0' : r) = "#UR/" : conv r
conv ('2' : '0' : '0' : r) = "#UB/" : conv r
conv ('3' : '0' : '0' : r) = "#UG/" : conv r
conv ('4' : '0' : '0' : r) = "#UFR/" : conv r
conv ('5' : '0' : '0' : r) = "#UPB/" : conv r
conv ('6' : '0' : '0' : r) = "#ULG/" : conv r
conv ('7' : '0' : '0' : r) = "#UF/" : conv r
conv ('8' : '0' : '0' : r) = "#UP/" : conv r
conv ('9' : '0' : '0' : r) = "#UL/" : conv r
conv ('0' : '0' : '0' : r) = "#EU/" : conv r
conv ('1' : '0' : r) = "#ER/" : conv r
conv ('2' : '0' : r) = "#EB/" : conv r
conv ('3' : '0' : r) = "#EG/" : conv r
conv ('4' : '0' : r) = "#EFR/" : conv r
conv ('5' : '0' : r) = "#EPB/" : conv r
conv ('6' : '0' : r) = "#ELG/" : conv r
conv ('7' : '0' : r) = "#EF/" : conv r
conv ('8' : '0' : r) = "#EP/" : conv r
conv ('9' : '0' : r) = "#EL/" : conv r
conv ('0' : '0' : r) = "#U/" : conv r
conv ('1' : r) = "#-R/" : conv r
conv ('2' : r) = "#-B/" : conv r
conv ('3' : r) = "#-G/" : conv r
conv ('4' : r) = "#-FR/" : conv r
conv ('5' : r) = "#-PB/" : conv r
conv ('6' : r) = "#-LG/" : conv r
conv ('7' : r) = "#-F/" : conv r
conv ('8' : r) = "#-P/" : conv r
conv ('9' : r) = "#-L/" : conv r
conv ('0' : r) = "#E/" : conv r
conv (' ' : r) = "S-P/" : conv r
conv (c : r) = [c] : conv r
conv [] = []

conv' :: String -> String
conv' = concat . conv

translate :: Message -> WidgetFor a (Either TranslationError Html)
translate msg = case translate' msg of
  Left e -> pure $ Left e
  Right (IsText t) -> text $ toText $ conv' $ toString t
  Right (IsHtml t) -> html $ toText $ conv' $ toString t
