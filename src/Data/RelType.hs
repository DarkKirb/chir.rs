module Data.RelType where

import Codec.CBOR.Class (Serialise)
import Data.Aeson (Options (constructorTagModifier), defaultOptions)
import Data.Aeson.TH (deriveJSON)
import Data.RelType.TH (modifyTag)
import Utils (tailOrEmpty)

-- append new types to the end, don’t reorder or change meaning of fields
data RelType
  = About
  | Acl
  | Alternate
  | Amphtml
  | Appendix
  | AppleTouchIcon
  | AppleTouchStartupImage
  | Archives
  | Author
  | BlockedBy
  | Bookmark
  | Canonical
  | Chapter
  | CiteAs
  | Collection
  | Contents
  | Convertedfrom
  | Copyright
  | CreateForm
  | Current
  | Describedby
  | Describes
  | Disclosure
  | DnsPrefetch
  | Duplicate
  | Edit
  | EditForm
  | EditMedia
  | Enclosure
  | External
  | First
  | Glossary
  | Help
  | Hosts
  | Hub
  | Icon
  | Index
  | Intervalafter
  | Intervalbefore
  | Intervalcontains
  | Intervaldisjoint
  | Intervalduring
  | Intervalequals
  | Intervalfinishedby
  | Intervalfinishes
  | Intervalin
  | Intervalmeets
  | Intervalmetby
  | Intervaloverlappedby
  | Intervaloverlaps
  | Intervalstartedby
  | Intervalstarts
  | Item
  | Last
  | LatestVersion
  | License
  | Linkset
  | Lrdd
  | Manifest
  | MaskIcon
  | Me
  | MediaFeed
  | Memento
  | Micropub
  | Modulepreload
  | Monitor
  | MonitorGroup
  | Next
  | NextArchive
  | Nofollow
  | Noopener
  | Noreferrer
  | Opener
  | Openid2'local_id
  | Openid2'provider
  | Original
  | P3pv1
  | Payment
  | Pingback
  | Preconnect
  | PredecessorVersion
  | Prefetch
  | Preload
  | Prerender
  | Prev
  | PrevArchive
  | Preview
  | Previous
  | PrivacyPolicy
  | Profile
  | Publication
  | Related
  | Replies
  | Restconf
  | Ruleinput
  | Search
  | Section
  | Self
  | Service
  | ServiceDesc
  | ServiceDoc
  | ServiceMeta
  | SipTrunkingCapability
  | Sponsored
  | Start
  | Status
  | Stylesheet
  | Subsection
  | SuccessorVersion
  | Sunset
  | Tag
  | TermsOfService
  | Timegate
  | Timemap
  | Type
  | Ugc
  | Up
  | VersionHistory
  | Via
  | Webmention
  | WorkingCopy
  | WorkingCopyOf
  deriving stock (Show, Generic, Eq)

instance Serialise RelType

$(deriveJSON (defaultOptions {constructorTagModifier = tailOrEmpty . modifyTag}) ''RelType)
