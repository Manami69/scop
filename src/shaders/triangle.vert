#version 410 core

layout (location = 0) in vec4 model;
layout (location = 1) in vec3 aRandColor;
layout (location = 2) in vec3 aColor;
layout (location = 3) in vec2 aTexCoord;
out vec4 vertexColor; // specify a color output to the fragment shader

uniform mat4 transform;
uniform mat4 perspective;
uniform mat4 camera;

out vec4 baseColor;
out vec4 randColor;
out vec2 pos;

void main()
{
    gl_Position = perspective * camera * transform  * model;
	baseColor = vec4(aColor , 1.0);
	randColor = vec4(aRandColor , 1.0);
	pos = model.xy;

    //TexCoord = aTexCoord;
	//TexDir = Position.xyz;
}
