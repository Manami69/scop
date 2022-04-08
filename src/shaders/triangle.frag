#version 410 core

out vec4 Color;

in vec4 vertexColor;
void main()
{
    Color = vertexColor; //texture(ourTexture, TexCoord);
}
