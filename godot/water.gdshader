shader_type spatial;

varying float height;

void vertex() {
	height = VERTEX.y;
	// Called for every vertex the material is visible on.
}

void fragment() {
	ALBEDO = vec3(0.0, 0.0, 1.0);
		
	
	
}

//void light() {
	// Called for every pixel for every light affecting the material.
	// Uncomment to replace the default light processing function with this one.
//}
