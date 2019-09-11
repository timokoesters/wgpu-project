#version 450

out gl_PerVertex {
    vec4 gl_Position;
};

layout(set = 0, binding = 0) buffer Particles {
    vec3[] positions;
    vec3[] velocities;
};

void main() {
    gl_Position = vec4(positions[gl_VertexIndex], 1.0);
}
