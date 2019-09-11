#version 450

out gl_PerVertex {
    vec4 gl_Position;
};

struct Particle {
    vec3 position;
    vec3 velocity;
};

layout(set = 0, binding = 0) buffer Data {
    Particle data[];
};

void main() {
    data[gl_VertexIndex].position += data[gl_VertexIndex].velocity;
    gl_Position = vec4(data[gl_VertexIndex].position, 1.0);
}
