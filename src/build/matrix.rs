use helpers::Class;

use super::{Artifact, Build, BuildStatus, ShortBuild};
use action::CommonAction;
use changeset;
use user::ShortUser;

build_with_common_fields_and_impl!(/// A `Build` from a MatrixProject
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MatrixBuild {
    /// Change set for this build
    pub change_set: changeset::CommonChangeSetList,
    /// Runs of each configuration
    pub runs: Vec<ShortBuild>,
    /// List of user ids who made a change since the last non-broken build
    pub culprits: Vec<ShortUser>,
});
register_class!("hudson.matrix.MatrixBuild" => MatrixBuild);

impl MatrixBuild {}

build_with_common_fields_and_impl!(/// A `Build` from a MatrixProject
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MatrixRun {
    /// Change set for this build
    pub change_set: changeset::CommonChangeSetList,
    /// List of user ids who made a change since the last non-broken build
    pub culprits: Vec<ShortUser>,
});
register_class!("hudson.matrix.MatriRun" => MatrixRun);

impl MatrixRun {}
