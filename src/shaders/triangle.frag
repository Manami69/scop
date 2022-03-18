#version 410 core

out vec4 Color;
in vec4 vertexColor; // the input variable from the vertex shader (same name and same type)  
in vec3 TexDir;
uniform samplerCube ourTexture;
void main()
{
    Color = texture(ourTexture, normalize(TexDir));//vec4(1.0, 0.0, 0.0 , 1.0); //texture(ourTexture, TexCoord);
}