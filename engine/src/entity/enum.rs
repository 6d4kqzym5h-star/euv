use crate::*;

/// Events that can be emitted by entities for inter-entity communication.
///
/// The event system allows decoupled game logic by enabling entities to
/// broadcast events that other entities or systems can subscribe to.
#[derive(Clone, Debug)]
pub enum EntityEvent {
    /// Emitted when the entity collides with another entity.
    Collision {
        /// The ID of the other entity involved in the collision.
        other_id: u64,
        /// The collision normal pointing from this entity to the other.
        normal: Vector2D,
        /// The penetration depth of the collision.
        depth: f64,
    },
    /// Emitted when the entity enters a trigger zone identified by a tag.
    TriggerEnter {
        /// The tag of the trigger zone that was entered.
        tag: String,
    },
    /// Emitted when the entity exits a trigger zone identified by a tag.
    TriggerExit {
        /// The tag of the trigger zone that was exited.
        tag: String,
    },
    /// Emitted when the entity is spawned into the scene.
    Spawn,
    /// Emitted when the entity is destroyed and removed from the scene.
    Destroy,
    /// A custom event with an arbitrary name and optional string data.
    Custom {
        /// The name identifying the custom event.
        name: String,
        /// Optional string payload carried by the event.
        data: String,
    },
}
