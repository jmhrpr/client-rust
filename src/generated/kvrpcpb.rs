/// A transactional get command. Lookup a value for `key` in the transaction with
/// starting timestamp = `version`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "3")]
    pub version: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetResponse {
    /// A region error indicates that the request was sent to the wrong TiKV node
    /// (or other, similar errors).
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    /// A value could not be retrieved due to the state of the database for the requested key.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<KeyError>,
    /// A successful result.
    #[prost(bytes = "vec", tag = "3")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    /// True if the key does not exist in the database.
    #[prost(bool, tag = "4")]
    pub not_found: bool,
    /// Time and scan details when processing the request.
    #[prost(message, optional, tag = "6")]
    pub exec_details_v2: ::core::option::Option<ExecDetailsV2>,
}
/// Scan fetches values for a range of keys; it is part of the transaction with
/// starting timestamp = `version`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScanRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    #[prost(bytes = "vec", tag = "2")]
    pub start_key: ::prost::alloc::vec::Vec<u8>,
    /// The maximum number of results to return.
    #[prost(uint32, tag = "3")]
    pub limit: u32,
    #[prost(uint64, tag = "4")]
    pub version: u64,
    /// Return only the keys found by scanning, not their values.
    #[prost(bool, tag = "5")]
    pub key_only: bool,
    #[prost(bool, tag = "6")]
    pub reverse: bool,
    /// For compatibility, when scanning forward, the range to scan is \[start_key, end_key), where start_key \< end_key;
    /// and when scanning backward, it scans \[end_key, start_key) in descending order, where end_key \< start_key.
    #[prost(bytes = "vec", tag = "7")]
    pub end_key: ::prost::alloc::vec::Vec<u8>,
    /// If sample_step > 0, skips 'sample_step - 1' number of keys after each returned key.
    /// locks are not checked.
    #[prost(uint32, tag = "8")]
    pub sample_step: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScanResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    /// Each KvPair may contain a key error.
    #[prost(message, repeated, tag = "2")]
    pub pairs: ::prost::alloc::vec::Vec<KvPair>,
    /// This KeyError exists when some key is locked but we cannot check locks of all keys.
    /// In this case, `pairs` should be empty and the client should redo scanning all the keys
    /// after resolving the lock.
    #[prost(message, optional, tag = "3")]
    pub error: ::core::option::Option<KeyError>,
}
/// A prewrite is the first phase of writing to TiKV. It contains all data to be written in a transaction.
/// TiKV will write the data in a preliminary state. Data cannot be read until it has been committed.
/// The client should only commit a transaction once all prewrites succeed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrewriteRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    /// The data to be written to the database.
    #[prost(message, repeated, tag = "2")]
    pub mutations: ::prost::alloc::vec::Vec<Mutation>,
    /// The client picks one key to be primary (unrelated to the primary key concept in SQL). This
    /// key's lock is the source of truth for the state of a transaction. All other locks due to a
    /// transaction will point to the primary lock.
    #[prost(bytes = "vec", tag = "3")]
    pub primary_lock: ::prost::alloc::vec::Vec<u8>,
    /// Identifies the transaction being written.
    #[prost(uint64, tag = "4")]
    pub start_version: u64,
    #[prost(uint64, tag = "5")]
    pub lock_ttl: u64,
    /// TiKV can skip some checks, used for speeding up data migration.
    #[prost(bool, tag = "6")]
    pub skip_constraint_check: bool,
    /// For pessimistic transaction, some mutations don't need to be locked, for example, non-unique index key.
    #[prost(bool, repeated, tag = "7")]
    pub is_pessimistic_lock: ::prost::alloc::vec::Vec<bool>,
    /// How many keys this transaction involves in this region.
    #[prost(uint64, tag = "8")]
    pub txn_size: u64,
    /// For pessimistic transactions only; used to check if a conflict lock is already committed.
    #[prost(uint64, tag = "9")]
    pub for_update_ts: u64,
    /// If min_commit_ts > 0, this is a large transaction request, the final commit_ts
    /// will be inferred from `min_commit_ts`.
    #[prost(uint64, tag = "10")]
    pub min_commit_ts: u64,
    /// When async commit is enabled, `secondaries` should be set as the key list of all secondary
    /// locks if the request prewrites the primary lock.
    #[prost(bool, tag = "11")]
    pub use_async_commit: bool,
    #[prost(bytes = "vec", repeated, tag = "12")]
    pub secondaries: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// When the transaction involves only one region, it's possible to commit the transaction
    /// directly with 1PC protocol.
    #[prost(bool, tag = "13")]
    pub try_one_pc: bool,
    /// The max commit ts is reserved for limiting the commit ts of 1PC or async commit, which can be used to avoid
    /// inconsistency with schema change.
    #[prost(uint64, tag = "14")]
    pub max_commit_ts: u64,
    /// The level of assertion to use on this prewrte request.
    #[prost(enumeration = "AssertionLevel", tag = "15")]
    pub assertion_level: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrewriteResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    #[prost(message, repeated, tag = "2")]
    pub errors: ::prost::alloc::vec::Vec<KeyError>,
    /// 0 if the min_commit_ts is not ready or any other reason that async
    /// commit cannot proceed. The client can then fallback to normal way to
    /// continue committing the transaction if prewrite are all finished.
    #[prost(uint64, tag = "3")]
    pub min_commit_ts: u64,
    /// When the transaction is successfully committed with 1PC protocol, this field will be set to
    /// the commit ts of the transaction. Otherwise, if TiKV failed to commit it with 1PC or the
    /// transaction is not 1PC, the value will be 0.
    #[prost(uint64, tag = "4")]
    pub one_pc_commit_ts: u64,
}
/// Lock a set of keys to prepare to write to them.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PessimisticLockRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    /// In this case every `Op` of the mutations must be `PessimisticLock`.
    #[prost(message, repeated, tag = "2")]
    pub mutations: ::prost::alloc::vec::Vec<Mutation>,
    #[prost(bytes = "vec", tag = "3")]
    pub primary_lock: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "4")]
    pub start_version: u64,
    #[prost(uint64, tag = "5")]
    pub lock_ttl: u64,
    /// Each locking command in a pessimistic transaction has its own timestamp. If locking fails, then
    /// the corresponding SQL statement can be retried with a later timestamp, TiDB does not need to
    /// retry the whole transaction. The name comes from the `SELECT ... FOR UPDATE` SQL statement which
    /// is a locking read. Each `SELECT ... FOR UPDATE` in a transaction will be assigned its own
    /// timestamp.
    #[prost(uint64, tag = "6")]
    pub for_update_ts: u64,
    /// If the request is the first lock request, we don't need to detect deadlock.
    #[prost(bool, tag = "7")]
    pub is_first_lock: bool,
    /// Time to wait for lock released in milliseconds when encountering locks.
    /// 0 means using default timeout in TiKV. Negative means no wait.
    #[prost(int64, tag = "8")]
    pub wait_timeout: i64,
    /// If it is true, TiKV will acquire the pessimistic lock regardless of write conflict
    /// and return the latest value. It's only supported for single mutation.
    #[deprecated]
    #[prost(bool, tag = "9")]
    pub force: bool,
    /// If it is true, TiKV will return values of the keys if no error, so TiDB can cache the values for
    /// later read in the same transaction.
    /// When 'force' is set to true, this field is ignored.
    #[prost(bool, tag = "10")]
    pub return_values: bool,
    /// If min_commit_ts > 0, this is large transaction proto, the final commit_ts
    /// would be infered from min_commit_ts.
    #[prost(uint64, tag = "11")]
    pub min_commit_ts: u64,
    /// If set to true, it means TiKV need to check if the key exists, and return the result in
    /// the `not_founds` feild in the response. This works no matter if `return_values` is set. If
    /// `return_values` is set, it simply makes no difference; otherwise, the `value` field of the
    /// repsonse will be empty while the `not_founds` field still indicates the keys' existence.
    #[prost(bool, tag = "12")]
    pub check_existence: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PessimisticLockResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    #[prost(message, repeated, tag = "2")]
    pub errors: ::prost::alloc::vec::Vec<KeyError>,
    /// It carries the latest value and its commit ts if force in PessimisticLockRequest is true.
    #[deprecated]
    #[prost(uint64, tag = "3")]
    pub commit_ts: u64,
    #[deprecated]
    #[prost(bytes = "vec", tag = "4")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    /// The values is set if 'return_values' is true in the request and no error.
    /// If 'force' is true, this field is not used.
    #[prost(bytes = "vec", repeated, tag = "5")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// Indicates whether the values at the same index is correspond to an existing key.
    /// In legacy TiKV, this field is not used even 'force' is false. In that case, an empty value indicates
    /// two possible situations: (1) the key does not exist. (2) the key exists but the value is empty.
    #[prost(bool, repeated, tag = "6")]
    pub not_founds: ::prost::alloc::vec::Vec<bool>,
}
/// Unlock keys locked using `PessimisticLockRequest`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PessimisticRollbackRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    #[prost(uint64, tag = "2")]
    pub start_version: u64,
    #[prost(uint64, tag = "3")]
    pub for_update_ts: u64,
    #[prost(bytes = "vec", repeated, tag = "4")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PessimisticRollbackResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    #[prost(message, repeated, tag = "2")]
    pub errors: ::prost::alloc::vec::Vec<KeyError>,
}
/// Used to update the lock_ttl of a psessimistic and/or large transaction to prevent it from been killed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxnHeartBeatRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    /// The key of the lock to update.
    #[prost(bytes = "vec", tag = "2")]
    pub primary_lock: ::prost::alloc::vec::Vec<u8>,
    /// Start timestamp of the large transaction.
    #[prost(uint64, tag = "3")]
    pub start_version: u64,
    /// The new TTL the sender would like.
    #[prost(uint64, tag = "4")]
    pub advise_lock_ttl: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxnHeartBeatResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<KeyError>,
    /// The TTL actually set on the requested lock.
    #[prost(uint64, tag = "3")]
    pub lock_ttl: u64,
}
/// CheckTxnStatusRequest checks the status of a transaction.
/// If the transaction is rollbacked/committed, return that result.
/// If the TTL of the transaction is exhausted, abort that transaction and inform the caller.
/// Otherwise, returns the TTL information for the transaction.
/// CheckTxnStatusRequest may also push forward the minCommitTS of a large transaction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckTxnStatusRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    /// Primary key and lock ts together to locate the primary lock of a transaction.
    #[prost(bytes = "vec", tag = "2")]
    pub primary_key: ::prost::alloc::vec::Vec<u8>,
    /// Starting timestamp of the transaction being checked.
    #[prost(uint64, tag = "3")]
    pub lock_ts: u64,
    /// The start timestamp of the transaction which this request is part of.
    #[prost(uint64, tag = "4")]
    pub caller_start_ts: u64,
    /// The client must specify the current time to TiKV using this timestamp. It is used to check TTL
    /// timeouts. It may be inaccurate.
    #[prost(uint64, tag = "5")]
    pub current_ts: u64,
    /// If true, then TiKV will leave a rollback tombstone in the write CF for `primary_key`, even if
    /// that key is not locked.
    #[prost(bool, tag = "6")]
    pub rollback_if_not_exist: bool,
    /// This field is set to true only if the transaction is known to fall back from async commit.
    /// Then, CheckTxnStatus treats the transaction as non-async-commit even if the use_async_commit
    /// field in the primary lock is true.
    #[prost(bool, tag = "7")]
    pub force_sync_commit: bool,
    /// If the check request is used to resolve or decide the transaction status for a input pessimistic
    /// lock, the transaction status could not be decided if the primary lock is pessimistic too and
    /// it's still uncertain.
    #[prost(bool, tag = "8")]
    pub resolving_pessimistic_lock: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckTxnStatusResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<KeyError>,
    /// Three kinds of transaction status:
    /// locked: lock_ttl > 0
    /// committed: commit_version > 0
    /// rollbacked: lock_ttl = 0 && commit_version = 0
    #[prost(uint64, tag = "3")]
    pub lock_ttl: u64,
    #[prost(uint64, tag = "4")]
    pub commit_version: u64,
    /// The action performed by TiKV (and why if the action is to rollback).
    #[prost(enumeration = "Action", tag = "5")]
    pub action: i32,
    #[prost(message, optional, tag = "6")]
    pub lock_info: ::core::option::Option<LockInfo>,
}
/// Part of the async commit protocol, checks for locks on all supplied keys. If a lock is missing,
/// does not have a successful status, or belongs to another transaction, TiKV will leave a rollback
/// tombstone for that key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckSecondaryLocksRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// Identifies the transaction we are investigating.
    #[prost(uint64, tag = "3")]
    pub start_version: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckSecondaryLocksResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<KeyError>,
    /// For each key in `keys` in `CheckSecondaryLocks`, there will be a lock in
    /// this list if there is a lock present and belonging to the correct transaction,
    /// nil otherwise.
    #[prost(message, repeated, tag = "3")]
    pub locks: ::prost::alloc::vec::Vec<LockInfo>,
    /// If any of the locks have been committed, this is the commit ts used. If no
    /// locks have been committed, it will be zero.
    #[prost(uint64, tag = "4")]
    pub commit_ts: u64,
}
/// The second phase of writing to TiKV. If there are no errors or conflicts, then this request
/// commits a transaction so that its data can be read by other transactions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    /// Identifies the transaction.
    #[prost(uint64, tag = "2")]
    pub start_version: u64,
    /// All keys in the transaction (to be committed).
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// Timestamp for the end of the transaction. Must be greater than `start_version`.
    #[prost(uint64, tag = "4")]
    pub commit_version: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<KeyError>,
    /// If the commit ts is derived from min_commit_ts, this field should be set.
    #[prost(uint64, tag = "3")]
    pub commit_version: u64,
}
/// Not yet implemented.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportRequest {
    #[prost(message, repeated, tag = "1")]
    pub mutations: ::prost::alloc::vec::Vec<Mutation>,
    #[prost(uint64, tag = "2")]
    pub commit_version: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    #[prost(string, tag = "2")]
    pub error: ::prost::alloc::string::String,
}
/// Cleanup a key by possibly unlocking it.
/// From 4.0 onwards, this message is no longer used.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CleanupRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "3")]
    pub start_version: u64,
    /// The current timestamp, used in combination with a lock's TTL to determine
    /// if the lock has expired. If `current_ts == 0`, then the key will be unlocked
    /// irrespective of its TTL.
    #[prost(uint64, tag = "4")]
    pub current_ts: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CleanupResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<KeyError>,
    /// Set if the key is already committed.
    #[prost(uint64, tag = "3")]
    pub commit_version: u64,
}
/// Similar to a `Get` request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, tag = "3")]
    pub version: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    #[prost(message, repeated, tag = "2")]
    pub pairs: ::prost::alloc::vec::Vec<KvPair>,
    /// Time and scan details when processing the request.
    #[prost(message, optional, tag = "4")]
    pub exec_details_v2: ::core::option::Option<ExecDetailsV2>,
    /// This KeyError exists when some key is locked but we cannot check locks of all keys.
    /// In this case, `pairs` should be empty and the client should redo batch get all the keys
    /// after resolving the lock.
    #[prost(message, optional, tag = "5")]
    pub error: ::core::option::Option<KeyError>,
}
/// Rollback a prewritten transaction. This will remove the preliminary data from the database,
/// unlock locks, and leave a rollback tombstone.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchRollbackRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    /// Identify the transaction to be rolled back.
    #[prost(uint64, tag = "2")]
    pub start_version: u64,
    /// The keys to rollback.
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchRollbackResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<KeyError>,
}
/// Scan the database for locks. Used at the start of the GC process to find all
/// old locks.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScanLockRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    /// Returns all locks with a start timestamp before `max_version`.
    #[prost(uint64, tag = "2")]
    pub max_version: u64,
    /// Start scanning from this key.
    #[prost(bytes = "vec", tag = "3")]
    pub start_key: ::prost::alloc::vec::Vec<u8>,
    /// The maximum number of locks to return.
    #[prost(uint32, tag = "4")]
    pub limit: u32,
    /// The exclusive upperbound for scanning.
    #[prost(bytes = "vec", tag = "5")]
    pub end_key: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScanLockResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<KeyError>,
    /// Info on all locks found by the scan.
    #[prost(message, repeated, tag = "3")]
    pub locks: ::prost::alloc::vec::Vec<LockInfo>,
}
/// For all keys locked by the transaction identified by `start_version`, either
/// commit or rollback the transaction and unlock the key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveLockRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    #[prost(uint64, tag = "2")]
    pub start_version: u64,
    /// `commit_version == 0` means the transaction was rolled back.
    /// `commit_version > 0` means the transaction was committed at the given timestamp.
    #[prost(uint64, tag = "3")]
    pub commit_version: u64,
    #[prost(message, repeated, tag = "4")]
    pub txn_infos: ::prost::alloc::vec::Vec<TxnInfo>,
    /// Only resolve specified keys.
    #[prost(bytes = "vec", repeated, tag = "5")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveLockResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<KeyError>,
}
/// Request TiKV to garbage collect all non-current data older than `safe_point`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    #[prost(uint64, tag = "2")]
    pub safe_point: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<KeyError>,
}
/// Delete a range of data from TiKV.
/// This message should not be used.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRangeRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    #[prost(bytes = "vec", tag = "2")]
    pub start_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub end_key: ::prost::alloc::vec::Vec<u8>,
    /// If true, the data will not be immediately deleted, but the operation will
    /// still be replicated via Raft. This is used to notify TiKV that the data
    /// will be deleted using `unsafe_destroy_range` soon.
    #[prost(bool, tag = "4")]
    pub notify_only: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRangeResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    #[prost(string, tag = "2")]
    pub error: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawGetRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub cf: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawGetResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    #[prost(string, tag = "2")]
    pub error: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "4")]
    pub not_found: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawBatchGetRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, tag = "3")]
    pub cf: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawBatchGetResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    #[prost(message, repeated, tag = "2")]
    pub pairs: ::prost::alloc::vec::Vec<KvPair>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawPutRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "4")]
    pub cf: ::prost::alloc::string::String,
    #[prost(uint64, tag = "5")]
    pub ttl: u64,
    #[prost(bool, tag = "6")]
    pub for_cas: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawPutResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    #[prost(string, tag = "2")]
    pub error: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawBatchPutRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    #[prost(message, repeated, tag = "2")]
    pub pairs: ::prost::alloc::vec::Vec<KvPair>,
    #[prost(string, tag = "3")]
    pub cf: ::prost::alloc::string::String,
    /// The time-to-live for each keys in seconds, and if the length of `ttls`
    /// is exactly one, the ttl will be applied to all keys. Otherwise, the length
    /// mismatch between `ttls` and `pairs` will return an error.
    #[prost(uint64, repeated, tag = "4")]
    pub ttls: ::prost::alloc::vec::Vec<u64>,
    #[prost(bool, tag = "5")]
    pub for_cas: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawBatchPutResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    #[prost(string, tag = "2")]
    pub error: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawDeleteRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub cf: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub for_cas: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawDeleteResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    #[prost(string, tag = "2")]
    pub error: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawBatchDeleteRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, tag = "3")]
    pub cf: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub for_cas: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawBatchDeleteResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    #[prost(string, tag = "2")]
    pub error: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawScanRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    #[prost(bytes = "vec", tag = "2")]
    pub start_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "3")]
    pub limit: u32,
    #[prost(bool, tag = "4")]
    pub key_only: bool,
    #[prost(string, tag = "5")]
    pub cf: ::prost::alloc::string::String,
    #[prost(bool, tag = "6")]
    pub reverse: bool,
    /// For compatibility, when scanning forward, the range to scan is \[start_key, end_key), where start_key \< end_key;
    /// and when scanning backward, it scans \[end_key, start_key) in descending order, where end_key \< start_key.
    #[prost(bytes = "vec", tag = "7")]
    pub end_key: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawScanResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    #[prost(message, repeated, tag = "2")]
    pub kvs: ::prost::alloc::vec::Vec<KvPair>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawDeleteRangeRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    #[prost(bytes = "vec", tag = "2")]
    pub start_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub end_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "4")]
    pub cf: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawDeleteRangeResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    #[prost(string, tag = "2")]
    pub error: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawBatchScanRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    /// scanning range
    #[prost(message, repeated, tag = "2")]
    pub ranges: ::prost::alloc::vec::Vec<KeyRange>,
    /// max number of returning kv pairs for each scanning range
    #[prost(uint32, tag = "3")]
    pub each_limit: u32,
    #[prost(bool, tag = "4")]
    pub key_only: bool,
    #[prost(string, tag = "5")]
    pub cf: ::prost::alloc::string::String,
    #[prost(bool, tag = "6")]
    pub reverse: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawBatchScanResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    #[prost(message, repeated, tag = "2")]
    pub kvs: ::prost::alloc::vec::Vec<KvPair>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnsafeDestroyRangeRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    #[prost(bytes = "vec", tag = "2")]
    pub start_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub end_key: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnsafeDestroyRangeResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    #[prost(string, tag = "2")]
    pub error: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterLockObserverRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    #[prost(uint64, tag = "2")]
    pub max_ts: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterLockObserverResponse {
    #[prost(string, tag = "1")]
    pub error: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckLockObserverRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    #[prost(uint64, tag = "2")]
    pub max_ts: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckLockObserverResponse {
    #[prost(string, tag = "1")]
    pub error: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub is_clean: bool,
    #[prost(message, repeated, tag = "3")]
    pub locks: ::prost::alloc::vec::Vec<LockInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveLockObserverRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    #[prost(uint64, tag = "2")]
    pub max_ts: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveLockObserverResponse {
    #[prost(string, tag = "1")]
    pub error: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhysicalScanLockRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    #[prost(uint64, tag = "2")]
    pub max_ts: u64,
    #[prost(bytes = "vec", tag = "3")]
    pub start_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "4")]
    pub limit: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhysicalScanLockResponse {
    #[prost(string, tag = "1")]
    pub error: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub locks: ::prost::alloc::vec::Vec<LockInfo>,
}
/// Sent from PD to a TiKV node.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplitRegionRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    #[deprecated]
    #[prost(bytes = "vec", tag = "2")]
    pub split_key: ::prost::alloc::vec::Vec<u8>,
    /// when use it to do batch split, `split_key` should be empty.
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub split_keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// Once enabled, the split_key will not be encoded.
    #[prost(bool, tag = "4")]
    pub is_raw_kv: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplitRegionResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    /// set when there are only 2 result regions.
    #[deprecated]
    #[prost(message, optional, tag = "2")]
    pub left: ::core::option::Option<super::metapb::Region>,
    /// set when there are only 2 result regions.
    #[deprecated]
    #[prost(message, optional, tag = "3")]
    pub right: ::core::option::Option<super::metapb::Region>,
    /// include all result regions.
    #[prost(message, repeated, tag = "4")]
    pub regions: ::prost::alloc::vec::Vec<super::metapb::Region>,
}
/// Sent from TiFlash to a TiKV node.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadIndexRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    /// TiKV checks the given range if there is any unapplied lock
    /// blocking the read request.
    #[prost(uint64, tag = "2")]
    pub start_ts: u64,
    #[prost(message, repeated, tag = "3")]
    pub ranges: ::prost::alloc::vec::Vec<KeyRange>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadIndexResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    #[prost(uint64, tag = "2")]
    pub read_index: u64,
    /// If `locked` is set, this read request is blocked by a lock.
    /// The lock should be returned to the client.
    #[prost(message, optional, tag = "3")]
    pub locked: ::core::option::Option<LockInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MvccGetByKeyRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MvccGetByKeyResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    #[prost(string, tag = "2")]
    pub error: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub info: ::core::option::Option<MvccInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MvccGetByStartTsRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    #[prost(uint64, tag = "2")]
    pub start_ts: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MvccGetByStartTsResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    #[prost(string, tag = "2")]
    pub error: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "4")]
    pub info: ::core::option::Option<MvccInfo>,
}
/// Miscellaneous metadata attached to most requests.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Context {
    #[prost(uint64, tag = "1")]
    pub region_id: u64,
    #[prost(message, optional, tag = "2")]
    pub region_epoch: ::core::option::Option<super::metapb::RegionEpoch>,
    #[prost(message, optional, tag = "3")]
    pub peer: ::core::option::Option<super::metapb::Peer>,
    #[prost(uint64, tag = "5")]
    pub term: u64,
    #[prost(enumeration = "CommandPri", tag = "6")]
    pub priority: i32,
    #[prost(enumeration = "IsolationLevel", tag = "7")]
    pub isolation_level: i32,
    #[prost(bool, tag = "8")]
    pub not_fill_cache: bool,
    #[prost(bool, tag = "9")]
    pub sync_log: bool,
    /// True means execution time statistics should be recorded and returned.
    #[prost(bool, tag = "10")]
    pub record_time_stat: bool,
    /// True means RocksDB scan statistics should be recorded and returned.
    #[prost(bool, tag = "11")]
    pub record_scan_stat: bool,
    #[prost(bool, tag = "12")]
    pub replica_read: bool,
    /// Read requests can ignore locks belonging to these transactions because either
    /// these transactions are rolled back or theirs commit_ts > read request's start_ts.
    #[prost(uint64, repeated, tag = "13")]
    pub resolved_locks: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, tag = "14")]
    pub max_execution_duration_ms: u64,
    /// After a region applies to `applied_index`, we can get a
    /// snapshot for the region even if the peer is a follower.
    #[prost(uint64, tag = "15")]
    pub applied_index: u64,
    /// A hint for TiKV to schedule tasks more fairly. Query with same task ID
    /// may share same priority and resource quota.
    #[prost(uint64, tag = "16")]
    pub task_id: u64,
    /// Not required to read the most up-to-date data, replicas with `safe_ts` >= `start_ts`
    /// can handle read request directly
    #[prost(bool, tag = "17")]
    pub stale_read: bool,
    /// Any additional serialized information about the request.
    #[prost(bytes = "vec", tag = "18")]
    pub resource_group_tag: ::prost::alloc::vec::Vec<u8>,
    /// Used to tell TiKV whether operations are allowed or not on different disk usages.
    #[prost(enumeration = "DiskFullOpt", tag = "19")]
    pub disk_full_opt: i32,
    /// Indicates the request is a retry request and the same request may have been sent before.
    #[prost(bool, tag = "20")]
    pub is_retry_request: bool,
    /// API version implies the encode of the key and value.
    #[prost(enumeration = "ApiVersion", tag = "21")]
    pub api_version: i32,
    /// Read request should read through locks belonging to these transactions because these
    /// transactions are committed and theirs commit_ts \<= read request's start_ts.
    #[prost(uint64, repeated, tag = "22")]
    pub committed_locks: ::prost::alloc::vec::Vec<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockInfo {
    #[prost(bytes = "vec", tag = "1")]
    pub primary_lock: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "2")]
    pub lock_version: u64,
    #[prost(bytes = "vec", tag = "3")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "4")]
    pub lock_ttl: u64,
    /// How many keys this transaction involves in this region.
    #[prost(uint64, tag = "5")]
    pub txn_size: u64,
    #[prost(enumeration = "Op", tag = "6")]
    pub lock_type: i32,
    #[prost(uint64, tag = "7")]
    pub lock_for_update_ts: u64,
    /// Fields for transactions that are using Async Commit.
    #[prost(bool, tag = "8")]
    pub use_async_commit: bool,
    #[prost(uint64, tag = "9")]
    pub min_commit_ts: u64,
    #[prost(bytes = "vec", repeated, tag = "10")]
    pub secondaries: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyError {
    /// Client should backoff or cleanup the lock then retry.
    #[prost(message, optional, tag = "1")]
    pub locked: ::core::option::Option<LockInfo>,
    /// Client may restart the txn. e.g write conflict.
    #[prost(string, tag = "2")]
    pub retryable: ::prost::alloc::string::String,
    /// Client should abort the txn.
    #[prost(string, tag = "3")]
    pub abort: ::prost::alloc::string::String,
    /// Write conflict is moved from retryable to here.
    #[prost(message, optional, tag = "4")]
    pub conflict: ::core::option::Option<WriteConflict>,
    /// Key already exists
    #[prost(message, optional, tag = "5")]
    pub already_exist: ::core::option::Option<AlreadyExist>,
    /// Deadlock is used in pessimistic transaction for single statement rollback.
    #[prost(message, optional, tag = "6")]
    pub deadlock: ::core::option::Option<Deadlock>,
    /// Commit ts is earlier than min commit ts of a transaction.
    #[prost(message, optional, tag = "7")]
    pub commit_ts_expired: ::core::option::Option<CommitTsExpired>,
    /// Txn not found when checking txn status.
    #[prost(message, optional, tag = "8")]
    pub txn_not_found: ::core::option::Option<TxnNotFound>,
    /// Calculated commit TS exceeds the limit given by the user.
    #[prost(message, optional, tag = "9")]
    pub commit_ts_too_large: ::core::option::Option<CommitTsTooLarge>,
    /// Assertion of a `Mutation` is evaluated as a failure.
    #[prost(message, optional, tag = "10")]
    pub assertion_failed: ::core::option::Option<AssertionFailed>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteConflict {
    #[prost(uint64, tag = "1")]
    pub start_ts: u64,
    #[prost(uint64, tag = "2")]
    pub conflict_ts: u64,
    #[prost(bytes = "vec", tag = "3")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    pub primary: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "5")]
    pub conflict_commit_ts: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlreadyExist {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Deadlock {
    #[prost(uint64, tag = "1")]
    pub lock_ts: u64,
    #[prost(bytes = "vec", tag = "2")]
    pub lock_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "3")]
    pub deadlock_key_hash: u64,
    #[prost(message, repeated, tag = "4")]
    pub wait_chain: ::prost::alloc::vec::Vec<super::deadlock::WaitForEntry>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitTsExpired {
    #[prost(uint64, tag = "1")]
    pub start_ts: u64,
    #[prost(uint64, tag = "2")]
    pub attempted_commit_ts: u64,
    #[prost(bytes = "vec", tag = "3")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "4")]
    pub min_commit_ts: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxnNotFound {
    #[prost(uint64, tag = "1")]
    pub start_ts: u64,
    #[prost(bytes = "vec", tag = "2")]
    pub primary_key: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitTsTooLarge {
    /// The calculated commit TS.
    #[prost(uint64, tag = "1")]
    pub commit_ts: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssertionFailed {
    #[prost(uint64, tag = "1")]
    pub start_ts: u64,
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "Assertion", tag = "3")]
    pub assertion: i32,
    #[prost(uint64, tag = "4")]
    pub existing_start_ts: u64,
    #[prost(uint64, tag = "5")]
    pub existing_commit_ts: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeDetail {
    /// Off-cpu wall time elapsed in TiKV side. Usually this includes queue waiting time and
    /// other kind of waitings in series.
    #[prost(int64, tag = "1")]
    pub wait_wall_time_ms: i64,
    /// Off-cpu and on-cpu wall time elapsed to actually process the request payload. It does not
    /// include `wait_wall_time`.
    /// This field is very close to the CPU time in most cases. Some wait time spend in RocksDB
    /// cannot be excluded for now, like Mutex wait time, which is included in this field, so that
    /// this field is called wall time instead of CPU time.
    #[prost(int64, tag = "2")]
    pub process_wall_time_ms: i64,
    /// KV read wall Time means the time used in key/value scan and get.
    #[prost(int64, tag = "3")]
    pub kv_read_wall_time_ms: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScanInfo {
    #[prost(int64, tag = "1")]
    pub total: i64,
    #[prost(int64, tag = "2")]
    pub processed: i64,
    #[prost(int64, tag = "3")]
    pub read_bytes: i64,
}
/// Only reserved for compatibility.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScanDetail {
    #[prost(message, optional, tag = "1")]
    pub write: ::core::option::Option<ScanInfo>,
    #[prost(message, optional, tag = "2")]
    pub lock: ::core::option::Option<ScanInfo>,
    #[prost(message, optional, tag = "3")]
    pub data: ::core::option::Option<ScanInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScanDetailV2 {
    /// Number of user keys scanned from the storage.
    /// It does not include deleted version or RocksDB tombstone keys.
    /// For Coprocessor requests, it includes keys that has been filtered out by
    /// Selection.
    #[prost(uint64, tag = "1")]
    pub processed_versions: u64,
    /// Number of bytes of user key-value pairs scanned from the storage, i.e.
    /// total size of data returned from MVCC layer.
    #[prost(uint64, tag = "8")]
    pub processed_versions_size: u64,
    /// Approximate number of MVCC keys meet during scanning. It includes
    /// deleted versions, but does not include RocksDB tombstone keys.
    ///
    /// When this field is notably larger than `processed_versions`, it means
    /// there are a lot of deleted MVCC keys.
    #[prost(uint64, tag = "2")]
    pub total_versions: u64,
    /// Total number of deletes and single deletes skipped over during
    /// iteration, i.e. how many RocksDB tombstones are skipped.
    #[prost(uint64, tag = "3")]
    pub rocksdb_delete_skipped_count: u64,
    /// Total number of internal keys skipped over during iteration.
    /// See <https://github.com/facebook/rocksdb/blob/9f1c84ca471d8b1ad7be9f3eebfc2c7e07dfd7a7/include/rocksdb/perf_context.h#L84> for details.
    #[prost(uint64, tag = "4")]
    pub rocksdb_key_skipped_count: u64,
    /// Total number of RocksDB block cache hits.
    #[prost(uint64, tag = "5")]
    pub rocksdb_block_cache_hit_count: u64,
    /// Total number of block reads (with IO).
    #[prost(uint64, tag = "6")]
    pub rocksdb_block_read_count: u64,
    /// Total number of bytes from block reads.
    #[prost(uint64, tag = "7")]
    pub rocksdb_block_read_byte: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecDetails {
    /// Available when ctx.record_time_stat = true or meet slow query.
    #[prost(message, optional, tag = "1")]
    pub time_detail: ::core::option::Option<TimeDetail>,
    /// Available when ctx.record_scan_stat = true or meet slow query.
    #[prost(message, optional, tag = "2")]
    pub scan_detail: ::core::option::Option<ScanDetail>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecDetailsV2 {
    /// Available when ctx.record_time_stat = true or meet slow query.
    #[prost(message, optional, tag = "1")]
    pub time_detail: ::core::option::Option<TimeDetail>,
    /// Available when ctx.record_scan_stat = true or meet slow query.
    #[prost(message, optional, tag = "2")]
    pub scan_detail_v2: ::core::option::Option<ScanDetailV2>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KvPair {
    #[prost(message, optional, tag = "1")]
    pub error: ::core::option::Option<KeyError>,
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mutation {
    #[prost(enumeration = "Op", tag = "1")]
    pub op: i32,
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "Assertion", tag = "4")]
    pub assertion: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MvccWrite {
    #[prost(enumeration = "Op", tag = "1")]
    pub r#type: i32,
    #[prost(uint64, tag = "2")]
    pub start_ts: u64,
    #[prost(uint64, tag = "3")]
    pub commit_ts: u64,
    #[prost(bytes = "vec", tag = "4")]
    pub short_value: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "5")]
    pub has_overlapped_rollback: bool,
    #[prost(bool, tag = "6")]
    pub has_gc_fence: bool,
    #[prost(uint64, tag = "7")]
    pub gc_fence: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MvccValue {
    #[prost(uint64, tag = "1")]
    pub start_ts: u64,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MvccLock {
    #[prost(enumeration = "Op", tag = "1")]
    pub r#type: i32,
    #[prost(uint64, tag = "2")]
    pub start_ts: u64,
    #[prost(bytes = "vec", tag = "3")]
    pub primary: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    pub short_value: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "5")]
    pub ttl: u64,
    #[prost(uint64, tag = "6")]
    pub for_update_ts: u64,
    #[prost(uint64, tag = "7")]
    pub txn_size: u64,
    #[prost(bool, tag = "8")]
    pub use_async_commit: bool,
    #[prost(bytes = "vec", repeated, tag = "9")]
    pub secondaries: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, repeated, tag = "10")]
    pub rollback_ts: ::prost::alloc::vec::Vec<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MvccInfo {
    #[prost(message, optional, tag = "1")]
    pub lock: ::core::option::Option<MvccLock>,
    #[prost(message, repeated, tag = "2")]
    pub writes: ::prost::alloc::vec::Vec<MvccWrite>,
    #[prost(message, repeated, tag = "3")]
    pub values: ::prost::alloc::vec::Vec<MvccValue>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxnInfo {
    #[prost(uint64, tag = "1")]
    pub txn: u64,
    #[prost(uint64, tag = "2")]
    pub status: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyRange {
    #[prost(bytes = "vec", tag = "1")]
    pub start_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub end_key: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaderInfo {
    #[prost(uint64, tag = "1")]
    pub region_id: u64,
    #[prost(uint64, tag = "2")]
    pub peer_id: u64,
    #[prost(uint64, tag = "3")]
    pub term: u64,
    #[prost(message, optional, tag = "4")]
    pub region_epoch: ::core::option::Option<super::metapb::RegionEpoch>,
    #[prost(message, optional, tag = "5")]
    pub read_state: ::core::option::Option<ReadState>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadState {
    #[prost(uint64, tag = "1")]
    pub applied_index: u64,
    #[prost(uint64, tag = "2")]
    pub safe_ts: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckLeaderRequest {
    #[prost(message, repeated, tag = "1")]
    pub regions: ::prost::alloc::vec::Vec<LeaderInfo>,
    #[prost(uint64, tag = "2")]
    pub ts: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckLeaderResponse {
    #[prost(uint64, repeated, tag = "1")]
    pub regions: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, tag = "2")]
    pub ts: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreSafeTsRequest {
    /// Get the minimal `safe_ts` from regions that overlap with the key range \[`start_key`, `end_key`)
    /// An empty key range means all regions in the store
    #[prost(message, optional, tag = "1")]
    pub key_range: ::core::option::Option<KeyRange>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreSafeTsResponse {
    #[prost(uint64, tag = "1")]
    pub safe_ts: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawGetKeyTtlRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub cf: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawGetKeyTtlResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    #[prost(string, tag = "2")]
    pub error: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub ttl: u64,
    #[prost(bool, tag = "4")]
    pub not_found: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawCasRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "4")]
    pub previous_not_exist: bool,
    #[prost(bytes = "vec", tag = "5")]
    pub previous_value: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "6")]
    pub cf: ::prost::alloc::string::String,
    #[prost(uint64, tag = "7")]
    pub ttl: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawCasResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    #[prost(string, tag = "2")]
    pub error: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub succeed: bool,
    /// The previous value regardless of whether the comparison is succeed.
    #[prost(bool, tag = "4")]
    pub previous_not_exist: bool,
    #[prost(bytes = "vec", tag = "5")]
    pub previous_value: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLockWaitInfoRequest {
    /// TODO: There may need some filter options to be used on conditional querying, e.g., finding
    /// the lock waiting status for some specified transaction.
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLockWaitInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    #[prost(string, tag = "2")]
    pub error: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub entries: ::prost::alloc::vec::Vec<super::deadlock::WaitForEntry>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawCoprocessorRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    #[prost(string, tag = "2")]
    pub copr_name: ::prost::alloc::string::String,
    /// Coprorcessor version constraint following SEMVER definition.
    #[prost(string, tag = "3")]
    pub copr_version_req: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub ranges: ::prost::alloc::vec::Vec<KeyRange>,
    #[prost(bytes = "vec", tag = "5")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawCoprocessorResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    /// Error message for cases like if no coprocessor with a matching name is found
    /// or on a version mismatch between plugin_api and the coprocessor.
    #[prost(string, tag = "2")]
    pub error: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawChecksumRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    #[prost(enumeration = "ChecksumAlgorithm", tag = "2")]
    pub algorithm: i32,
    #[prost(message, repeated, tag = "3")]
    pub ranges: ::prost::alloc::vec::Vec<KeyRange>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawChecksumResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    #[prost(string, tag = "2")]
    pub error: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub checksum: u64,
    #[prost(uint64, tag = "4")]
    pub total_kvs: u64,
    #[prost(uint64, tag = "5")]
    pub total_bytes: u64,
}
/// The API version the server and the client is using.
/// See more details in <https://github.com/tikv/rfcs/blob/master/text/0069-api-v2.md.>
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ApiVersion {
    /// Mainly for TxnKV and not safe to use RawKV along with TxnKV.
    ///
    /// V1 server only accepts V1 requests. Except that the V1 raw requests with TTL
    /// will be rejected.
    V1 = 0,
    /// Only RawKV is available, and then 8 bytes representing the unix timestamp in
    /// seconds for expiring time will be append to the value of all RawKV kv pairs.
    ///
    /// ---
    ///
    /// ## \| User value     | Expire Ts                               |
    ///
    /// ## \| 0x12 0x34 0x56 | 0x00 0x00 0x00 0x00 0x00 0x00 0xff 0xff |
    ///
    /// V1TTL server only accepts V1 raw requests.
    /// V1 client should not use `V1TTL` in request. V1 client should always send `V1`.
    V1ttl = 1,
    /// TxnKV keys start with `x{keyspace id}`, `m`, or `t`.
    ///
    /// RawKV keys must be in `default` CF and all start with `r{keyspace id}` prefix,
    /// where the keyspace id is in varint format (little endian), whose bytes expect
    /// the last one always sets the most significant bit to 1.
    ///
    /// The last byte in the raw value must be a meta flag. For example:
    ///
    /// ---
    ///
    /// ## \| User value     | Meta flags        |
    ///
    /// ## \| 0x12 0x34 0x56 | 0x00 (0b00000000) |
    ///
    /// As shown in the example below, the least significant bit of the meta flag
    /// indicates whether the value contains 8 bytes expire ts at the very left to the
    /// meta flags.
    ///
    /// ---
    ///
    /// ## \| User value     | Expire Ts                               | Meta flags        |
    ///
    /// ## \| 0x12 0x34 0x56 | 0x00 0x00 0x00 0x00 0x00 0x00 0xff 0xff | 0x01 (0b00000001) |
    ///
    /// V2 server accpets V2 requests and V1 txn requests that statrts with TiDB key
    /// prefix (`m` and `t`).
    V2 = 2,
}
impl ApiVersion {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ApiVersion::V1 => "V1",
            ApiVersion::V1ttl => "V1TTL",
            ApiVersion::V2 => "V2",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "V1" => Some(Self::V1),
            "V1TTL" => Some(Self::V1ttl),
            "V2" => Some(Self::V2),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CommandPri {
    /// Normal is the default value.
    Normal = 0,
    Low = 1,
    High = 2,
}
impl CommandPri {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CommandPri::Normal => "Normal",
            CommandPri::Low => "Low",
            CommandPri::High => "High",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Normal" => Some(Self::Normal),
            "Low" => Some(Self::Low),
            "High" => Some(Self::High),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IsolationLevel {
    /// SI = snapshot isolation
    Si = 0,
    /// RC = read committed
    Rc = 1,
}
impl IsolationLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IsolationLevel::Si => "SI",
            IsolationLevel::Rc => "RC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SI" => Some(Self::Si),
            "RC" => Some(Self::Rc),
            _ => None,
        }
    }
}
/// Operation allowed info during each TiKV storage threshold.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DiskFullOpt {
    /// The default value, means operations are not allowed either under almost full or already full.
    NotAllowedOnFull = 0,
    /// Means operations will be allowed when disk is almost full.
    AllowedOnAlmostFull = 1,
    /// Means operations will be allowed when disk is already full.
    AllowedOnAlreadyFull = 2,
}
impl DiskFullOpt {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DiskFullOpt::NotAllowedOnFull => "NotAllowedOnFull",
            DiskFullOpt::AllowedOnAlmostFull => "AllowedOnAlmostFull",
            DiskFullOpt::AllowedOnAlreadyFull => "AllowedOnAlreadyFull",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NotAllowedOnFull" => Some(Self::NotAllowedOnFull),
            "AllowedOnAlmostFull" => Some(Self::AllowedOnAlmostFull),
            "AllowedOnAlreadyFull" => Some(Self::AllowedOnAlreadyFull),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Op {
    Put = 0,
    Del = 1,
    Lock = 2,
    Rollback = 3,
    /// insert operation has a constraint that key should not exist before.
    Insert = 4,
    PessimisticLock = 5,
    CheckNotExists = 6,
}
impl Op {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Op::Put => "Put",
            Op::Del => "Del",
            Op::Lock => "Lock",
            Op::Rollback => "Rollback",
            Op::Insert => "Insert",
            Op::PessimisticLock => "PessimisticLock",
            Op::CheckNotExists => "CheckNotExists",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Put" => Some(Self::Put),
            "Del" => Some(Self::Del),
            "Lock" => Some(Self::Lock),
            "Rollback" => Some(Self::Rollback),
            "Insert" => Some(Self::Insert),
            "PessimisticLock" => Some(Self::PessimisticLock),
            "CheckNotExists" => Some(Self::CheckNotExists),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Assertion {
    None = 0,
    Exist = 1,
    NotExist = 2,
}
impl Assertion {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Assertion::None => "None",
            Assertion::Exist => "Exist",
            Assertion::NotExist => "NotExist",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "None" => Some(Self::None),
            "Exist" => Some(Self::Exist),
            "NotExist" => Some(Self::NotExist),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AssertionLevel {
    /// No assertion.
    Off = 0,
    /// Assertion is enabled, but not enforced when it might affect performance.
    Fast = 1,
    /// Assertion is enabled and enforced.
    Strict = 2,
}
impl AssertionLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AssertionLevel::Off => "Off",
            AssertionLevel::Fast => "Fast",
            AssertionLevel::Strict => "Strict",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Off" => Some(Self::Off),
            "Fast" => Some(Self::Fast),
            "Strict" => Some(Self::Strict),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Action {
    NoAction = 0,
    TtlExpireRollback = 1,
    LockNotExistRollback = 2,
    MinCommitTsPushed = 3,
    TtlExpirePessimisticRollback = 4,
    LockNotExistDoNothing = 5,
}
impl Action {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Action::NoAction => "NoAction",
            Action::TtlExpireRollback => "TTLExpireRollback",
            Action::LockNotExistRollback => "LockNotExistRollback",
            Action::MinCommitTsPushed => "MinCommitTSPushed",
            Action::TtlExpirePessimisticRollback => "TTLExpirePessimisticRollback",
            Action::LockNotExistDoNothing => "LockNotExistDoNothing",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NoAction" => Some(Self::NoAction),
            "TTLExpireRollback" => Some(Self::TtlExpireRollback),
            "LockNotExistRollback" => Some(Self::LockNotExistRollback),
            "MinCommitTSPushed" => Some(Self::MinCommitTsPushed),
            "TTLExpirePessimisticRollback" => Some(Self::TtlExpirePessimisticRollback),
            "LockNotExistDoNothing" => Some(Self::LockNotExistDoNothing),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExtraOp {
    Noop = 0,
    /// ReadOldValue represents to output the previous value for delete/update operations.
    ReadOldValue = 1,
}
impl ExtraOp {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExtraOp::Noop => "Noop",
            ExtraOp::ReadOldValue => "ReadOldValue",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Noop" => Some(Self::Noop),
            "ReadOldValue" => Some(Self::ReadOldValue),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChecksumAlgorithm {
    Crc64Xor = 0,
}
impl ChecksumAlgorithm {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ChecksumAlgorithm::Crc64Xor => "Crc64_Xor",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Crc64_Xor" => Some(Self::Crc64Xor),
            _ => None,
        }
    }
}
