#version 410 core

layout (location = 0) in vec3 aPos;

out vec3 TexCoords;

uniform mat4 perspective;
uniform mat4 camera;

void main()
{
    TexCoords = aPos;
    gl_Position = perspective * camera * vec4(aPos, 1.0);
}  