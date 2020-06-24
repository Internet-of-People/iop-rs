mod plugin;
mod private;
mod private_kind;
mod public;
mod public_kind;

pub use plugin::*;
pub use private::*;
pub use private_kind::*;
pub use public::*;
pub use public_kind::*;

use super::*;

type MorpheusPublic = hd_morpheus::Public;
type MorpheusPrivate = hd_morpheus::Private;
type MorpheusPrivateKind = hd_morpheus::PrivateKind;
type MorpheusPublicKind = hd_morpheus::PublicKind;
type MorpheusBoundPlugin = BoundPlugin<hd_morpheus::Plugin, MorpheusPublic, MorpheusPrivate>;
