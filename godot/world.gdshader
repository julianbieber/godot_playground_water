shader_type spatial;

void vertex() {
	// Called for every vertex the material is visible on.
}

void fragment() {
	
	ALBEDO = vec3(0.1, UV.x* 0.01, UV.y* 0.01);
	
	
}

//void light() {
	// Called for every pixel for every light affecting the material.
	// Uncomment to replace the default light processing function with this one.
//}
