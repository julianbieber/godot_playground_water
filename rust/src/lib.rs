mod voxel_mesh;
mod voxel_storage;

use godot::engine::EditorInterface;
use godot::engine::Engine;
use godot::engine::GeometryInstance3D;
use godot::engine::Material;
use godot::engine::MeshInstance3D;
use godot::engine::ResourceFormatLoader;
use godot::engine::ResourceImporter;
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

use crate::voxel_storage::VoxelStorage;

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
        if let Some(mut parent) = EditorInterface::singleton().get_edited_scene_root() {
            let (mut mesh, storage) = create_mesh();
            parent.add_child(mesh.clone());
            mesh.set_owner(parent);
            godot_print!("attached node");
        }
    }

    #[func]
    fn clear(&self) {}
}

fn create_mesh() -> (Gd<Node>, voxel_storage::VoxelStorage) {
    let mut storage = VoxelStorage::empty();
    sphere(&mut storage);
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
    (instance.upcast(), storage)
}

fn sphere(storage: &mut VoxelStorage) {
    for x in 0..64 {
        for y in 0..64 {
            for z in 0..64 {
                if x + y + z < 32 {
                    storage.set([x, y, z]);
                }
            }
        }
    }
}
