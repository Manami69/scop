#version 410 core

out vec4 Color;
in vec4 vertexColor; // the input variable from the vertex shader (same name and same type)  
in vec2 TexCoord;
uniform sampler2D ourTexture;
void main()
{
    Color = vec4(1.0, 0.0, 0.0 , 1.0); //texture(ourTexture, TexCoord);
}