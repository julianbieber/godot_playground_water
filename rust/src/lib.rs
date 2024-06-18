mod voxel_mesh;
mod voxel_storage;
mod water_sim;

use godot::engine::EditorInterface;
use godot::engine::Engine;
use godot::engine::GeometryInstance3D;
use godot::engine::MeshInstance3D;
use godot::engine::ResourceLoader;
use godot::engine::Shader;
use godot::engine::ShaderMaterial;
use godot::prelude::*;

struct MyExtension;
#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

use godot::engine::EditorPlugin;

#[derive(GodotClass)]
#[class(tool, init, editor_plugin, base=EditorPlugin)]
struct WorldGen {
    base: Base<EditorPlugin>,
    #[export]
    name: GString,
    gen: Option<Gd<GenMeshNode>>,
}

use godot::engine::ArrayMesh;
use godot::engine::IEditorPlugin;
use godot::obj::Gd;

use crate::voxel_storage::Chunks;
use crate::voxel_storage::VoxelStorage;
use crate::water_sim::simulate_water;

#[godot_api]
impl IEditorPlugin for WorldGen {
    fn enter_tree(&mut self) {
        if Engine::singleton().is_editor_hint() {
            let n = GenMeshNode::new_alloc();
            self.base_mut().add_tool_menu_item(
                "GenerateMesh".into(),
                Callable::from_object_method(&n, "gen"),
            );
            self.gen = Some(n);
            godot_print!("enter");
        }
    }

    fn exit_tree(&mut self) {
        self.base_mut().remove_tool_menu_item("GenerateMesh".into());
        if let Some(v) = self.gen.clone() {
            v.free();
        }
        godot_print!("exit");
    }
}

#[derive(GodotClass)]
#[class(tool, init, base=Node)]
struct GenMeshNode {
    base: Base<Node>,
}

#[godot_api]
impl GenMeshNode {
    #[func]
    fn gen(&mut self) {
        godot_print!("call");
        if let Some(parent) = EditorInterface::singleton().get_edited_scene_root() {
            let mut chunks = Chunks::gen(-2..2, -2..2);
            for _ in 0..64 {
                simulate_water(&mut chunks);
            }
            for (coord, s) in chunks.ground.iter() {
                let w = &chunks.water[coord];
                let mut ground = create_ground_mesh(
                    Vector3::new(coord[0] as f32 * 64.0, 0.0, coord[1] as f32 * 64.0),
                    s,
                );
                let p = parent.clone();
                parent.clone().add_child(ground.clone());
                ground.set_owner(p);
                let mut water = create_water_mesh(
                    Vector3::new(coord[0] as f32 * 64.0, 0.0, coord[1] as f32 * 64.0),
                    w,
                );
                let p = parent.clone();
                parent.clone().add_child(water.clone());
                water.set_owner(p);
                godot_print!("attached node");
            }
        }
    }

    #[func]
    fn clear(&self) {}
}

fn create_ground_mesh(p: Vector3, storage: &VoxelStorage) -> Gd<Node> {
    let mesh: Gd<ArrayMesh> = voxel_mesh::blocky(&storage.visible_faces());
    let mut instance = MeshInstance3D::new_alloc();
    instance.set_mesh(mesh.upcast());
    let mut geo = instance.clone().upcast::<GeometryInstance3D>();
    let mut sh = ShaderMaterial::new_gd();
    let shader: Gd<Shader> = ResourceLoader::load(
        &mut ResourceLoader::singleton(),
        "res://world.gdshader".into_godot(),
    )
    .unwrap()
    .cast();
    sh.set_shader(shader);
    geo.set_material_override(sh.upcast());
    let mut transform: Gd<Node3D> = instance.clone().upcast();
    transform.set_position(p);
    instance.upcast()
}
fn create_water_mesh(p: Vector3, storage: &VoxelStorage) -> Gd<Node> {
    let mesh: Gd<ArrayMesh> = voxel_mesh::blocky(&storage.visible_faces());
    let mut instance = MeshInstance3D::new_alloc();
    instance.set_mesh(mesh.upcast());
    let mut geo = instance.clone().upcast::<GeometryInstance3D>();
    let mut sh = ShaderMaterial::new_gd();
    let shader: Gd<Shader> = ResourceLoader::load(
        &mut ResourceLoader::singleton(),
        "res://water.gdshader".into_godot(),
    )
    .unwrap()
    .cast();
    sh.set_shader(shader);
    geo.set_material_override(sh.upcast());
    geo.set_transparency(0.5);
    let mut transform: Gd<Node3D> = instance.clone().upcast();
    transform.set_position(p);
    instance.upcast()
}
