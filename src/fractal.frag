#version 330

#define PI 3.14159
#define product(a, b) vec2(a.x*b.x-a.y*b.y, a.x*b.y+a.y*b.x)
#define conjugate(a) vec2(a.x,-a.y)
#define divide(a, b) vec2(((a.x*b.x+a.y*b.y)/(b.x*b.x+b.y*b.y)),((a.y*b.x-a.x*b.y)/(b.x*b.x+b.y*b.y)))
#define squareroot(a) sqrt(0.5*vec2(length(a)+a.x, length(a)-a.x) ) * vec2(1.0, sign(a.y))
#define power(a, n) pow(length(a), n)*vec2(cos(atan(a.y, a.x)*n), sin(atan(a.y, a.x )*n))
#define EPSILON 0.000001
#define MAX_ITER 120

in vec3 v_pos;
out vec4 f_color;

uniform vec2 size;

//https://github.com/hughsk/glsl-hsv2rgb
vec3 hsv2rgb(vec3 c) {
  vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
  vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
  return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}


void main()
{
    float x_y = size.x/size.y;
    vec2 p = vec2(v_pos.x, v_pos.y/x_y);
    vec2 r = vec2(1.0, 1.0);

    float j = MAX_ITER;
    for(int i = 0; i < MAX_ITER; i++){
        vec2 last_p = p;
        vec2 poly = power(p, 5.0) - power(p, 4.0) + power(p, 3.0) - power(p, 2.0) + p - vec2(1.0,0.0);
        vec2 derivative = product(vec2(5.0,0.0), power(p, 4.0)) - product(vec2(4.0,0.0), power(p, 3.0)) + product(vec2(3.0,0.0), power(p, 2.0)) - product(vec2(2.0,0.0), p) + 1.;
        p = p - product(r, divide(poly, derivative));

        if(distance(p, last_p) < EPSILON){
            j = i;
            break;
        }

    }
    float theta = atan(p.y, p.x + 0.000000001);
    float thetaNorm = (PI + theta) / (2.*PI);

    f_color = vec4(hsv2rgb(vec3(thetaNorm, 1.0, 1.0) +.1), 1.0 -  j/MAX_ITER);
}