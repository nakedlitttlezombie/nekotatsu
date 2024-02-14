/// DirEntry("NekoModels/BackupHistory.kt")
/// Manually fixed; I don't know why it starts at 0 in the original source
/// Should not be a big deal though
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BrokenBackupHistory {
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub last_read: i64,
    #[prost(int64, tag = "3")]
    pub read_duration: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupHistory {
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub last_read: i64,
    #[prost(int64, tag = "3")]
    pub read_duration: i64,
}
/// DirEntry("NekoModels/BackupTracking.kt")
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupTracking {
    #[prost(int32, tag = "1")]
    pub sync_id: i32,
    #[prost(int64, tag = "2")]
    pub library_id: i64,
    #[prost(int32, tag = "3")]
    pub media_id_int: i32,
    #[prost(string, tag = "4")]
    pub tracking_url: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub title: ::prost::alloc::string::String,
    #[prost(float, tag = "6")]
    pub last_chapter_read: f32,
    #[prost(int32, tag = "7")]
    pub total_chapters: i32,
    #[prost(float, tag = "8")]
    pub score: f32,
    #[prost(int32, tag = "9")]
    pub status: i32,
    #[prost(int64, tag = "10")]
    pub started_reading_date: i64,
    #[prost(int64, tag = "11")]
    pub finished_reading_date: i64,
    #[prost(int64, tag = "100")]
    pub media_id: i64,
}
/// DirEntry("NekoModels/BackupManga.kt")
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupManga {
    #[prost(int64, tag = "1")]
    pub source: i64,
    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub artist: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub author: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "7")]
    pub genre: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int32, tag = "8")]
    pub status: i32,
    #[prost(string, tag = "9")]
    pub thumbnail_url: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub custom_cover: ::prost::alloc::string::String,
    #[prost(int64, tag = "11")]
    pub last_update: i64,
    #[prost(int64, tag = "12")]
    pub last_init: i64,
    #[prost(int64, tag = "13")]
    pub date_added: i64,
    #[prost(int32, tag = "14")]
    pub viewer: i32,
    #[prost(int32, tag = "15")]
    pub flags: i32,
    #[prost(message, repeated, tag = "16")]
    pub chapters: ::prost::alloc::vec::Vec<BackupChapter>,
    #[prost(int32, repeated, tag = "17")]
    pub categories: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, repeated, tag = "18")]
    pub tracking: ::prost::alloc::vec::Vec<BackupTracking>,
    #[prost(bool, tag = "100")]
    pub favorite: bool,
    #[prost(int32, tag = "101")]
    pub chapter_flags: i32,
    #[prost(message, repeated, tag = "102")]
    pub broken_history: ::prost::alloc::vec::Vec<BrokenBackupHistory>,
    #[prost(int32, tag = "103")]
    pub viewer_flags: i32,
    #[prost(message, repeated, tag = "104")]
    pub history: ::prost::alloc::vec::Vec<BackupHistory>,
    #[prost(string, tag = "800")]
    pub custom_title: ::prost::alloc::string::String,
    #[prost(string, tag = "900")]
    pub merged_manga_url: ::prost::alloc::string::String,
    #[prost(string, tag = "901")]
    pub scanlator_filter: ::prost::alloc::string::String,
    #[prost(string, tag = "902")]
    pub merged_manga_image_url: ::prost::alloc::string::String,
    #[prost(string, tag = "903")]
    pub alternative_artwork: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "904")]
    pub merge_manga_list: ::prost::alloc::vec::Vec<BackupMergeManga>,
}
/// DirEntry("NekoModels/BackupMergeManga.kt")
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupMergeManga {
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub cover_url: ::prost::alloc::string::String,
    #[prost(int32, tag = "4")]
    pub merge_type: i32,
}
/// DirEntry("NekoModels/BackupChapter.kt")
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupChapter {
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub scanlator: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub read: bool,
    #[prost(bool, tag = "5")]
    pub bookmark: bool,
    #[prost(int32, tag = "6")]
    pub last_page_read: i32,
    #[prost(int64, tag = "7")]
    pub date_fetch: i64,
    #[prost(int64, tag = "8")]
    pub date_upload: i64,
    #[prost(float, tag = "9")]
    pub chapter_number: f32,
    #[prost(int32, tag = "10")]
    pub source_order: i32,
    #[prost(int32, tag = "800")]
    pub pages_left: i32,
}
/// DirEntry("NekoModels/BackupCategory.kt")
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupCategory {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub order: i32,
    #[prost(int32, tag = "3")]
    pub update_interval: i32,
    #[prost(int32, tag = "100")]
    pub flags: i32,
}
/// DirEntry("NekoModels/Backup.kt")
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Backup {
    #[prost(message, repeated, tag = "1")]
    pub backup_manga: ::prost::alloc::vec::Vec<BackupManga>,
    #[prost(message, repeated, tag = "2")]
    pub backup_categories: ::prost::alloc::vec::Vec<BackupCategory>,
}
