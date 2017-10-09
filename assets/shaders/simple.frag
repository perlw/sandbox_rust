#version 330 core

in vec2 texCoord;
out vec4 fragment;

void main() {
    fragment = vec4(texCoord.s, texCoord.t, 1.0, 1.0);
}
