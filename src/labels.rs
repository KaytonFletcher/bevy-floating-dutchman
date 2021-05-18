use bevy::prelude::StageLabel;
#[derive(Debug, Clone, PartialEq, Eq, Hash, StageLabel)]
pub enum CustomStages {
    Physics,
    Debug,
}
