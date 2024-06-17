#version 460

const int MAX_LIGHTS = 160;

uniform sampler2D textureSampler;
uniform vec2 screenSize;
uniform vec2 lightsPosition[MAX_LIGHTS];
uniform vec4 lightsColor[MAX_LIGHTS];
uniform int lightsAmount;
uniform float lightsRadius[MAX_LIGHTS];
uniform float lightsRotation[MAX_LIGHTS]; // Cone angle for cone lights
uniform int lightsType[MAX_LIGHTS];
uniform float lightsAngle[MAX_LIGHTS];

const int RADIAL_LIGHT = 0;
const int AMBIENT_LIGHT = 1;
const int CONE_LIGHT = 2;

out vec4 fragColor;  // Define the output variable

void main() {
    vec2 uv = gl_FragCoord.xy / screenSize;
    vec3 color_gradient = vec3(0.0);
    vec4 color = texture(textureSampler, uv);
    float falloffFactor;

    for (int i = 0; i < lightsAmount; i++) {
        float lightAlpha = lightsColor[i].a;
        if (lightsType[i] == AMBIENT_LIGHT) {
            color_gradient += lightsColor[i].rgb * lightAlpha;
        }
        else if (lightsType[i] == RADIAL_LIGHT) {
            float curveAmount = 1.5;
            vec2 lightPosition = vec2(lightsPosition[i].x, -lightsPosition[i].y + screenSize.y);
            float worldDistanceToLight = distance(lightPosition, gl_FragCoord.xy);

            // Calculate the distance from the current pixel to the center of the light
            falloffFactor = 1.0 / (-max(0.0, 1.0 - worldDistanceToLight / lightsRadius[i]) / 2.0 + 1.0);
            float cur_gradient = max(0.0, pow((falloffFactor - 1.0), curveAmount));

            // Apply the gradient as a mask to the texture color
            color_gradient += cur_gradient * lightsColor[i].rgb * lightAlpha;
        }
        else if (lightsType[i] == CONE_LIGHT) {
            float curveAmount = 1.5;
            vec2 light_pos = vec2(lightsPosition[i].x, -lightsPosition[i].y + screenSize.y);
            float worldDistanceToLight = distance(light_pos, gl_FragCoord.xy);

            // Calculate the distance from the current pixel to the center of the light
            falloffFactor = 1.0 / (-max(0.0, 1.0 - worldDistanceToLight / lightsRadius[i]) / 2.0 + 1.0);
            float cur_gradient = max(0.0, pow((falloffFactor - 1.0), curveAmount));

            float cone_factor = 1.0;
            float light_angle = lightsRotation[i];
            vec2 light_angle_v = vec2(cos(light_angle), sin(light_angle));

            vec2 light_direction = light_pos - gl_FragCoord.xy;
            float cone = dot(normalize(light_direction), normalize(light_angle_v));
            
            // Smoothly attenuate intensity towards the edges of the cone
            float softness = 0.15; // Adjust this value to control softness of the edge
            float cone_angle = lightsAngle[i];
            float softness_factor = smoothstep(cos(cone_angle/2.0), cos((cone_angle/2.0) - softness), cone);
            
            // Apply softness to cone_factor
            cone_factor *= softness_factor;
            
            if (cone < cos(cone_angle/2.0)){
                cone_factor = 0.0;
            }

            // Apply the gradient as a mask to the texture color
            color_gradient += cur_gradient * lightsColor[i].rgb * lightAlpha * cone_factor;
        }
    }
    // Output the final color with the original alpha
    fragColor = color * vec4(color_gradient, 1.0);
}
