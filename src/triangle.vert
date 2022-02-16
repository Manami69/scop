#version 410 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 aColor;
out vec4 vertexColor; // specify a color output to the fragment shader

void main()
{
    gl_Position = vec4(Position, 1.0);
    vertexColor = vec4(aColor, 1.0); // set the output variable to a dark-red color
}