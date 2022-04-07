#version 410 core

out vec4 Color;

in vec4 TexCoord; // coordonnee envoyees par le vertex shader  

void main()
{
    Color = vec4(1.0, 0.0, 0.0 , 1.0); //texture(ourTexture, TexCoord);
}

// #version 410 core

// out vec4 color;

// in vec2 TexCoord; // coordonnee envoyees par le vertex shader

// uniform sampler2D image; // texture parsee et envoyee en tant que variable globale dans le shader

// void main()
// {
//     color = texture(image, TexCoord); // fonction qui recupere les texels des textures
// }