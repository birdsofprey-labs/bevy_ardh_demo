use std::{collections::VecDeque, hash::Hash};

mod camera_controller;
use bevy_ardh::{ardh::{ArdhFlat, QT, TileId}, quadtree::ZNodeIndex};
use bevy::{prelude::*, utils::HashSet, render::{RenderPlugin, settings::{WgpuSettings, WgpuFeatures, RenderCreation}, render_resource::{AsBindGroup, ShaderRef, TextureDescriptor, AddressMode, SamplerDescriptor}, primitives::Aabb, texture::{ImageSampler, ImageSamplerDescriptor, ImageAddressMode, ImageLoaderSettings}}, pbr::{wireframe::{WireframePlugin, WireframeConfig, WireframeColor, Wireframe}, ExtendedMaterial, MaterialExtension, OpaqueRendererMethod, ScreenSpaceAmbientOcclusionBundle, CascadeShadowConfigBuilder}, reflect::{TypeUuid, TypePath}, core_pipeline::experimental::taa::TemporalAntiAliasPlugin, ecs::query::WorldQuery};
//use bevy_infinite_grid::{GridShadowCamera, InfiniteGrid, InfiniteGridBundle, InfiniteGridPlugin};
use camera_controller::{CameraController, CameraControllerPlugin};


fn main() {
    App::new()
    .insert_resource(Msaa::Sample4)
    
    .add_plugins(
        DefaultPlugins.set(AssetPlugin { file_path: "../assets".to_string(), ..default() }))
    .insert_resource(WireframeConfig {
        global: false,
        default_color: Color::WHITE,
    })
    .add_plugins((bevy_ardh::ArdhPlugin, CameraControllerPlugin))
    .add_systems(Startup, setup_system)
    .run();
}



    

fn setup_system(
    //mut wireframe_config: ResMut<WireframeConfig>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    //mut materials: ResMut<Assets<CustomMaterial>>,
) {


    commands.insert_resource(ClearColor(Color::rgb(0.4627450980392157, 0.6352941176470588, 0.9098039215686274)));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.03,
    });

    

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 2500.0, 0.0),
            ..default()
        },
        CameraController::default(),
        FogSettings {
            color: Color::rgba(0.25, 0.25, 0.35, 1.0),
            falloff:FogFalloff::from_visibility_color(3500.0, Color::WHITE), //FogFalloff::Atmospheric { extinction: Vec3::splat(0.0075), inscattering: Vec3::splat(0.01) },
            ..default()
        },
       // GridShadowCamera,
       //paceAmbientOcclusionBundle::default()
    ));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 85000.0,
            shadows_enabled: true,
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        transform: Transform::from_translation(Vec3::Y * 100. + Vec3::X * 50. + Vec3::Z * 20.)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });



    let mat = standard_materials.add(StandardMaterial::default());

   

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "This text changes in the bottom right",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "Count",
                TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ),
        ]),
        bevy_ardh::TextChanges,
    ));

    commands.spawn(bevy_ardh::SphericalArdh { base_radius: 1000.0, tile_mesh_type: bevy_ardh::TileMeshType::WithSkirts, enqueue_creation: true });
}

