module Config (ConfigFile (database, databasePoolSize), logLevel', staticDir', toLogLevel, DatabaseConfig (..), toPostgresConf, toSqliteConfInfo, listenPort', loadConfigAuto) where

import Control.Exception (try)
import Control.Lens ((%~))
import Control.Lens.TH (makeLensesFor)
import Control.Monad.Logger (LogLevel (..), MonadLoggerIO, logErrorSH)
import Database.Persist.Postgresql qualified as Postgres
import Database.Persist.Sqlite qualified as Sqlite
import Dhall (FromDhall, auto, input)
import Utils (fallbackAll, tailOrEmpty)

data SqliteConfig = SqliteConfig
  { filename :: Text
  , walEnabled :: Maybe Bool
  , fkEnabled :: Maybe Bool
  , extraPragmas :: Maybe [Text]
  }
  deriving stock (Generic, Show)

instance FromDhall SqliteConfig

toSqliteConfInfo :: SqliteConfig -> Sqlite.SqliteConnectionInfo
toSqliteConfInfo conf =
  Sqlite.mkSqliteConnectionInfo (filename conf)
    & Sqlite.walEnabled %~ (`fromMaybe` walEnabled conf)
    & Sqlite.fkEnabled %~ (`fromMaybe` fkEnabled conf)
    & Sqlite.extraPragmas %~ (`fromMaybe` extraPragmas conf)

data PostgresConfig = PostgresConfig
  { connectionString :: Text
  , poolStripes :: Natural
  , poolIdleTimeout :: Natural
  }
  deriving stock (Generic, Show)

instance FromDhall PostgresConfig

toPostgresConf :: PostgresConfig -> Int -> Postgres.PostgresConf
toPostgresConf conf poolSize =
  Postgres.PostgresConf
    { Postgres.pgConnStr = encodeUtf8 $ connectionString conf
    , Postgres.pgPoolStripes = if poolStripes conf == 0 then poolSize else fromIntegral $ poolStripes conf
    , Postgres.pgPoolIdleTimeout = fromIntegral $ poolIdleTimeout conf
    , Postgres.pgPoolSize = poolSize
    }

data DatabaseConfig = DatabaseSQLite SqliteConfig | DatabasePostgres PostgresConfig
  deriving stock (Generic, Show)

instance FromDhall DatabaseConfig

data LogLevelConfig = LogLevelDebug | LogLevelInfo | LogLevelWarn | LogLevelError | LogLevelOther Text
  deriving stock (Generic, Show)

instance FromDhall LogLevelConfig

toLogLevel :: LogLevelConfig -> LogLevel
toLogLevel LogLevelDebug = LevelDebug
toLogLevel LogLevelInfo = LevelInfo
toLogLevel LogLevelWarn = LevelWarn
toLogLevel LogLevelError = LevelError
toLogLevel (LogLevelOther a) = LevelOther a

data ConfigFile = ConfigFile
  { listenPort :: Word16
  , database :: DatabaseConfig
  , databasePoolSize :: Natural
  , staticDir :: Text
  , logLevel :: LogLevelConfig
  }
  deriving stock (Generic, Show)

makeLensesFor
  [ ("staticDir", "staticDir'")
  , ("logLevel", "logLevel'")
  , ("listenPort", "listenPort'")
  ]
  ''ConfigFile

instance FromDhall ConfigFile

loadConfig :: (MonadLoggerIO m) => Text -> m (Either SomeException ConfigFile)
loadConfig cfg = do
  res <- liftIO $ try $ input auto cfg
  case res of
    Left e -> do
      $(logErrorSH) e
      return $ Left e
    v -> return v

loadConfigAuto :: (MonadLoggerIO m) => m ConfigFile
loadConfigAuto = do
  optionEnv <- lookupEnv "CHIR_RS_CONFIG"
  let configFiles = case optionEnv of
        Just config -> [config]
        _ -> []
  args <- getArgs
  let configFiles' = tailOrEmpty args ++ configFiles
  let configFiles'' = map toText configFiles'
  result <- fallbackAll (loadConfig <$> configFiles'') (liftIO $ try $ fail "Can’t find valid config file")
  case result of
    Right config -> return config
    Left e -> liftIO $ fail $ displayException e