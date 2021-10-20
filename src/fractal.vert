#version 330

in vec2 position;

out vec3 v_pos;

void main()
{
    gl_Position = vec4(position.x, position.y, 0.0, 1.0);
    v_pos = vec3(position.x, position.y, 0.0);
}

