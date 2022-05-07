#version 410 core

out vec4 Color;
in vec4 baseColor;
in vec4 randColor;
in vec4 vertexColor;
in vec2 pos;
uniform int indextext;
uniform float opacity;
uniform sampler2D texture1;

void main()
{
	vec4 text = texture(texture1, pos);
	if (indextext == 0) { 
    	Color = baseColor * (1 - opacity) + randColor * opacity; 
	}
	else if (indextext == 1) {
		Color = randColor * (1 - opacity) + text * opacity; 
	}
	else if (indextext == 2) {
		Color = text * (1 - opacity) + baseColor * opacity;
	}
}
