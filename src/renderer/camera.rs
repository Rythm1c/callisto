use math::{
    mat4::{self, Mat4},
    quaternion::Quat,
    vec3::{Vec3, cross, vec3},
};

#[derive(Clone, Copy, PartialEq)]
pub enum CameraMotion {
    Forwards,
    BackWards,
    Left,
    Right,
    Still,
    Up,
    Down,
}

#[derive(Clone, Copy)]
pub struct CameraController {
    pub sensitivity: f32,
    pub speed: f32,
}

impl CameraController {
    pub fn new() -> Self {
        Self {
            sensitivity: 0.05, // Reduced for better control
            speed: 10.0,
        }
    }

    pub fn rotate(&self, camera: &mut Camera, dx: f32, dy: f32) {
        // FPS-style rotation: update pitch and yaw
        camera.yaw += -dx * self.sensitivity;
        camera.picth += dy * self.sensitivity;

        // Clamp pitch to avoid flipping
        if camera.picth > 89.0 {
            camera.picth = 89.0;
        }
        if camera.picth < -89.0 {
            camera.picth = -89.0;
        }

        // Update orientation quaternion from euler angles
        let yaw_quat = Quat::rotation_y(camera.yaw);
        let pitch_quat = Quat::rotation_x(camera.picth);
        camera.orientation = yaw_quat * pitch_quat;
    }

    pub fn set_camera_motion(camera: &mut Camera, motion: CameraMotion) {
        camera.motion = motion
    }

    pub fn update_motion(&self, camera: &mut Camera, delta: f32) {
        match camera.motion {
            CameraMotion::Still => {}

            CameraMotion::Forwards => {
                self.move_forward(camera, delta);
            }
            CameraMotion::BackWards => {
                self.move_forward(camera, -delta);
            }
            CameraMotion::Left => {
                self.strafe(camera, -delta);
            }
            CameraMotion::Right => {
                self.strafe(camera, delta);
            }
            CameraMotion::Up => {
                self.move_up(camera, delta);
            }
            CameraMotion::Down => {
                self.move_up(camera, -delta);
            }
        }
    }

    pub fn move_forward(&self, camera: &mut Camera, delta: f32) {
        // Calculate forward direction from orientation

        camera.pos = camera.pos + camera.get_target() * self.speed * delta;
    }

    pub fn strafe(&self, camera: &mut Camera, delta: f32) {
        // Calculate right vector perpendicular to forward
        //let yaw_rad = camera.yaw.to_radians();
        let right = cross(&camera.get_target(), &Vec3::Y);

        camera.pos = camera.pos + right * self.speed * delta;
    }

    pub fn move_up(&self, camera: &mut Camera, delta: f32) {
        camera.pos.y += self.speed * delta;
    }
}

#[derive(Clone, Copy)]
pub struct Camera {
    pub pos: Vec3,
    pub orientation: Quat,
    pub picth: f32,
    pub yaw: f32,
    pub aspect_ratio: f32,
    pub motion: CameraMotion,
    //projection
    pub fov: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Self {
        Self {
            pos: vec3(0.0, 3.0, -10.0),
            orientation: Quat::ZERO,
            picth: 0.0,
            yaw: 0.0,
            aspect_ratio,
            motion: CameraMotion::Still,
            fov: 45.0,
            near: 0.1,
            far: 100.0,
        }
    }

    pub fn get_target(&self) -> Vec3 {
        (self.orientation * Vec3::Z).unit()
    }

    pub fn view_matrix(&self) -> Mat4 {
        mat4::look_at(self.pos, self.pos + self.get_target(), Vec3::Y)
    }

    pub fn projection_matrix(&self) -> Mat4 {
        mat4::perspective(self.fov, self.aspect_ratio, self.near, self.far)
    }

    pub fn view_projection_matrix(&self) -> Mat4 {
        mat4::transpose(&(self.projection_matrix() * self.view_matrix()))
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CameraUniform {
    pub view_proj: Mat4,
}
