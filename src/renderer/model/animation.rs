/* pub struct Animation {
    clip: Clip,
    name: Option<String>,
}

impl Animation {
    pub fn new(clip: Clip, name: Option<String>) -> Self {
        Self { clip, name }
    }

    pub fn from_gltf(
        animation: &gltf::Animation,
        skin_map: &Option<std::collections::HashMap<usize, usize>>,
    ) -> Self {
        let name = animation.name().map(|n| n.to_string());

        // For simplicity, we assume one skin per animation in this example
        let skin = skin_map
            .as_ref()
            .and_then(|map| map.get(&animation.index()).cloned());

        let mut clip = Clip::new();

        for channel in animation.channels() {
            let sampler = channel.sampler();
            let target = channel.target();

            // Process keyframes and add to clip
            // This is a simplified example; actual implementation would handle interpolation, etc.
            for (input, output) in sampler.inputs().zip(sampler.outputs()) {
                let time = input;
                let value = output;

                clip.add_keyframe(target.node().index(), time, value);
            }
        }

        Self { clip, skin, name }
    }
}

struct AnimationData {
    name: Option<String>,
}

fn animation_data_from_gltf(animation: &gltf::Animation) -> AnimationData {
    let mut name = None;
    if let Some(n) = animation.name() {
        name = Some(String::from(n));
    }
    animation.channels().map(|channel| channel.target().node());

    AnimationData { name }
}
 */