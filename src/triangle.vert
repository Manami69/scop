#version 410 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 aColor;
layout (location = 2) in vec2 aTexCoord;
out vec4 vertexColor; // specify a color output to the fragment shader
out vec2 TexCoord;

uniform mat4 transform;
uniform mat4 perspective;

void main()
{
    gl_Position = perspective * transform * vec4(Position, 1.0);
    vertexColor = vec4(aColor, 1.0); // set the output variable to a dark-red color
    TexCoord = aTexCoord;
}