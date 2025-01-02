#version 330

layout (location = 0) in vec3 aPos;  // Vertex position
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aTexCoords;

uniform mat4 model;     // Model matrix
uniform mat4 view;      // View matrix
uniform mat4 projection; // Projection matrix

out vec3 vertexColor;   // Output color to fragment shader

void main()
{
    // Apply model, view, and projection transformations
    gl_Position = projection * view * model * vec4(aPos, 1.0);
//    gl_Position = vec4(aPos,1.0);
    vertexColor = aPos; // Pass color to fragment shader
}
