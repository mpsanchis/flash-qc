use crate::cube::{B, D, F, L, R, U};
use crate::math::Vec3;
use crate::render::Camera;

pub struct FaceHit {
    pub face: usize,
}

pub struct RayPicker;

impl RayPicker {
    /// Pick which face of the cube was clicked
    pub fn pick(
        camera: &Camera,
        mouse_x: f32,
        mouse_y: f32,
        canvas_width: f32,
        canvas_height: f32,
    ) -> Option<FaceHit> {
        // Convert mouse coordinates to normalized device coordinates
        let ndc_x = (2.0 * mouse_x) / canvas_width - 1.0;
        let ndc_y = 1.0 - (2.0 * mouse_y) / canvas_height;

        // Get camera position and ray direction
        let cam_pos = camera.get_position();
        let ray_dir = get_ray_direction(camera, ndc_x, ndc_y);

        // Test intersection with each face plane
        let faces = [
            (U, Vec3::new(0.0, 1.5, 0.0), Vec3::new(0.0, 1.0, 0.0)),
            (D, Vec3::new(0.0, -1.5, 0.0), Vec3::new(0.0, -1.0, 0.0)),
            (F, Vec3::new(0.0, 0.0, 1.5), Vec3::new(0.0, 0.0, 1.0)),
            (B, Vec3::new(0.0, 0.0, -1.5), Vec3::new(0.0, 0.0, -1.0)),
            (L, Vec3::new(-1.5, 0.0, 0.0), Vec3::new(-1.0, 0.0, 0.0)),
            (R, Vec3::new(1.5, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0)),
        ];

        let mut closest: Option<(f32, FaceHit)> = None;

        for (face_idx, plane_point, plane_normal) in faces {
            if let Some((t, _)) =
                ray_plane_intersection(&cam_pos, &ray_dir, &plane_point, &plane_normal)
                    .filter(|(_, p)| is_within_face_bounds(face_idx, p))
                    .filter(|(t, _)| closest.is_none() || *t < closest.as_ref().unwrap().0)
            {
                closest = Some((t, FaceHit { face: face_idx }));
            }
        }

        closest.map(|(_, hit)| hit)
    }
}

fn get_ray_direction(camera: &Camera, ndc_x: f32, ndc_y: f32) -> Vec3 {
    let aspect = camera.aspect;
    let fov_tan = (camera.fov / 2.0).tan();

    // Ray direction in camera space
    let ray_cam = Vec3::new(ndc_x * aspect * fov_tan, ndc_y * fov_tan, -1.0).normalize();

    // Transform to world space using inverse of camera rotation
    let inv_rot = crate::math::Quaternion::new(
        -camera.rotation.x,
        -camera.rotation.y,
        -camera.rotation.z,
        camera.rotation.w,
    );
    let rot_mat = inv_rot.to_rotation_matrix();

    Vec3::new(
        rot_mat.data[0] * ray_cam.x + rot_mat.data[4] * ray_cam.y + rot_mat.data[8] * ray_cam.z,
        rot_mat.data[1] * ray_cam.x + rot_mat.data[5] * ray_cam.y + rot_mat.data[9] * ray_cam.z,
        rot_mat.data[2] * ray_cam.x + rot_mat.data[6] * ray_cam.y + rot_mat.data[10] * ray_cam.z,
    )
}

fn ray_plane_intersection(
    ray_origin: &Vec3,
    ray_dir: &Vec3,
    plane_point: &Vec3,
    plane_normal: &Vec3,
) -> Option<(f32, Vec3)> {
    let denom = plane_normal.dot(ray_dir);
    if denom.abs() < 1e-6 {
        return None; // Ray parallel to plane
    }

    let diff = *plane_point - *ray_origin;
    let t = diff.dot(plane_normal) / denom;

    if t < 0.0 {
        return None; // Intersection behind ray origin
    }

    let hit_point = *ray_origin + *ray_dir * t;
    Some((t, hit_point))
}

fn is_within_face_bounds(face: usize, point: &Vec3) -> bool {
    let bound = 1.5;
    let (a, b) = match face {
        U | D => (point.x, point.z),
        F | B => (point.x, point.y),
        L | R => (point.z, point.y),
        _ => return false,
    };

    a.abs() <= bound && b.abs() <= bound
}
