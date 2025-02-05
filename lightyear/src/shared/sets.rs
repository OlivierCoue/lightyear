//! Bevy [`SystemSet`] that are shared between the server and client
use bevy::prelude::SystemSet;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct ClientMarker;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct ServerMarker;

/// System sets related to Replication
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum InternalReplicationSet<M> {
    // RECEIVE
    /// System that copies the resource data from the entity to the resource in the receiving world
    ReceiveResourceUpdates,

    // SEND
    /// Set the hash for each entity that is pre-spawned on the client
    /// (has a PreSpawnedPlayerObject component)
    SetPreSpawnedHash,
    /// System that handles the addition/removal of the `Replicate` component
    BeforeBuffer,
    /// Gathers entity despawns and component removals
    /// Needs to run once per frame instead of once per send_interval
    /// because they rely on bevy events that are cleared every frame
    BufferDespawnsAndRemovals,

    /// System Set to gather all the replication updates to send
    /// These systems only run once every send_interval
    BufferEntityUpdates,
    BufferComponentUpdates,
    BufferResourceUpdates,

    /// All systems that buffer replication messages
    Buffer,
    /// System that handles the update of an existing replication component
    AfterBuffer,
    /// SystemSet where we actually buffer the replication messages.
    /// Runs every send_interval, not every frame
    SendMessages,
    /// SystemSet that encompasses all send replication systems
    All,
    _Marker(std::marker::PhantomData<M>),
    SendMessage,
    EmitEvents,
}

/// Main SystemSets used by lightyear to receive and send data
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub(crate) enum InternalMainSet<M> {
    /// Systems that receives data from the remote peers
    /// Runs in `PreUpdate`.
    Receive,
    /// Systems that writes networking events (ReceiveMessage, ConnectionEvent, etc.)
    /// Runs in `PreUpdate`, after `Receive`
    ReceiveEvents,

    /// Systems that reads networking events (SendMessage) and buffers them
    /// so that they can be sent over the network.
    /// Runs in `PostUpdate`, before `Send`
    SendEvents,
    /// SystemSet where we actually send packets over the network.
    ///
    /// Runs in `PostUpdate`
    Send,
    _Marker(std::marker::PhantomData<M>),
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum MainSet {
    /// Systems that receives data from the remote peers
    /// Runs in `PreUpdate`.
    Receive,
    /// Systems that writes networking events (ReceiveMessage, ConnectionEvent, etc.)
    /// Runs in `PreUpdate`, after `Receive`
    ReceiveEvents,

    /// Systems that reads networking events (SendMessage) and buffers them
    /// so that they can be sent over the network.
    /// Runs in `PostUpdate`, before `Send`
    SendEvents,
    /// SystemSet where we actually send packets over the network.
    ///
    /// Runs in `PostUpdate`
    Send,
}

/// SystemSet that run during the FixedUpdate schedule
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum FixedUpdateSet {
    /// System that runs in the FixedFirst schedule to increment the ticks
    TickUpdate,
}
