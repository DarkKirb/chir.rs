let SqliteConfig =
      { Type =
          { filename : Text
          , walEnabled : Optional Bool
          , fkEnabled : Optional Bool
          , extraPragmas : Optional (List Text)
          }
      , default =
        { walEnabled = None Bool
        , fkEnabled = None Bool
        , extraPragmas = None (List Text)
        }
      }

let PostgresConfig =
      { Type =
          { connectionString : Text
          , poolStripes : Natural
          , poolIdleTimeout : Natural
          }
      , default = { poolStripes = 0, poolIdleTimeout = 300 }
      }

let LogLevel =
      { Type =
          < LogLevelDebug
          | LogLevelInfo
          | LogLevelWarn
          | LogLevelError
          | LogLevelOther : Text
          >
      }

let Config =
      { Type =
          { listenPort : Natural
          , database : PostgresConfig.Type
          , databasePoolSize : Natural
          , staticDir : Text
          , logLevel : LogLevel.Type
          , nodeName : Text
          , signUpKey : Text
          , rpId : Text
          }
      , default =
        { databasePoolSize = 10
        , staticDir = "./static"
        , logLevel = LogLevel.Type.LogLevelInfo
        }
      }

in  Config::{
    , listenPort = 62936
    , database = PostgresConfig::{
      , connectionString = "postgresql:///darkkirb?host=/run/postgresql"
      }
    , logLevel = LogLevel.Type.LogLevelDebug
    , nodeName = "example"
    , signUpKey = "changeme"
    , rpId = "thinkrac.tailbab65.ts.net"
    }
