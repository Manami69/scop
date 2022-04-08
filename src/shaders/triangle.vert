#version 410 core

layout (location = 0) in vec4 model;
layout (location = 1) in vec3 aColor;
// layout (location = 1) in vec2 aTexCoord;
out vec4 vertexColor; // specify a color output to the fragment shader

uniform mat4 transform;
uniform mat4 perspective;
uniform mat4 camera;


void main()
{
    gl_Position = perspective * camera* transform  * model;
    vertexColor = vec4(aColor , 1.0); // set the output variable to a dark-red color
    //TexCoord = aTexCoord;
	//TexDir = Position.xyz;
}
