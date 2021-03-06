#version 330

#define PI 3.14159
#define product(a, b) vec2(a.x*b.x-a.y*b.y, a.x*b.y+a.y*b.x)
#define conjugate(a) vec2(a.x,-a.y)
//#define divide(a, b) vec2(((a.x*b.x+a.y*b.y)/(b.x*b.x+b.y*b.y)),((a.y*b.x-a.x*b.y)/(b.x*b.x+b.y*b.y)))
#define divide(a, b) vec2(dot(a, b)/dot(b, b),a.y*b.x/dot(b, b) - a.x*b.y/dot(b, b))
#define squareroot(a) sqrt(0.5*vec2(length(a)+a.x, length(a)-a.x) ) * vec2(1.0, sign(a.y))
#define power(a, n) pow(length(a), n)*vec2(cos(atan(a.y, a.x)*n), sin(atan(a.y, a.x )*n))
#define EPSILON 0.000001

in vec3 v_pos;
out vec4 f_color;

uniform vec2 size;
uniform vec2 center;
uniform float t;
uniform float zoom;
uniform int max_iter;
uniform int order;

//https://github.com/hughsk/glsl-hsv2rgb
vec3 hsv2rgb(vec3 c) {
  vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
  vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
  return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

// vec2 function(vec2 p, int o){
//     vec2 poly = -vec2(1.0, 0.0);
//     for(int i = 1; i <= o; ++i){
//         poly += power(p, i);
//     }
//     return poly;
// }

// vec2 derivative(vec2 p, int o){
//     vec2 derivative = vec2(0.0, 0.0);
//     for(int i = 1; i < o; ++i){
//         derivative += product(vec2(i+1,0.0), power(p, i));
//     }
//     return derivative;
// }

vec2 function(vec2 p, int o){
    return power(p, o) - vec2(1., 0.);
}

vec2 derivative(vec2 p, int o){
    return product(vec2(o, 0.0), power(p, o-1));
}

void main()
{
    vec2 p = vec2(v_pos.x*size.x/2., v_pos.y*size.y/2.);
    p = p*zoom + center;
    vec2 r = vec2(cos(t/4.), sin(t/4.));//vec2(0.7, 0.7)+0.5*sin(t);

    float j = max_iter;
    for(int i = 0; i < max_iter; i++){
        vec2 last_p = p;
        p = p - product(r, divide(function(p, order), derivative(p, order)));

        if(distance(p, last_p) < EPSILON){
            j = i;
            break;
        }

    }
    float theta = atan(p.y, p.x);
    float thetaNorm = (PI + theta) / (PI/2.);

    f_color = vec4(hsv2rgb(vec3(thetaNorm, 1.0, 1.0)), 0.5);
}