pub const VERTEX_SHADER: &str = r#"
    attribute vec3 a_position;
    attribute vec3 a_normal;
    attribute vec3 a_color;

    uniform mat4 u_model;
    uniform mat4 u_view;
    uniform mat4 u_projection;

    varying vec3 v_color;
    varying vec3 v_normal;

    void main() {
        v_color = a_color;
        v_normal = mat3(u_model) * a_normal;
        gl_Position = u_projection * u_view * u_model * vec4(a_position, 1.0);
    }
"#;

pub const FRAGMENT_SHADER: &str = r#"
    precision mediump float;

    varying vec3 v_color;
    varying vec3 v_normal;

    uniform vec3 u_light_dir;

    void main() {
        vec3 normal = normalize(v_normal);
        vec3 light = normalize(u_light_dir);
        float diffuse = max(dot(normal, light), 0.0);
        vec3 ambient = vec3(0.3);
        vec3 color = v_color * (ambient + diffuse * 0.7);
        gl_FragColor = vec4(color, 1.0);
    }
"#;
