use crate::{DirectionalLight, PointLight, SpecializedMaterial, StandardMaterial};
use bevy_asset::Handle;
use bevy_ecs::{bundle::Bundle, component::Component};
use bevy_render::{
    mesh::Mesh,
    primitives::{CubemapFrusta, Frustum},
    view::{ComputedVisibility, Visibility, VisibleEntities},
};
use bevy_transform::components::{GlobalTransform, Transform};

/// A component bundle for PBR entities with a [`Mesh`] and a [`StandardMaterial`].
pub type PbrBundle = MaterialMeshBundle<StandardMaterial>;

/// A component bundle for entities with a [`Mesh`] and a [`SpecializedMaterial`].
#[derive(Bundle, Clone)]
pub struct MaterialMeshBundle<M: SpecializedMaterial> {
    pub mesh: Handle<Mesh>,
    pub material: Handle<M>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub computed_visibility: ComputedVisibility,
}

impl<M: SpecializedMaterial> Default for MaterialMeshBundle<M> {
    fn default() -> Self {
        Self {
            mesh: Default::default(),
            material: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
        }
    }
}

#[derive(Component, Clone, Debug, Default)]
pub struct CubemapVisibleEntities {
    data: [VisibleEntities; 6],
}

impl CubemapVisibleEntities {
    pub fn get(&self, i: usize) -> &VisibleEntities {
        &self.data[i]
    }

    pub fn get_mut(&mut self, i: usize) -> &mut VisibleEntities {
        &mut self.data[i]
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &VisibleEntities> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut VisibleEntities> {
        self.data.iter_mut()
    }
}

/// A component bundle for [`PointLight`] entities.
#[derive(Debug, Bundle, Default)]
pub struct PointLightBundle {
    pub point_light: PointLight,
    pub cubemap_visible_entities: CubemapVisibleEntities,
    pub cubemap_frusta: CubemapFrusta,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

/// A component bundle for [`DirectionalLight`] entities.
#[derive(Debug, Bundle, Default)]
pub struct DirectionalLightBundle {
    pub directional_light: DirectionalLight,
    pub frustum: Frustum,
    pub visible_entities: VisibleEntities,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}
