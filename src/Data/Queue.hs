-- | Postgresql queue
module Data.Queue (Queue (..), run, scheduleTask, addTask, runTaskIn) where

import Codec.Serialise (Serialise, deserialise, serialise)
import Control.Concurrent (forkIO, threadDelay)
import Control.Monad.Trans.Resource (MonadUnliftIO)
import Data.ByteString qualified as BS
import Data.Time (NominalDiffTime, UTCTime, addUTCTime, getCurrentTime)
import Database.Persist (Entity (entityKey, entityVal), PersistEntity (Key), PersistQueryWrite (updateWhere), PersistStoreWrite (delete, update), PersistValue (PersistUTCTime), (+=.), (<=.), (=.))
import Database.Persist qualified as P
import Database.Persist.Sql (ConnectionPool, SqlPersistT, rawSql, runSqlPool)
import GHC.Conc (getNumProcessors)
import Model (EntityField (..), Jobs (..))
import System.Random (randomRIO)
import Utils (repeatM, timeoutM, whileM)
import Yesod (MonadUnliftIO (withRunInIO))

data Queue m a e where
  Queue ::
    (MonadUnliftIO m, Serialise a, Serialise e) =>
    { queueDbPool :: ConnectionPool
    , queueHandler :: a -> m (Either e ())
    } ->
    Queue m a e

claim :: (MonadUnliftIO m, Serialise a) => SqlPersistT m (Maybe (Key Jobs, a))
claim = do
  time <- liftIO getCurrentTime
  jobs :: [Entity Jobs] <- rawSql "UPDATE jobs SET updated_at = ?, locked = 'T', locked_at = ? WHERE id IN (SELECT id FROM jobs WHERE locked = 'F' AND run_at <= ? ORDER BY run_at ASC, created_at ASC LIMIT 1) RETURNING ??" [PersistUTCTime time, PersistUTCTime time, PersistUTCTime time]
  case jobs of
    [job] -> return $ Just (entityKey job, deserialise $ fromStrict $ jobsPayload $ entityVal job)
    [] -> return Nothing
    _ -> error "Database error"

unclaim :: (MonadUnliftIO m, Serialise e) => (Key Jobs, e) -> SqlPersistT m ()
unclaim (queueId, value) = do
  time <- liftIO getCurrentTime
  current <-
    P.get queueId >>= \case
      Just current -> return current
      Nothing -> error "Missing queue item"
  -- exponential backoff for delay
  let nextDelaySecs = round (min 604800 $ 60 * 2 ** fromIntegral (jobsAttempts current) :: Double)
  let nextDelay :: NominalDiffTime = fromInteger nextDelaySecs
  let runAt = addUTCTime nextDelay time
  update
    queueId
    [ JobsUpdatedAt =. time
    , JobsRunAt =. runAt
    , JobsLastError =. toStrict (serialise value)
    , JobsAttempts +=. 1
    , JobsLocked =. False
    , JobsLocked_at =. Nothing
    , JobsLocked_by =. Nothing
    ]

runOne :: (MonadUnliftIO m, Serialise a, Serialise e) => Queue m a e -> m Bool
runOne queue = do
  let dbpool = queueDbPool queue
  claimed <- runSqlPool claim dbpool
  case claimed of
    Nothing -> return False
    Just (queueId, job) -> do
      result <- timeoutM 300_000_000 $ queueHandler queue job
      case result of
        Just (Right _) -> runSqlPool (delete queueId) dbpool
        v -> runSqlPool (unclaim (queueId, v)) dbpool
      return True

runThread :: (MonadUnliftIO m, Serialise a, Serialise e) => Queue m e a -> m ()
runThread queue = do
  liftIO $ putStrLn "Run queue…"
  -- Run until queue is empty
  whileM $ runOne queue
  secs <- liftIO $ randomRIO (10 :: Int, 30)
  liftIO $ threadDelay (secs * 1_000_000)
  pass

cleanupThread :: (MonadUnliftIO m, Serialise a, Serialise e) => Queue m e a -> m ()
cleanupThread queue = do
  liftIO $ putStrLn "Cleanup queue…"
  now <- liftIO getCurrentTime
  let oldTime = addUTCTime (-360 :: NominalDiffTime) now
  runSqlPool (updateWhere [JobsLocked_at <=. Just oldTime] [JobsUpdatedAt =. now, JobsLocked =. False, JobsLocked_at =. Nothing, JobsLocked_by =. Nothing]) (queueDbPool queue)
  liftIO $ threadDelay 360_000_000

run :: (MonadUnliftIO m, Serialise a, Serialise e) => Queue m e a -> m ()
run queue = do
  concurrency <- liftIO getNumProcessors
  repeatM
    concurrency
    ( withRunInIO
        ( \run' -> forkIO $ do
            _ <- infinitely $ run' $ runThread queue
            pass
        )
        >> pass
    )
  _ <-
    withRunInIO
      ( \run' -> forkIO $ do
          _ <- infinitely $ run' $ cleanupThread queue
          pass
      )
  pass

scheduleTask :: (MonadUnliftIO m, Serialise a, Serialise e) => Queue m e a -> a -> UTCTime -> m ()
scheduleTask queue task at = do
  now <- liftIO getCurrentTime
  runSqlPool (P.insert_ $ Jobs now now at (toStrict $ serialise task) BS.empty 0 False Nothing Nothing) (queueDbPool queue)
  pass

addTask :: (MonadUnliftIO m, Serialise a, Serialise e) => Queue m e a -> a -> m ()
addTask queue task = do
  now <- liftIO getCurrentTime
  scheduleTask queue task now

runTaskIn :: (MonadUnliftIO m, Serialise a, Serialise e) => Queue m e a -> a -> NominalDiffTime -> m ()
runTaskIn queue task in_time = do
  now <- liftIO getCurrentTime
  scheduleTask queue task (addUTCTime in_time now)