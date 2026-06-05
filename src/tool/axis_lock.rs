use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AxisLockType {
    Global,
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransformType {
    Translation,
    Rotation,
    Scale,
}

pub fn restrict_transform(
    t: Transform,
    lock_type: AxisLockType,
    transform_type: TransformType,
) -> Transform {
    match (lock_type, transform_type) {
        (AxisLockType::Global, _) => t,
        (AxisLockType::X, TransformType::Translation) => {
            let translation = t.translation * Vec3::new(1.0, 0.0, 0.0);
            Transform {
                translation,
                rotation: t.rotation,
                scale: t.scale,
            }
        }
        (AxisLockType::Y, TransformType::Translation) => {
            let translation = t.translation * Vec3::new(0.0, 1.0, 0.0);
            Transform {
                translation,
                rotation: t.rotation,
                scale: t.scale,
            }
        }
        (AxisLockType::Z, TransformType::Translation) => {
            let translation = t.translation * Vec3::new(0.0, 0.0, 1.0);
            Transform {
                translation,
                rotation: t.rotation,
                scale: t.scale,
            }
        }
        (AxisLockType::X, TransformType::Scale) => {
            let scale = Vec3::new(t.scale.x, 1.0, 1.0);
            Transform {
                translation: t.translation,
                rotation: t.rotation,
                scale,
            }
        }
        (AxisLockType::Y, TransformType::Scale) => {
            let scale = Vec3::new(1.0, t.scale.y, 1.0);
            Transform {
                translation: t.translation,
                rotation: t.rotation,
                scale,
            }
        }
        (AxisLockType::Z, TransformType::Scale) => {
            let scale = Vec3::new(1.0, 1.0, t.scale.z);
            Transform {
                translation: t.translation,
                rotation: t.rotation,
                scale,
            }
        }
        (AxisLockType::X, TransformType::Rotation) => {
            let rotation = t.rotation * Quat::from_vec4(Vec4::new(1.0, 0.0, 0.0, 1.0));
            Transform {
                translation: t.translation,
                rotation,
                scale: t.scale,
            }
        }
        (AxisLockType::Y, TransformType::Rotation) => {
            let rotation = t.rotation * Quat::from_vec4(Vec4::new(0.0, 1.0, 0.0, 1.0));
            Transform {
                translation: t.translation,
                rotation,
                scale: t.scale,
            }
        }
        (AxisLockType::Z, TransformType::Rotation) => {
            let rotation = t.rotation * Quat::from_vec4(Vec4::new(0.0, 0.0, 1.0, 1.0));
            Transform {
                translation: t.translation,
                rotation,
                scale: t.scale,
            }
        }
    }
}
