#version 410 core

out vec4 Color;
in vec4 vertexColor; // the input variable from the vertex shader (same name and same type)  
void main()
{
    Color = vertexColor;
}