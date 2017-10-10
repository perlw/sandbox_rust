#version 330 core

uniform sampler2D tex;

in vec2 texCoord;
out vec4 fragment;

void main() {
    fragment = texture(tex, texCoord);
}
