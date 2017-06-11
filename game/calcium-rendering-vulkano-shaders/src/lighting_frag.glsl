#version 450
#extension GL_ARB_separate_shader_objects : enable

const float PI = 3.14159265359;

// TODO: Move the lighting pass to additive lighting geometry passes.
const int AMOUNT_POINTLIGHT = 32;

struct PointLight {
    vec3 position;
    vec3 color;
    float inverse_radius_sqr;
};

layout(set = 0, binding = 0) uniform sampler2D u_gbuffer_position;
layout(set = 0, binding = 1) uniform sampler2D u_gbuffer_base_color;
layout(set = 0, binding = 2) uniform sampler2D u_gbuffer_normal;
layout(set = 0, binding = 3) uniform sampler2D u_gbuffer_roughness;
layout(set = 0, binding = 4) uniform sampler2D u_gbuffer_metallic;
layout(set = 0, binding = 5) uniform LightData {
    vec3 camera_position;

    vec3 ambient_color;

    vec3 directional_color;
    vec3 directional_direction;

    int point_lights_amount;
    PointLight point_lights[AMOUNT_POINTLIGHT];
} u_light_data;

layout(location = 0) in vec2 f_uv;

layout(location = 0) out vec4 o_color;

// Calculates how much the light falls off over distance. Uses the UE4 Inverse
//  Square Falloff method, which is more physically correct than
//  constant-linear-quadratic, and also allows us strict radius control.
// https://github.com/EpicGames/UnrealEngine/blob/release/Engine/Shaders/
//  DeferredLightingCommon.usf#L414
float calculate_attenuation(vec3 light_position, float inverse_raidus_sqr, vec3 position) {
    float distance = length(light_position - position);
    float distance_sqr = (distance * distance);
    float light_radius_attenuation = pow(clamp(
        1.0f - pow(distance_sqr * inverse_raidus_sqr, 2.0f),
        0.0f, 1.0f
    ), 2.0f);

    return light_radius_attenuation;
}

// Calculates how much the surface reflects light versus how much it refracts light
//  headon_reflection = How much the surface reflects if looking directly at the
//                      surface. Metallic surfaces should specify a tinted value
//                      taken from a material database, while non-metallic
//                      surfaces look fine at vec3(0.04).
vec3 fresnel_schlick(float cos_theta, vec3 headon_reflection)
{
    return headon_reflection + (1.0 - headon_reflection) * pow(1.0 - cos_theta, 5.0);
}

// TODO: Document and change variable names for the functions below

float distribution_ggx(vec3 N, vec3 H, float roughness)
{
    float a      = roughness*roughness;
    float a2     = a*a;
    float NdotH  = max(dot(N, H), 0.0);
    float NdotH2 = NdotH*NdotH;

    float nom   = a2;
    float denom = (NdotH2 * (a2 - 1.0) + 1.0);
    denom = PI * denom * denom;

    return nom / denom;
}

float geometry_schlick_ggx(float NdotV, float roughness)
{
    float r = (roughness + 1.0);
    float k = (r*r) / 8.0;

    float nom   = NdotV;
    float denom = NdotV * (1.0 - k) + k;

    return nom / denom;
}

float geometry_smith(vec3 N, vec3 V, vec3 L, float roughness)
{
    float NdotV = max(dot(N, V), 0.0);
    float NdotL = max(dot(N, L), 0.0);
    float ggx2  = geometry_schlick_ggx(NdotV, roughness);
    float ggx1  = geometry_schlick_ggx(NdotL, roughness);

    return ggx1 * ggx2;
}

vec3 calculate_light(
    vec3 light_color, vec3 light_direction, vec3 camera_direction, float attenuation,
    vec3 base_color, vec3 normal, float metallic, float roughness
) {
    // Calculate the "halfway vector", the halfway vector is exactly between the
    //  direction the camera and the light are at, and we can use how much it
    //  aligns with the normal to calculate the amount of specular.
    vec3 halfway_vector = normalize(camera_direction + light_direction);

    // Calculate the "radiance" of this light on this fragment. This means
    //  if the light were traveling in a straight line to this fragment's
    //  position, how strong it would be. This means it doesn't take into
    //  account which side the light is hitting, only base light strength
    //  and distance.
    vec3 radiance = light_color * attenuation;

    // Calculate how much this surface reflects instead of refracting. This
    //  depends on how much of a metal our surface is, and what angle the
    //  camera is at depending on the surface and the light.
    vec3 headon_reflection = mix(vec3(0.04), base_color, metallic);
    float cos_theta = max(dot(halfway_vector, camera_direction), 0.0);
    vec3 reflection_ratio = fresnel_schlick(cos_theta, headon_reflection);

    // TODO: Comment what the heck the NDF and G are
    float NDF = distribution_ggx(normal, halfway_vector, roughness);
    float G   = geometry_smith(normal, camera_direction, light_direction, roughness);

    // Calculate the Cook-Torrance BRDF
    // TODO: Rename various values to better reflect what they are and
    //  comment what exactly this process is better
    // 0.001 is added at the end to prevent a divide by zero crash, for
    //  weird spooky border cases
    vec3 nominator = NDF * G * reflection_ratio;
    float denominator = 4 * max(dot(normal, camera_direction), 0.0) * max(dot(normal, light_direction), 0.0) + 0.001;
    vec3 specular = nominator / denominator;

    // Calculate the light's contribution to the reflectance equation
    // TODO: Needs the same treatment as the code above here does
    vec3 kS = reflection_ratio;
    vec3 kD = vec3(1.0) - kS;
    kD *= 1.0 - metallic;

    // Finally add all the light values together and apply it to the base_color
    // TODO: Needs the same treatment as the code above here does
    float NdotL = max(dot(normal, light_direction), 0.0);
    return (kD * base_color / PI + specular) * radiance * NdotL;
}

void main() {
    // TODO: Allow these values to be specified
    const float ao = 1.0;

    // Retrieve the data for this pixel
    vec3 position = texture(u_gbuffer_position, f_uv).rgb;
    vec4 base_color_full = texture(u_gbuffer_base_color, f_uv);
    vec3 base_color = base_color_full.rgb;
    vec3 normal = texture(u_gbuffer_normal, f_uv).rgb;
    float metallic = texture(u_gbuffer_metallic, f_uv).r;
    float roughness = texture(u_gbuffer_roughness, f_uv).r;

    // Discard this fragment if there isn't actually any data there
    // TODO: Because this is a shader, early bail optimization doesn't work.
    //  The purpose of doing this is to show the background color/cubemap. That
    //  should be moved to emissive color instead.
    if (base_color_full.a == 0.0) {
        discard;
    }

    // Calculate the direction from this fragment to the camera, which is
    //  relevant to various light effects
    vec3 camera_direction = normalize(u_light_data.camera_position - position);

    // Go over all lights and accumulate their light values
    vec3 total_light = vec3(0.0);
    for(int i = 0; i < u_light_data.point_lights_amount; ++i)
    {
        // Get data from the light
        vec3 light_position = u_light_data.point_lights[i].position;
        vec3 light_color = u_light_data.point_lights[i].color;
        float light_inverse_radius_sqr = u_light_data.point_lights[i].inverse_radius_sqr;

        // Calculate the direction the light is at relative to the fragment
        vec3 light_direction = normalize(light_position - position);

        float light_distance = length(light_position - position);
        float attenuation = calculate_attenuation(
            light_position, light_inverse_radius_sqr, position
        );

        // Perform the actual light calculation and add it to the rest
        total_light += calculate_light(
            light_color, light_direction, camera_direction, attenuation,
            base_color, normal, metallic, roughness
        );
    }

    // Improvised directional lighting TODO: Investigate what other engines use
    total_light += calculate_light(
        u_light_data.directional_color, u_light_data.directional_direction,
        camera_direction, 1.0,
        base_color, normal, metallic, roughness
    );

    // Improvised ambient lighting TODO: Investigate what other engines use
    total_light += u_light_data.ambient_color * base_color * ao;

    // Finally, apply the resulting lighting on the base color
    o_color = vec4(total_light, 1.0);
}
