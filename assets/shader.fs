#version 330

in vec3 vertexColor;  // Color passed from the vertex shader
out vec4 FragColor;   // Final color output

void main()
{
    vec3 normalizedColor = vertexColor * 0.5 + 0.5;

    FragColor = vec4(normalizedColor.r, normalizedColor.g * 0.7, normalizedColor.b * 0.9, 1.0);  // Use the passed color, with full opacity
}
