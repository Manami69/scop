#version 410 core

out vec4 Color;
in vec4 baseColor;
in vec4 randColor;
in vec4 vertexColor;
in vec2 pos;
in vec2 TexCoord;
in vec3 norm;
in vec3 FragPos;
uniform int indextext;
uniform float opacity;
uniform sampler2D texture1;
uniform sampler2D texture2;
uniform vec3 lighting;

void main()
{
	vec4 text = texture(texture1, pos);
	vec4 textureCoor = texture(texture2, TexCoord);
	vec3 lightDir = normalize(lighting - FragPos);
	float diff = max(dot(norm, lightDir), 0.0);
	vec3 diffuse = diff * vec3(1.,1.,1.);
	vec4 couleur;
	if (indextext == 0) { 
    	couleur = baseColor * (1 - opacity) + randColor * opacity; 
	}
	else if (indextext == 1) {
		couleur = randColor * (1 - opacity) + text * opacity; 
	}
	else if (indextext == 2) {
		couleur = text * (1 - opacity) + textureCoor * opacity;
	}
	else if (indextext == 3) {
		couleur = textureCoor * (1 - opacity) + baseColor * opacity;
	}
	Color = vec4(couleur.rgb * diffuse, couleur.a);
}
