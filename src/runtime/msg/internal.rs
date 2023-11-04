use url::Url;

use crate::models::ctx::CtxError;
use crate::models::link::LinkError;
use crate::models::local_search::Searchable;
use crate::models::streaming_server::{
    PlaybackDevice, Settings as StreamingServerSettings, StatisticsRequest,
};
use crate::runtime::EnvError;
use crate::types::addon::{Descriptor, Manifest, ResourceRequest, ResourceResponse};
use crate::types::api::{
    APIRequest, AuthRequest, DataExportResponse, DatastoreRequest, LinkCodeResponse,
    LinkDataResponse,
};
use crate::types::library::{LibraryBucket, LibraryItem, LibraryItemId};
use crate::types::profile::{Auth, AuthKey, Profile, User};
use crate::types::resource::Stream;
use crate::types::streaming_server::Statistics;
use crate::types::streams::StreamItemState;

pub type CtxStorageResponse = (
    Option<Profile>,
    Option<LibraryBucket>,
    Option<LibraryBucket>,
);

pub type AuthResponse = (Auth, Vec<Descriptor>, Vec<LibraryItem>);

pub type LibraryPlanResponse = (Vec<String>, Vec<String>);

//
// Those messages are meant to be dispatched and handled only inside stremio-core crate
//
#[derive(Debug)]
pub enum Internal {
    /// Result for authenticate to API.
    CtxAuthResult(AuthRequest, Result<AuthResponse, CtxError>),
    /// Result for pull addons from API.
    AddonsAPIResult(APIRequest, Result<Vec<Descriptor>, CtxError>),
    /// Result for pull user from API.
    UserAPIResult(APIRequest, Result<User, CtxError>),
    /// Result for library sync plan with API.
    LibrarySyncPlanResult(DatastoreRequest, Result<LibraryPlanResponse, CtxError>),
    /// Result for pull library items from API.
    LibraryPullResult(DatastoreRequest, Result<Vec<LibraryItem>, CtxError>),
    /// Dispatched when expired session is detected
    Logout,
    /// Internal event dispatched on user action or login
    /// to install the addon if it's not present
    InstallTraktAddon,
    /// Dispatched when addons needs to be installed.
    InstallAddon(Descriptor),
    /// Dispatched when addons needs to be uninstalled.
    UninstallAddon(Descriptor),
    UninstallTraktAddon,
    /// Dispatched when a new stream is loaded into the Player.
    StreamLoaded {
        stream: Stream,
        stream_request: Option<ResourceRequest>,
        meta_request: Option<ResourceRequest>,
    },
    /// Dispatched when stream item's state has changed
    StreamStateChanged {
        state: Option<StreamItemState>,
        stream_request: Option<ResourceRequest>,
        meta_request: Option<ResourceRequest>,
    },
    /// Dispatched when library item needs to be updated in the memory, storage and API.
    UpdateLibraryItem(LibraryItem),
    /// Dispatched when some of auth, addons or settings changed.
    ProfileChanged,
    /// Dispatched when library changes with a flag if its already persisted.
    LibraryChanged(bool),
    /// Dispatched when streams bucket changes with a flag if its already persisted.
    StreamsChanged(bool),
    /// User notifications have changed
    NotificationsChanged,
    /// Dismiss all Notifications for a given [`MetaItemId`].
    DismissNotificationItem(LibraryItemId),
    /// Result for loading link code.
    LinkCodeResult(Result<LinkCodeResponse, LinkError>),
    /// Result for loading link data.
    LinkDataResult(String, Result<LinkDataResponse, LinkError>),
    /// Result for loading streaming server settings.
    StreamingServerSettingsResult(Url, Result<StreamingServerSettings, EnvError>),
    /// Result for loading streaming server base url.
    StreamingServerBaseURLResult(Url, Result<Url, EnvError>),
    // Result for loading streaming server playback devices.
    StreamingServerPlaybackDevicesResult(Url, Result<Vec<PlaybackDevice>, EnvError>),
    /// Result for updating streaming server settings.
    StreamingServerUpdateSettingsResult(Url, Result<(), EnvError>),
    /// Result for creating a torrent.
    StreamingServerCreateTorrentResult(String, Result<(), EnvError>),
    // Result for playing on device.
    StreamingServerPlayOnDeviceResult(String, Result<(), EnvError>),
    // Result for streaming server statistics.
    StreamingServerStatisticsResult((Url, StatisticsRequest), Result<Statistics, EnvError>),
    /// Result for fetching resource from addons.
    ResourceRequestResult(ResourceRequest, Box<Result<ResourceResponse, EnvError>>),
    /// Result for fetching manifest from addon.
    ManifestRequestResult(Url, Result<Manifest, EnvError>),
    /// TODO: write some obvious comment about what it is
    NotificationsRequestResult(ResourceRequest, Box<Result<ResourceResponse, EnvError>>),
    /// Result for requesting a `dataExport` of user data.
    DataExportResult(AuthKey, Result<DataExportResponse, CtxError>),
    /// The result of querying the data for LocalSearch
    LoadLocalSearchResult(Url, Result<Vec<Searchable>, EnvError>),
}
