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
use godot::engine::WorkerThreadPool;
use godot::obj::WithBaseField;
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
use crate::voxel_storage::VoxelWorld;
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
            self.base_mut().add_tool_menu_item(
                "simulate_water".into(),
                Callable::from_object_method(&n, "simulate_water_step"),
            );
            self.gen = Some(n);
            godot_print!("enter");
        }
    }

    fn exit_tree(&mut self) {
        self.base_mut().remove_tool_menu_item("GenerateMesh".into());
        self.base_mut()
            .remove_tool_menu_item("simulate_water".into());
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
        if let Some(parent) = EditorInterface::singleton().get_edited_scene_root() {
            let mut world_node = World::new_alloc();
            world_node.add_to_group("world".into());
            parent.clone().add_child(world_node.clone().upcast());
            world_node.set_owner(parent.clone());
            let children = world_node.call("initialize".into(), &[]);
            let children: Array<Gd<Node>> = children.to();
            for mut child in children.iter_shared() {
                world_node.clone().add_child(child.clone());
                child.set_owner(parent.clone());
            }
        }
    }

    #[func]
    fn simulate_water_step(&self) {
        if let Some(parent) = EditorInterface::singleton().get_edited_scene_root() {
            let children = parent.get_children();
            for mut child in children.iter_shared() {
                if child.is_in_group("world".into()) {
                    child.call("simulate_step".into(), &[]);
                }
            }
        }
    }
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

#[derive(GodotClass)]
#[class(base=Node3D)]
struct World {
    base: Base<Node3D>,
    voxels: VoxelWorld,
}

#[godot_api]
impl INode3D for World {
    fn init(base: Base<Node3D>) -> Self {
        let mut world = VoxelWorld::gen(-2..2, -2..2);
        for i in 0..128u8 {
            simulate_water(&mut world, i as u8);
        }
        World {
            base,
            voxels: world,
        }
    }
}

#[godot_api]
impl World {
    #[func]
    fn initialize(&mut self) -> Array<Gd<Node>> {
        let mut r = Array::new();
        for (coord, s) in self.voxels.ground.iter() {
            let w = &self.voxels.water[coord];
            let ground = create_ground_mesh(
                Vector3::new(coord[0] as f32 * 64.0, 0.0, coord[1] as f32 * 64.0),
                s,
            );
            r.push(ground);
            // p.add_child(ground.clone());
            // ground.set_owner(p.upcast());
            let mut water = create_water_mesh(
                Vector3::new(coord[0] as f32 * 64.0, 0.0, coord[1] as f32 * 64.0),
                w,
            );
            water.add_to_group("Water".into());
            r.push(water);
        }
        r
    }

    #[func]
    fn simulate_step(&mut self) {
        simulate_water(&mut self.voxels, 0);
        let children = self.base().get_children();
        for child in children.iter_shared() {
            if child.is_in_group("water".into()) {
                godot_print!("found water node");
            }
        }
    }
}
