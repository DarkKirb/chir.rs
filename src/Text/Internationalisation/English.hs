{-# OPTIONS_GHC -Wno-overlapping-patterns #-}

module Text.Internationalisation.English where

import Text.Internationalisation.Types (
  Message (..),
  TranslationError,
  html,
  text,
  untranslated,
 )
import Yesod (Html, WidgetFor)

translate :: Message -> WidgetFor a (Either TranslationError Html)
translate MsgAkkoma = text "Akkoma"
translate MsgAlternateLink = text "(alt)"
translate MsgAndALotMore = text "and a lot more…"
translate MsgAuthorize = text "Authorize"
translate MsgAutomatic = text "Automatic"
translate MsgForgejo = text "Forgejo"
translate MsgGithub = text "Github"
translate MsgGPGKeyNote = text "(GPG key)"
translate MsgFooterText = text "Made with coffee in Haskell."
translate MsgFootnoteBridgedMatrix = text "Bridged to Matrix."
translate MsgImgSammyTheTanukiBabyLottePFP = text "Portrait of Charlotte, an orange raccoon with pink face and purple mask. It is wearing a purple bandanna around its neck and a purple pacifier with a pink heart on it."
translate MsgImgSammyTheTanukiBabyLottePFPFigcaption = html "Artwork by <a href=\"https://furaffinity.net/user/sammythetanuki\" hreflang=\"en\">Sammy the Tanuki</a>"
translate MsgJumpToContent = text "Jump to content…"
translate MsgMyDiscord = text "Discord (darkkirb)"
translate MsgMyEmail = text "Email (lotte@chir.rs)"
translate MsgMyFediverse = text "Fediverse (@charlotte@akko.chir.rs)"
translate MsgMyForgejo = text "My Forgejo instance"
translate MsgMyKeybase = text "Keybase (darkkirb)"
translate MsgMyMatrixChat = text "Matrix (@lotte:chir.rs)"
translate MsgMySignalWhatsapp = text "Signal/Whatsapp: available upon request"
translate MsgMyTelegram = text "Telegram (darkkirb)"
translate MsgNextcloud = text "Nextcloud"
translate MsgPageBlog = text "Blog"
translate MsgPageChat = text "Chat"
translate MsgPageHome = text "Homepage"
translate MsgPageSubtitle = text "Raccoons? On the internet? It’s more likely than you think!"
translate MsgPageTitle = text "Lotte’s Attic"
translate MsgPeertube = text "Peertube"
translate MsgReportedSightings = text "Reported Sightings"
translate MsgReportedSightingsSubtitle = text "Ordered by report credibility."
translate MsgSiteIndex = text "Index"
translate MsgSelfIntroPart1 = html "Hi! I’m <span class=\"p-name\">Charlotte</span>, or just <span class=\"p-nick\">Lotte</span>, a <abbr class=\"dt-bday\" title=\"2001-01-10\">22 year old</abbr> <span class=\"p-species\">raccoon</span> therian and <span class=\"p-role\">student</span> from <span class=\"p-country-name\">Germany</span>."
translate MsgSelfIntroPart2 = text "I do system administration, software development, and occasionally hardware design."
translate MsgSelfIntroPart3 = text "I love coffee, lasagna, pizza, raccoons, and plushies."
translate MsgSelfIntroPart4 = html "My pronouns are <span class=\"p-pronoun\">it/its</span>, <span class=\"p-pronoun\">rac/racs/racself</span>, <span class=\"p-pronoun\">🦝/🦝s/🦝self</span>, and <span class=\"p-pronoun\">plush/plushs/plushself</span>."
translate MsgSettingLanguage = text "Language:"
translate MsgSettingsCookieNotice = text "We use cookies to store and transmit changed settings."
translate MsgSettingsMenu = text "Settings"
translate MsgSettingTheme = text "Theme:"
translate MsgUsername = text "Username:"
translate MsgSettingSunsetTheme = text "Sunset (dark theme)"
translate MsgSettingTransRightsTheme = text "Trans Rights (light theme)"
translate MsgSettingBlackTheme = text "Black (high contrast dark theme)"
translate MsgSettingWhiteTheme = text "White (high contrast light theme)"
translate _ = untranslated
