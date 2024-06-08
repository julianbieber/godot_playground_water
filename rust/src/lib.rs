mod voxel_storage;
use godot::engine::mesh::PrimitiveType;
use godot::engine::EditorInterface;
use godot::engine::Engine;
use godot::engine::MeshInstance3D;
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

use godot::engine::mesh;
use godot::engine::ArrayMesh;
use godot::engine::IEditorPlugin;
use godot::obj::Gd;

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
    fn gen(&self) {
        godot_print!("call");
        if let Some(mut parent) = EditorInterface::singleton().get_edited_scene_root() {
            let mut mesh = create_mesh();
            parent.add_child(mesh.clone());
            mesh.set_owner(parent);
            godot_print!("attached node");
        }
    }

    #[func]
    fn clear(&self) {}
}

fn create_mesh() -> Gd<Node> {
    let mut mesh = ArrayMesh::new_gd();
    let mut array = VariantArray::new();
    array.resize(mesh::ArrayType::MAX.ord() as usize, &Variant::nil());
    // let (positions, indices) = create_voxels();
    // array.set(
    //     mesh::ArrayType::VERTEX.ord() as usize,
    //     positions.to_variant(),
    // );
    // array.set(mesh::ArrayType::INDEX.ord() as usize, indices.to_variant());
    // mesh.add_surface_from_arrays(PrimitiveType::TRIANGLES, array);
    // let mut mesh_instance = MeshInstance3D::new_alloc();
    // mesh_instance.set_mesh(mesh.upcast());
    // mesh_instance.upcast()
    todo!()
}

// This chunk will cover just a single octant of a sphere SDF (radius 15).
// Some quads were generated.
