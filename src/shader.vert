#version 450

out gl_PerVertex {
    vec4 gl_Position;
};

layout(set = 0, binding = 0) buffer PrimeIndices {
    vec3[] positions;
};

void main() {
    gl_Position = vec4(positions[0], 1.0);
}
